#[repr(C)]
pub struct TCCState;

use std::ffi::{CString, c_char, c_int, c_void};

pub const TCC_OUTPUT_MEMORY: i32 = 1;
pub const TCC_RELOCATE_AUTO: *mut c_void = 1 as *mut c_void;

unsafe extern "C" {
    pub fn tcc_new() -> *mut TCCState;
    pub fn tcc_delete(s: *mut TCCState);
    pub fn tcc_compile_string(s: *mut TCCState, code: *const c_char) -> c_int;
    pub fn tcc_set_output_type(s: *mut TCCState, ty: c_int) -> c_int;

    pub fn tcc_relocate(s: *mut TCCState, ptr: *mut c_void) -> c_int;

    pub fn tcc_get_symbol(s: *mut TCCState, name: *const c_char) -> *mut c_void;

    pub fn tcc_add_include_path(s: *mut TCCState, pathname: *const c_char) -> c_int;
    pub fn tcc_set_lib_path(s: *mut TCCState, pathname: *const c_char) -> c_int;
    pub fn tcc_add_symbol(s: *mut TCCState, name: *const c_char, func_ptr: *const c_void) -> c_int;
}

use std::fs;
use std::path::Path;

pub struct AbyssJit {
    state: *mut TCCState,
}

impl AbyssJit {
    pub fn new() -> Self {
        unsafe {
            let state = tcc_new();
            if state.is_null() {
                panic!("Could not initialize TCC state");
            }

            let tcc_dir = Path::new("third_party/tcc");

            let abs_path = fs::canonicalize(&tcc_dir)
                  .expect("CRITICAL: Could not find 'third_party/tcc' directory. Make sure it exists relative to where you run cargo.");

            let path_str = abs_path
                .to_str()
                .expect("Path contains invalid UTF-8 characters");
            let c_path = CString::new(path_str).unwrap();

            tcc_set_lib_path(state, c_path.as_ptr());

            let include_path = abs_path.join("include");
            if include_path.exists() {
                let c_inc = CString::new(include_path.to_str().unwrap()).unwrap();
                tcc_add_include_path(state, c_inc.as_ptr());
            }

            tcc_set_output_type(state, TCC_OUTPUT_MEMORY);

            AbyssJit { state }
        }
    }

    pub fn compile(&mut self, c_code: &str) -> Result<(), String> {
        let c_str = CString::new(c_code).unwrap();
        unsafe {
            let ret = tcc_compile_string(self.state, c_str.as_ptr());
            if ret == -1 {
                return Err("Compilation failed".to_string());
            }
        }
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), String> {
        unsafe {
            let ret = tcc_relocate(self.state, TCC_RELOCATE_AUTO);
            if ret < 0 {
                return Err("Relocation failed".to_string());
            }
        }
        Ok(())
    }

    pub fn get_function<T>(&self, func_name: &str) -> Option<T> {
        let c_name = CString::new(func_name).unwrap();
        let sym = unsafe { tcc_get_symbol(self.state, c_name.as_ptr()) };

        if sym.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute_copy(&sym) })
        }
    }

    pub fn add_function(&self, name: &str, func_ptr: *const c_void) {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            tcc_add_symbol(self.state, c_name.as_ptr(), func_ptr);
        }
    }
}

impl Drop for AbyssJit {
    fn drop(&mut self) {
        unsafe {
            tcc_delete(self.state);
        }
    }
}
