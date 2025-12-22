use crate::hir::FlatProgram;
use abyss_parser::ast::{
    Expr, FunctionBody, FunctionDef, Program, StaticDef, Stmt, StructDef, Type,
};
use std::collections::HashMap;

pub struct Flattener {
    path: Vec<String>,
    current_scope_renames: HashMap<String, String>,
    output: FlatProgram,
}

impl Flattener {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            current_scope_renames: HashMap::new(),
            output: FlatProgram::new(),
        }
    }

    pub fn flatten(mut self, program: Program) -> FlatProgram {
        self.visit_program(program);
        self.output
    }

    fn visit_program(&mut self, program: Program) {
        self.process_modules(program.modules);

        let mut local_renames = HashMap::new();

        for s in &program.structs {
            let mangled = self.generate_mangled_name(&s.name);
            local_renames.insert(s.name.clone(), mangled);
        }

        for f in &program.functions {
            let mangled = self.generate_mangled_name(&f.name);
            local_renames.insert(f.name.clone(), mangled);
        }

        let old_renames = self.current_scope_renames.clone();
        self.current_scope_renames.extend(local_renames);

        self.process_top_level_structs(program.structs);
        self.process_top_level_functions(program.functions);
        self.process_top_level_statics(program.statics);

        self.current_scope_renames = old_renames;
    }

    fn process_modules(&mut self, modules: Vec<(String, Program, bool)>) {
        for (mod_name, sub_program, _) in modules {
            self.path.push(mod_name);
            self.visit_program(sub_program);
            self.path.pop();
        }
    }

    fn process_top_level_structs(&mut self, structs: Vec<StructDef>) {
        for mut s in structs {
            if let Some(new_name) = self.current_scope_renames.get(&s.name) {
                s.name = new_name.clone();
            }
            for field in &mut s.fields {
                self.rename_in_type(&mut field.1);
            }
            self.output.structs.push(s);
        }
    }

    fn process_top_level_statics(&mut self, statics: Vec<StaticDef>) {
        for mut s in statics {
            s.name = self.generate_mangled_name(&s.name);
            self.rename_in_type(&mut s.ty);
            self.output.statics.push(s);
        }
    }

    fn process_top_level_functions(&mut self, functions: Vec<FunctionDef>) {
        for func in functions {
            self.visit_function(func);
        }
    }

    fn visit_function(&mut self, mut func: FunctionDef) {
        let original_name = func.name.clone();

        if let Some(new_name) = self.current_scope_renames.get(&func.name) {
            func.name = new_name.clone();
        } else {
            func.name = self.generate_mangled_name(&func.name);
        }

        for arg in &mut func.params {
            self.rename_in_type(&mut arg.1);
        }
        self.rename_in_type(&mut func.return_type);

        let mut extracted_inner_funcs = Vec::new();

        if let FunctionBody::UserDefined(ref mut stmts) = func.body {
            self.apply_renames(stmts);

            let (inner_funcs, extracted_structs, cleaned_stmts) =
                self.isolate_inner_functions(stmts, &func.name);

            extracted_inner_funcs = inner_funcs;

            for s in extracted_structs {
                self.output.structs.push(s);
            }

            *stmts = cleaned_stmts;
        }

        self.output.functions.push(func);

        self.path.push(original_name);
        for inner_func in extracted_inner_funcs {
            self.visit_function(inner_func);
        }
        self.path.pop();
    }

    fn generate_mangled_name(&self, name: &str) -> String {
        if self.path.is_empty() {
            name.to_string()
        } else {
            format!("{}__{}", self.path.join("__"), name)
        }
    }

    fn isolate_inner_functions(
        &self,
        stmts: &mut Vec<Stmt>,
        parent_mangled_name: &str,
    ) -> (Vec<FunctionDef>, Vec<StructDef>, Vec<Stmt>) {
        let mut inner_funcs = Vec::new();
        let mut cleaned_stmts = Vec::new();
        let mut inner_structs = Vec::new();

        for stmt in stmts.drain(..) {
            match stmt {
                Stmt::FunctionDef(inner_func_box) => {
                    let mut inner_func = *inner_func_box;
                    let old_name = inner_func.name.clone();
                    let new_name = format!("{}__{}", parent_mangled_name, old_name);
                    inner_func.name = new_name;

                    inner_funcs.push(inner_func);
                }
                Stmt::StructDef(inner_struct_box) => {
                    let mut inner_struct = *inner_struct_box;
                    let old_name = inner_struct.name.clone();
                    let new_name = format!("{}__{}", parent_mangled_name, old_name);
                    inner_struct.name = new_name;
                    inner_structs.push(inner_struct);
                }
                _ => cleaned_stmts.push(stmt),
            }
        }

        (inner_funcs, inner_structs, cleaned_stmts)
    }

    fn apply_renames(&self, stmts: &mut [Stmt]) {
        for stmt in stmts {
            self.rename_in_stmt(stmt);
        }
    }

    fn rename_in_stmt(&self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Let(_, Some(ty), Some(expr)) | Stmt::Const(_, Some(ty), Some(expr)) => {
                self.rename_in_type(ty);
                self.rename_in_expr(expr);
            }
            Stmt::Let(_, Some(ty), None) | Stmt::Const(_, Some(ty), None) => {
                self.rename_in_type(ty);
            }
            Stmt::Ret(expr) | Stmt::Expr(expr) => {
                self.rename_in_expr(expr);
            }
            Stmt::Assign(lhs, rhs) => {
                self.rename_in_expr(lhs);
                self.rename_in_expr(rhs);
            }
            Stmt::If(cond, then_stmt, else_stmt) => {
                self.rename_in_expr(cond);
                self.rename_in_stmt(then_stmt);
                if let Some(else_s) = else_stmt {
                    self.rename_in_stmt(else_s);
                }
            }
            Stmt::While(cond, body) => {
                self.rename_in_expr(cond);
                self.rename_in_stmt(body);
            }
            Stmt::Block(stmts) => {
                self.apply_renames(stmts);
            }
            _ => {}
        }
    }

    fn rename_in_expr(&self, expr: &mut Expr) {
        match expr {
            Expr::Ident(path) => {
                if path.len() == 1 {
                    if let Some(new_name) = self.current_scope_renames.get(&path[0]) {
                        path[0] = new_name.clone();
                    }
                } else if path.len() > 1 {
                    let new_name = path.join("__");
                    *path = vec![new_name];
                }
            }
            Expr::StructInit(path, fields, generics) => {
                if path.len() == 1 {
                    if let Some(new_name) = self.current_scope_renames.get(&path[0]) {
                        path[0] = new_name.clone();
                    }
                } else if path.len() > 1 {
                    let new_name = path.join("__");
                    *path = vec![new_name];
                }

                for (_, val_expr) in fields {
                    self.rename_in_expr(val_expr);
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            Expr::Call(callee, args, generics) => {
                self.rename_in_expr(callee);
                for arg in args {
                    self.rename_in_expr(arg);
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            Expr::Binary(left, _, right) => {
                self.rename_in_expr(left);
                self.rename_in_expr(right);
            }
            Expr::Unary(_, operand) => {
                self.rename_in_expr(operand);
            }
            Expr::Index(arr, idx) => {
                self.rename_in_expr(arr);
                self.rename_in_expr(idx);
            }
            Expr::Deref(inner) | Expr::AddrOf(inner) => {
                self.rename_in_expr(inner);
            }
            Expr::Member(obj, _) => {
                self.rename_in_expr(obj);
            }
            Expr::MethodCall(obj, _, args, generics) => {
                self.rename_in_expr(obj);
                for arg in args {
                    self.rename_in_expr(arg);
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            Expr::Cast(inner, ty) => {
                self.rename_in_expr(inner);
                self.rename_in_type(ty);
            }
            Expr::SizeOf(ty) => {
                self.rename_in_type(ty);
            }
            _ => {}
        }
    }

    fn rename_in_type(&self, ty: &mut Type) {
        match ty {
            Type::Struct(path, generics) => {
                if path.len() == 1 {
                    if let Some(new_name) = self.current_scope_renames.get(&path[0]) {
                        path[0] = new_name.clone();
                    }
                } else if path.len() > 1 {
                    let new_name = path.join("__");
                    *path = vec![new_name];
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            Type::Pointer(inner) => self.rename_in_type(inner),
            Type::Array(inner, _) => self.rename_in_type(inner),
            Type::Function(args, ret, _generics) => {
                for arg in args {
                    self.rename_in_type(arg);
                }
                self.rename_in_type(ret);
            }
            _ => {}
        }
    }
}
