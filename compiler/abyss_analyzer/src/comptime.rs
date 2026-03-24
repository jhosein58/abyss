use abyss_ir::builder::IrBuilder;
use abyss_parser::ast::{Lit, OrderedFloat};
use abyss_types::{
    tast::{SequenceElement, TypedExpr, TypedExprKind},
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
    globals_cache: Vec<(String, TypedExpr)>,
}

impl ComptimeEngine {
    pub fn new() -> Self {
        Self {
            vm: AbyssVm::new_empty(),
            builder: IrBuilder::new(),
            compiler: IrCompiler::new(),
            globals_cache: Vec::new(),
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

    pub fn register_global(&mut self, name: String, mut expr: TypedExpr) {
        self.globals_cache.retain(|(n, _)| n != &name);

        if !matches!(expr.kind, TypedExprKind::FunctionDef { .. }) {
            expr = self.evaluate_expr(expr);

            if let TypedExprKind::Lit(lit) = &expr.kind {
                let val_u64 = match lit {
                    abyss_parser::ast::Lit::Int(i) => *i as u64,
                    abyss_parser::ast::Lit::Float(f) => f.0.to_bits(),
                    abyss_parser::ast::Lit::Bool(b) => {
                        if *b {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };

                //self.compiler.global_constants.insert(name.clone(), val_u64);
            }
        }

        self.globals_cache.push((name.clone(), expr.clone()));

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

    fn reconstruct_value(
        &self,
        raw_val: u64,
        expected_ty: &Type,
        span: abyss_diagnostics::Span,
        id: u32,
    ) -> TypedExpr {
        match expected_ty {
            Type::I32 | Type::Metatype => TypedExpr {
                kind: TypedExprKind::Lit(Lit::Int(raw_val as i64)),
                ty: expected_ty.clone(),
                span,
                id,
            },
            Type::F32 => TypedExpr {
                kind: TypedExprKind::Lit(Lit::Float(OrderedFloat(f64::from_bits(raw_val)))),
                ty: expected_ty.clone(),
                span,
                id,
            },
            Type::Bool => TypedExpr {
                kind: TypedExprKind::Lit(Lit::Bool(raw_val != 0)),
                ty: expected_ty.clone(),
                span,
                id,
            },
            Type::Unit => TypedExpr {
                kind: TypedExprKind::Lit(Lit::Bool(false)),
                ty: expected_ty.clone(),
                span,
                id,
            },

            Type::Array(inner_ty, len) => {
                let ptr = raw_val as usize;
                let mut elements = Vec::new();

                for i in 0..*len {
                    let elem_raw = self.vm.read_heap_u64(ptr, i);
                    let elem_expr = self.reconstruct_value(elem_raw, inner_ty, span.clone(), id);

                    elements.push(SequenceElement {
                        label: None,
                        expr: elem_expr,
                    });
                }

                TypedExpr {
                    kind: TypedExprKind::SequenceInit(elements),
                    ty: expected_ty.clone(),
                    span,
                    id,
                }
            }

            Type::Struct(fields) => {
                let ptr = raw_val as usize;
                let mut elements = Vec::new();

                for (i, field) in fields.iter().enumerate() {
                    let elem_raw = self.vm.read_heap_u64(ptr, i);
                    let elem_expr = self.reconstruct_value(elem_raw, &field.ty, span.clone(), id);

                    elements.push(SequenceElement {
                        label: Some(field.name.clone()),
                        expr: elem_expr,
                    });
                }

                TypedExpr {
                    kind: TypedExprKind::SequenceInit(elements),
                    ty: expected_ty.clone(),
                    span,
                    id,
                }
            }

            _ => panic!("Unsupported comptime return type: {:?}", expected_ty),
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

        self.reconstruct_value(raw_result, &base_ty, span, id)
    }
}
