use abyss_ir::facade::Ir;
use abyss_ir::ir::{IrStmt, IrType};
use abyss_vm::codegen::IrCompiler;
use abyss_vm::vm::core::AbyssVm;
use std::rc::Rc;

pub struct Runtime {
    compiler: IrCompiler,
    vm_hooks: Vec<Box<dyn Fn(&mut AbyssVm)>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            compiler: IrCompiler::new(),
            vm_hooks: Vec::new(),
        }
    }

    pub fn register_builtin(
        &mut self,
        name: &str,
        params: Vec<IrType>,
        ret: IrType,
        is_ref: Vec<bool>,
        func: Rc<dyn Fn(&[u64], &mut [u8]) -> u64 + 'static>,
    ) {
        self.compiler.register_extern(name, params, ret);

        let name_owned = name.to_string();
        let arity = is_ref.len() as u8;

        self.vm_hooks.push(Box::new(move |vm| {
            vm.register_host_function(&name_owned, arity as usize, is_ref.clone(), func.clone());
        }));
    }

    pub fn execute(self, stmts: Vec<IrStmt>) {
        let ir_program = Ir::program(stmts);

        let (instructions, constants, extern_defs) = self.compiler.compile(&ir_program);
        let mut vm = AbyssVm::new(instructions, constants);

        for hook in &self.vm_hooks {
            hook(&mut vm);
        }

        vm.load_imports(&extern_defs);
        vm.init_globals(ir_program.globals.len());

        vm.run();
    }
}
