use abyss_parser::ast::TypeAlias;

use crate::{
    hir::FlatProgram,
    new_type_checker::{context::TypeContext, passes::type_checker::TypeCheckPass},
};

pub mod context;
pub mod passes;
pub mod visitor;

pub trait Pass {
    fn name(&self) -> &str;
    fn run(&mut self, ctx: &mut TypeContext);
}

pub struct PassManager {
    ctx: TypeContext,
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self {
            ctx: TypeContext::new(),
            passes: Vec::new(),
        }
    }

    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    pub fn run(mut self, program: FlatProgram) -> (FlatProgram, TypeContext) {
        self.load_program_into_context(program);

        let passes = std::mem::take(&mut self.passes);
        for mut pass in passes {
            println!("Running Pass: {}", pass.name());
            pass.run(&mut self.ctx);
        }

        let new_program = self.export_context_to_program();

        (new_program, self.ctx)
    }

    fn load_program_into_context(&mut self, program: FlatProgram) {
        for func in program.functions {
            let is_generic = !func.generics.is_empty();

            let result = if is_generic {
                self.ctx.register_generic_func(func.clone())
            } else {
                self.ctx.register_concrete_func(func.clone())
            };

            if let Err(e) = result {
                panic!("{e}");
            }
        }

        for def in program.structs {
            let is_generic = !def.generics.is_empty();

            let result = if is_generic {
                self.ctx.register_generic_struct(def.clone())
            } else {
                self.ctx.register_concrete_struct(def.clone())
            };

            if let Err(e) = result {
                panic!("{e}");
            }
        }

        for s in program.statics {
            let _ = self.ctx.register_static(s);
        }

        for ty in program.type_aliases {
            let _ = self.ctx.register_type_alias(ty);
        }
    }

    fn export_context_to_program(&mut self) -> FlatProgram {
        let mut program = FlatProgram::default();

        for (_, func) in self.ctx.concrete_funcs.drain() {
            program.functions.push(func);
        }

        for (_, st) in self.ctx.concrete_structs.drain() {
            program.structs.push(st);
        }

        for (_, st) in self.ctx.statics.drain() {
            program.statics.push(st);
        }

        for (n, ty) in self.ctx.type_aliases.drain() {
            program.type_aliases.push(TypeAlias {
                is_pub: true,
                name: n,
                ty,
            });
        }

        program
    }
}

pub struct DefaultTypeChecker {
    manager: PassManager,
}

impl DefaultTypeChecker {
    pub fn new() -> Self {
        let mut manager = PassManager::new();

        manager.add_pass(Box::new(TypeCheckPass::new()));

        Self { manager }
    }

    pub fn check(self, program: FlatProgram) -> (FlatProgram, TypeContext) {
        self.manager.run(program)
    }
}
