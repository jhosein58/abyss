pub mod io;
pub mod mem;

use crate::{
    interface::{AbyssLibrary, NativeFunctionDef},
    stdlib::{
        io::{
            abyss_print, abyss_printb, abyss_printbln, abyss_printf, abyss_printfln, abyss_printi,
            abyss_printiln, abyss_println,
        },
        mem::{abyss_alloc, abyss_free},
    },
};

pub struct StandardLib;

impl AbyssLibrary for StandardLib {
    fn get_functions() -> &'static [NativeFunctionDef] {
        static FUNCTIONS: &[NativeFunctionDef] = &[
            // io
            NativeFunctionDef {
                name: "print",
                arity: 1,
                func: abyss_print,
            },
            NativeFunctionDef {
                name: "println",
                arity: 1,
                func: abyss_println,
            },
            NativeFunctionDef {
                name: "printi",
                arity: 1,
                func: abyss_printi,
            },
            NativeFunctionDef {
                name: "printiln",
                arity: 1,
                func: abyss_printiln,
            },
            NativeFunctionDef {
                name: "printf",
                arity: 1,
                func: abyss_printf,
            },
            NativeFunctionDef {
                name: "printfln",
                arity: 1,
                func: abyss_printfln,
            },
            NativeFunctionDef {
                name: "printb",
                arity: 1,
                func: abyss_printb,
            },
            NativeFunctionDef {
                name: "printbln",
                arity: 1,
                func: abyss_printbln,
            },
            // mem
            NativeFunctionDef {
                name: "alloc",
                arity: 1,
                func: abyss_alloc,
            },
            NativeFunctionDef {
                name: "free",
                arity: 1,
                func: abyss_free,
            },
        ];
        FUNCTIONS
    }
}
