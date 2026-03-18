use std::collections::HashMap;

use abyss_ir::builder::IrBuilder;
use abyss_parser::ast::{Lit, OrderedFloat};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use abyss_vm::{
    codegen::IrCompiler,
    vm::{core::AbyssVm, types::NativeFunction},
};

pub struct ComptimeEngine {
    vm: AbyssVm,
    pub builder: IrBuilder,
    compiler: IrCompiler,
    globals_cache: HashMap<String, TypedExpr>,
}

impl ComptimeEngine {
    pub fn new() -> Self {
        Self {
            vm: AbyssVm::new_empty(),
            builder: IrBuilder::new(),
            compiler: IrCompiler::new(),
            globals_cache: HashMap::new(),
        }
    }

    pub fn register_native_with_index(
        &mut self,
        name: &str,
        index: usize,
        arity: u8,
        func: NativeFunction,
    ) {
        self.vm.register_native(arity, func);

        self.builder.register_native(name, index);
    }
    pub fn register_global(&mut self, name: String, expr: TypedExpr) {
        self.globals_cache.insert(name.clone(), expr.clone());

        if matches!(expr.kind, TypedExprKind::FunctionDef { .. }) {
            if let Some(ir_prog) = self.builder.build_single_function(expr) {
                let old_inst_len = self.compiler.instructions.len();
                let old_const_len = self.compiler.constants.len();

                self.compiler.compile_chunk(&ir_prog, false);

                self.vm
                    .inject_instructions(&self.compiler.instructions[old_inst_len..]);
                self.vm
                    .inject_constants(&self.compiler.constants[old_const_len..]);
            }
        }
    }

    pub fn evaluate_expr(&mut self, expr: TypedExpr) -> TypedExpr {
        if matches!(expr.kind, TypedExprKind::Lit(_)) {
            return expr;
        }

        let expected_ty = expr.ty.clone();
        let span = expr.span.clone();
        let id = expr.id;

        let compiler_inst_count = self.compiler.instructions.len();
        let compiler_const_count = self.compiler.constants.len();

        let ir_prog = self.builder.build_thunk_program(expr, &self.globals_cache);

        let thunk_start_ip = self.compiler.compile_chunk(&ir_prog, true);

        let vm_saved_ip = self
            .vm
            .inject_instructions(&self.compiler.instructions[compiler_inst_count..]);
        let vm_saved_const = self
            .vm
            .inject_constants(&self.compiler.constants[compiler_const_count..]);

        let raw_result = self.vm.run_thunk(thunk_start_ip).unwrap_or(0);

        self.vm.rewind_state(vm_saved_ip, vm_saved_const);
        self.compiler
            .rewind(compiler_inst_count, compiler_const_count);

        let base_ty = expected_ty.underlying_type();

        let ast_lit = match base_ty {
            Type::I32 | Type::Metatype => Lit::Int(raw_result as i64),
            Type::F32 => Lit::Float(OrderedFloat(f64::from_bits(raw_result))),
            Type::Bool => Lit::Bool(raw_result != 0),
            Type::Unit => Lit::Bool(false),

            _ => panic!("Unsupported comptime return type: {:?}", expected_ty),
        };

        TypedExpr {
            kind: TypedExprKind::Lit(ast_lit),
            ty: expected_ty,
            span,
            id,
        }
    }
}
