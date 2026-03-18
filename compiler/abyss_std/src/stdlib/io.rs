use std::io::{self, Write};

use abyss_vm::vm::core::AbyssVm;

pub fn abyss_print(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let ptr = args[0];

    let string_value = vm.read_c_string(ptr);

    print!("{}", string_value);
    let _ = io::stdout().flush();
    vm.out.push_str(&string_value);

    0
}

pub fn abyss_printi(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let v = format!("{}", args[0] as i64);

    print!("{}", v);
    let _ = io::stdout().flush();
    vm.out.push_str(&v);
    0
}
pub fn abyss_printiln(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let v = format!("{}\n", args[0] as i64);

    print!("{}", v);
    let _ = io::stdout().flush();
    vm.out.push_str(&v);
    0
}
pub fn abyss_printf(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let float_val = f64::from_bits(args[0]);

    let output = format!("{}", float_val);

    print!("{}", output);
    let _ = io::stdout().flush();
    vm.out.push_str(&output);

    0
}

pub fn abyss_printfln(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let float_val = f64::from_bits(args[0]);

    let output = format!("{}\n", float_val);

    print!("{}", output);
    let _ = io::stdout().flush();
    vm.out.push_str(&output);

    0
}

pub fn abyss_printb(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let bool_val = args[0];

    let output = if bool_val == 0 {
        format!("false")
    } else {
        format!("true")
    };

    print!("{}", output);
    let _ = io::stdout().flush();
    vm.out.push_str(&output);

    0
}

pub fn abyss_printbln(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let bool_val = args[0];

    let output = if bool_val == 0 {
        format!("false\n")
    } else {
        format!("true\n")
    };

    print!("{}", output);
    let _ = io::stdout().flush();
    vm.out.push_str(&output);

    0
}
