use crate::new_type_checker::{
    Pass,
    context::TypeContext,
    passes::type_checker::{
        monomorphization::MonomorphizationPass, resolution::ResolutionPass, safety::SafetyPass,
    },
};

pub mod monomorphization;
pub mod resolution;
pub mod safety;
pub mod utils;

pub struct TypeCheckPass {
    resolution: ResolutionPass,
    safety: SafetyPass,
    monomorphization: MonomorphizationPass,
}

impl TypeCheckPass {
    pub fn new() -> Self {
        TypeCheckPass {
            resolution: ResolutionPass::new(),
            safety: SafetyPass::new(),
            monomorphization: MonomorphizationPass::new(),
        }
    }
}

impl Pass for TypeCheckPass {
    fn name(&self) -> &str {
        "TypeCheckPass"
    }

    fn run(&mut self, ctx: &mut TypeContext) {
        self.resolution.run(ctx);
        self.safety.run(ctx);
        self.monomorphization.run(ctx);
    }
}
