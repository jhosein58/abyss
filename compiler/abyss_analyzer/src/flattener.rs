use crate::hir::FlatProgram;
use abyss_parser::ast::{
    Expr, ExprKind, FunctionBody, FunctionDef, Lit, Pattern, Program, StaticDef, Stmt, StmtKind,
    StructDef, Type,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SymbolInfo {
    pub mangled_name: String,
    pub is_pub: bool,
    pub kind: SymbolKind,
}

#[derive(Clone, Debug)]
pub enum SymbolKind {
    Struct,
    Function,
    Static,
}

struct Scope {
    renames: HashMap<String, String>,
    imports: HashMap<String, String>,
}

pub struct Flattener {
    path: Vec<String>,
    scopes: Vec<Scope>,
    global_symbols: HashMap<String, SymbolInfo>,
    output: FlatProgram,
}

impl Flattener {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            scopes: vec![Scope {
                renames: HashMap::new(),
                imports: HashMap::new(),
            }],
            global_symbols: HashMap::new(),
            output: FlatProgram::new(),
        }
    }

    pub fn flatten(mut self, program: Program) -> FlatProgram {
        self.collect_definitions(&program, &vec![]);
        self.output.type_aliases = program.type_aliases.clone();
        self.visit_program(program);
        self.output
    }

    fn collect_definitions(&mut self, program: &Program, current_path: &[String]) {
        let prefix = if current_path.is_empty() {
            String::new()
        } else {
            current_path.join("__") + "__"
        };

        for s in &program.structs {
            let mangled = format!("{}{}", prefix, s.name);
            self.global_symbols.insert(
                mangled.clone(),
                SymbolInfo {
                    mangled_name: mangled,
                    is_pub: s.is_pub,
                    kind: SymbolKind::Struct,
                },
            );
        }

        for f in &program.functions {
            let is_extern = matches!(f.body, FunctionBody::Extern);
            let mangled = if is_extern {
                f.name.clone()
            } else {
                format!("{}{}", prefix, f.name)
            };

            self.global_symbols.insert(
                mangled.clone(),
                SymbolInfo {
                    mangled_name: mangled,
                    is_pub: f.is_pub,
                    kind: SymbolKind::Function,
                },
            );
        }

        for (mod_name, sub_prog, _) in &program.modules {
            let mut new_path = current_path.to_vec();
            new_path.push(mod_name.clone());
            self.collect_definitions(sub_prog, &new_path);
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            renames: HashMap::new(),
            imports: HashMap::new(),
        });
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn add_local_rename(&mut self, name: String, mangled: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.renames.insert(name, mangled);
        }
    }

    fn add_import(&mut self, local_name: String, full_path_mangled: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.imports.insert(local_name, full_path_mangled);
        }
    }

    fn resolve_name(&self, name: &str) -> String {
        for scope in self.scopes.iter().rev() {
            if let Some(mangled) = scope.renames.get(name) {
                return mangled.clone();
            }
            if let Some(imported) = scope.imports.get(name) {
                return imported.clone();
            }
        }

        let current_prefix = if self.path.is_empty() {
            String::new()
        } else {
            self.path.join("__") + "__"
        };
        let potential_sibling = format!("{}{}", current_prefix, name);

        if self.global_symbols.contains_key(&potential_sibling) {
            return potential_sibling;
        }

        name.to_string()
    }

    fn check_visibility(&self, target_mangled: &str) {
        if let Some(info) = self.global_symbols.get(target_mangled) {
            if !info.is_pub {
                let current_context = if self.path.is_empty() {
                    String::new()
                } else {
                    self.path.join("__")
                };

                let target_module_path = match target_mangled.rfind("__") {
                    Some(index) => &target_mangled[..index],
                    None => "",
                };

                let has_access = current_context == target_module_path;

                if !has_access {
                    eprintln!(
                        "Error: Symbol '{}' is private and cannot be accessed from module '{}'",
                        target_mangled,
                        if current_context.is_empty() {
                            "Root".to_string()
                        } else {
                            current_context
                        }
                    );
                }
            }
        }
    }

    fn process_top_level_imports(&mut self, uses: Vec<Vec<String>>) {
        for path in uses {
            let mangled_target = path.join("__");
            let local_alias = path.last().unwrap().clone();
            self.check_visibility(&mangled_target);
            self.add_import(local_alias, mangled_target);
        }
    }

    fn visit_program(&mut self, program: Program) {
        let prefix = if self.path.is_empty() {
            String::new()
        } else {
            self.path.join("__") + "__"
        };

        for s in &program.structs {
            let mangled = format!("{}{}", prefix, s.name);
            self.add_local_rename(s.name.clone(), mangled);
        }

        for u in &program.unions {
            let mangled = format!("{}{}", prefix, u.name);
            self.add_local_rename(u.name.clone(), mangled);
        }

        for f in &program.functions {
            let is_extern = matches!(f.body, FunctionBody::Extern);
            let mangled = if is_extern {
                f.name.clone()
            } else {
                format!("{}{}", prefix, f.name)
            };
            self.add_local_rename(f.name.clone(), mangled);
        }
        for s in &program.statics {
            let mangled = format!("{}{}", prefix, s.name);
            self.add_local_rename(s.name.clone(), mangled);
        }

        self.process_top_level_imports(program.uses);
        self.process_modules(program.modules);
        self.process_top_level_structs(program.structs);
        self.process_top_level_statics(program.statics);
        self.process_top_level_functions(program.functions);
    }

    fn process_modules(&mut self, modules: Vec<(String, Program, bool)>) {
        for (mod_name, sub_program, _) in modules {
            self.path.push(mod_name);
            self.enter_scope();
            self.visit_program(sub_program);
            self.exit_scope();
            self.path.pop();
        }
    }

    fn process_top_level_structs(&mut self, structs: Vec<StructDef>) {
        for mut s in structs {
            s.name = self.resolve_name(&s.name);
            for field in &mut s.fields {
                self.rename_in_type(&mut field.1);
            }
            self.output.structs.push(s);
        }
    }

    fn process_top_level_statics(&mut self, statics: Vec<StaticDef>) {
        for mut s in statics {
            s.name = self.resolve_name(&s.name);
            self.rename_in_type(&mut s.ty);
            self.rename_in_expr(&mut s.value);
            self.output.statics.push(s);
        }
    }

    fn process_top_level_functions(&mut self, functions: Vec<FunctionDef>) {
        for mut func in functions {
            let resolved_name = self.resolve_name(&func.name);
            func.name = resolved_name;

            self.enter_scope();

            for (_arg_name, ty) in &mut func.params {
                self.rename_in_type(ty);
            }
            self.rename_in_type(&mut func.return_type);

            if let FunctionBody::UserDefined(ref mut stmts) = func.body {
                for stmt in stmts.iter() {
                    match &stmt.kind {
                        StmtKind::FunctionDef(inner) => {
                            let mangled = format!("{}__{}", func.name, inner.name);
                            self.add_local_rename(inner.name.clone(), mangled);
                        }
                        StmtKind::StructDef(inner) => {
                            let mangled = format!("{}__{}", func.name, inner.name);
                            self.add_local_rename(inner.name.clone(), mangled);
                        }
                        _ => {}
                    }
                }
                self.process_stmts(stmts);

                let (inner_funcs, extracted_structs, cleaned_stmts) =
                    self.isolate_inner_functions(stmts, &func.name);

                *stmts = cleaned_stmts;

                for s in extracted_structs {
                    self.output.structs.push(s);
                }

                for inner_func in inner_funcs {
                    self.visit_function(inner_func);
                }
            }

            self.exit_scope();
            self.output.functions.push(func);
        }
    }

    fn process_stmts(&mut self, stmts: &mut [Stmt]) {
        for stmt in stmts {
            match &mut stmt.kind {
                StmtKind::Use(path) => {
                    let mangled_target = path.join("__");
                    let local_alias = path.last().unwrap().clone();
                    self.check_visibility(&mangled_target);
                    self.add_import(local_alias, mangled_target);
                }
                StmtKind::Let(_name, ty, expr) => {
                    if let Some(t) = ty {
                        self.rename_in_type(t);
                    }
                    if let Some(e) = expr {
                        self.rename_in_expr(e);
                    }
                }
                _ => self.rename_in_stmt(stmt),
            }
        }
    }

    fn visit_function(&mut self, mut func: FunctionDef) {
        let original_name = func.name.clone();

        let resolved = self.resolve_name(&func.name);
        if resolved != func.name {
            func.name = resolved.clone();
        }

        self.enter_scope();

        for (_arg_name, ty) in &mut func.params {
            self.rename_in_type(ty);
        }
        self.rename_in_type(&mut func.return_type);

        let mut extracted_inner_funcs = Vec::new();

        if let FunctionBody::UserDefined(ref mut stmts) = func.body {
            for stmt in stmts.iter() {
                match &stmt.kind {
                    StmtKind::FunctionDef(inner) => {
                        let inner_name = &inner.name;
                        let mangled = format!("{}__{}", func.name, inner_name);
                        self.add_local_rename(inner_name.clone(), mangled);
                    }
                    StmtKind::StructDef(inner) => {
                        let inner_name = &inner.name;
                        let mangled = format!("{}__{}", func.name, inner_name);
                        self.add_local_rename(inner_name.clone(), mangled);
                    }
                    _ => {}
                }
            }
            self.process_stmts(stmts);

            let (inner_funcs, extracted_structs, cleaned_stmts) =
                self.isolate_inner_functions(stmts, &func.name);

            extracted_inner_funcs = inner_funcs;

            for s in extracted_structs {
                self.output.structs.push(s);
            }

            *stmts = cleaned_stmts;
        }

        self.exit_scope();
        self.output.functions.push(func);

        self.path.push(original_name);
        for inner_func in extracted_inner_funcs {
            self.visit_function(inner_func);
        }
        self.path.pop();
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
            let span = stmt.span;

            match stmt.kind {
                StmtKind::FunctionDef(inner_func_box) => {
                    let mut inner_func = *inner_func_box;
                    let old_name = inner_func.name.clone();
                    let new_name = format!("{}__{}", parent_mangled_name, old_name);
                    inner_func.name = new_name;
                    inner_funcs.push(inner_func);
                }
                StmtKind::StructDef(inner_struct_box) => {
                    let mut inner_struct = *inner_struct_box;
                    let old_name = inner_struct.name.clone();
                    let new_name = format!("{}__{}", parent_mangled_name, old_name);
                    inner_struct.name = new_name;
                    inner_structs.push(inner_struct);
                }
                other_kind => cleaned_stmts.push(Stmt {
                    kind: other_kind,
                    span,
                }),
            }
        }

        (inner_funcs, inner_structs, cleaned_stmts)
    }

    fn rename_in_stmt(&self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::Let(_, Some(ty), Some(expr)) | StmtKind::Const(_, Some(ty), Some(expr)) => {
                self.rename_in_type(ty);
                self.rename_in_expr(expr);
            }
            StmtKind::Let(_, Some(ty), None) | StmtKind::Const(_, Some(ty), None) => {
                self.rename_in_type(ty);
            }
            StmtKind::Ret(expr) | StmtKind::Expr(expr) => {
                self.rename_in_expr(expr);
            }
            StmtKind::Assign(lhs, rhs) => {
                self.rename_in_expr(lhs);
                self.rename_in_expr(rhs);
            }
            StmtKind::If(cond, then_stmt, else_stmt) => {
                self.rename_in_expr(cond);
                self.rename_in_stmt(then_stmt);
                if let Some(else_s) = else_stmt {
                    self.rename_in_stmt(else_s);
                }
            }
            StmtKind::While(cond, body) => {
                self.rename_in_expr(cond);
                self.rename_in_stmt(body);
            }
            StmtKind::Block(stmts) => {
                for s in stmts {
                    self.rename_in_stmt(s);
                }
            }
            _ => {}
        }
    }

    fn rename_in_expr(&self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Ident(path) => {
                if path.len() == 1 {
                    let resolved = self.resolve_name(&path[0]);
                    if resolved.contains("__") {
                        self.check_visibility(&resolved);
                    }
                    path[0] = resolved;
                } else {
                    let full_mangled = path.join("__");
                    self.check_visibility(&full_mangled);
                    *path = vec![full_mangled];
                }
            }
            ExprKind::Call(callee, args, generics) => {
                self.rename_in_expr(callee);
                for arg in args {
                    self.rename_in_expr(arg);
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            ExprKind::StructInit(path, fields, generics) => {
                if path.len() == 1 {
                    let resolved = self.resolve_name(&path[0]);
                    path[0] = resolved;
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
            ExprKind::UnionInit(path, variants) => {
                if path.len() == 1 {
                    let resolved = self.resolve_name(&path[0]);
                    path[0] = resolved;
                } else if path.len() > 1 {
                    let new_name = path.join("__");
                    *path = vec![new_name];
                }

                for (_, val_expr) in variants {
                    self.rename_in_expr(val_expr);
                }
            }
            ExprKind::Binary(left, _, right) => {
                self.rename_in_expr(left);
                self.rename_in_expr(right);
            }
            ExprKind::Unary(_, operand) => {
                self.rename_in_expr(operand);
            }
            ExprKind::Index(arr, idx) => {
                self.rename_in_expr(arr);
                self.rename_in_expr(idx);
            }
            ExprKind::Deref(inner) | ExprKind::AddrOf(inner) => {
                self.rename_in_expr(inner);
            }
            ExprKind::Member(obj, _) => {
                self.rename_in_expr(obj);
            }
            ExprKind::MethodCall(obj, _, args, generics) => {
                self.rename_in_expr(obj);
                for arg in args {
                    self.rename_in_expr(arg);
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            ExprKind::Cast(inner, ty) => {
                self.rename_in_expr(inner);
                self.rename_in_type(ty);
            }
            ExprKind::Is(inner, ty) => {
                self.rename_in_expr(inner);
                self.rename_in_type(ty);
            }
            ExprKind::SizeOf(ty) => {
                self.rename_in_type(ty);
            }
            ExprKind::Match(expr, arms) => {
                self.rename_in_expr(expr);
                for (pattern, arm_expr) in arms {
                    if let Pattern::Variant(path, _) = pattern {
                        if path.len() == 1 {
                            let resolved = self.resolve_name(&path[0]);
                            if resolved.contains("__") {
                                self.check_visibility(&resolved);
                            }
                            path[0] = resolved;
                        } else {
                            let full_mangled = path.join("__");
                            self.check_visibility(&full_mangled);
                            *path = vec![full_mangled];
                        }
                    }
                    self.rename_in_expr(arm_expr);
                }
            }
            ExprKind::Ternary(cond, t_expr, f_expr) => {
                self.rename_in_expr(cond);
                self.rename_in_expr(t_expr);
                self.rename_in_expr(f_expr);
            }
            ExprKind::Lit(lit) => {
                if let Lit::Array(exprs) = lit {
                    for e in exprs {
                        self.rename_in_expr(e);
                    }
                }
            }
        }
    }

    fn rename_in_type(&self, ty: &mut Type) {
        match ty {
            Type::Struct(path, generics) => {
                if path.len() == 1 {
                    let resolved = self.resolve_name(&path[0]);
                    path[0] = resolved;
                } else if path.len() > 1 {
                    let new_name = path.join("__");
                    *path = vec![new_name];
                }
                for g in generics {
                    self.rename_in_type(g);
                }
            }
            Type::Pointer(inner) => self.rename_in_type(inner),
            Type::Const(inner) => self.rename_in_type(inner),
            Type::Array(inner, _) => self.rename_in_type(inner),
            Type::Function(args, ret, _generics) => {
                for arg in args {
                    self.rename_in_type(arg);
                }
                self.rename_in_type(ret);
            }
            Type::Union(types) => {
                for t in types {
                    self.rename_in_type(t);
                }
            }
            _ => {}
        }
    }
}
