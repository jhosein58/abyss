use std::{
    collections::HashMap,
    fs,
    io::Read,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicI64, Ordering},
    },
    time::Instant,
};

use abyss_codegen::{
    director::Director,
    jit::CraneliftTarget,
    target::{Target, Type},
};
use abyss_parser::parser::Parser;

pub unsafe extern "C" fn print_i(n: i64) {
    println!("{}", n);
}

pub unsafe extern "C" fn print_f(n: f64) {
    println!("{}", n);
}

static mut SEED: u64 = 0x123456789ABCDEF;

#[inline]
fn rand_u64() -> u64 {
    unsafe {
        let mut x = SEED;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        SEED = x;
        x
    }
}

#[inline]
pub fn rand() -> f64 {
    let x = rand_u64();
    (x as f64) / (u64::MAX as f64)
}

static NEXT_ARRAY_ID: AtomicI64 = AtomicI64::new(1);

fn array_heap() -> &'static Mutex<HashMap<i64, Vec<f64>>> {
    static HEAP: OnceLock<Mutex<HashMap<i64, Vec<f64>>>> = OnceLock::new();
    HEAP.get_or_init(|| Mutex::new(HashMap::new()))
}

pub extern "C" fn arr_alloc(size: i64) -> i64 {
    if size < 0 {
        eprintln!("[Runtime Error] Array size cannot be negative: {}", size);
        return 0;
    }

    let id = NEXT_ARRAY_ID.fetch_add(1, Ordering::Relaxed);
    let new_vec = vec![0.0; size as usize];

    let heap = array_heap();
    if let Ok(mut map) = heap.lock() {
        map.insert(id, new_vec);
    }

    id
}

pub extern "C" fn arr_set(id: i64, index: i64, value: f64) {
    let heap = array_heap();
    if let Ok(mut map) = heap.lock() {
        if let Some(vec) = map.get_mut(&id) {
            if index >= 0 && index < vec.len() as i64 {
                vec[index as usize] = value;
            } else {
                eprintln!(
                    "[Runtime Error] Array index out of bounds: ID={}, Index={}, Len={}",
                    id,
                    index,
                    vec.len()
                );
            }
        } else {
            eprintln!(
                "[Runtime Error] Accessing freed or invalid array ID: {}",
                id
            );
        }
    }
}

pub extern "C" fn arr_get(id: i64, index: i64) -> f64 {
    let heap = array_heap();
    if let Ok(map) = heap.lock() {
        if let Some(vec) = map.get(&id) {
            if index >= 0 && index < vec.len() as i64 {
                return vec[index as usize];
            } else {
                eprintln!(
                    "[Runtime Error] Array index out of bounds: ID={}, Index={}, Len={}",
                    id,
                    index,
                    vec.len()
                );
            }
        } else {
            eprintln!(
                "[Runtime Error] Accessing freed or invalid array ID: {}",
                id
            );
        }
    }
    0.0
}

pub extern "C" fn arr_free(id: i64) {
    let heap = array_heap();
    if let Ok(mut map) = heap.lock() {
        if map.remove(&id).is_none() {
            eprintln!("[Runtime Warning] Double free or invalid ID: {}", id);
        }
    }
}

pub extern "C" fn arr_len(id: i64) -> i64 {
    let heap = array_heap();
    if let Ok(map) = heap.lock() {
        if let Some(vec) = map.get(&id) {
            return vec.len() as i64;
        }
    }
    0
}

fn main() {
    let mut input = String::new();
    fs::File::open("main.a")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    // println!("Code:\n{}", input);

    println!("Compiling...");

    let t = Instant::now();

    let symbols = [
        ("print_i", print_i as *const u8),
        ("print_f", print_f as *const u8),
        ("rand", rand as *const u8),
        ("arr", arr_alloc as *const u8),
        ("arr_set", arr_set as *const u8),
        ("arr_get", arr_get as *const u8),
        ("arr_len", arr_len as *const u8),
        ("arr_free", arr_free as *const u8),
    ];

    let mut target = CraneliftTarget::new(&symbols);
    target.declare_extern_function("print_i", &[("n".to_string(), Type::Int)], Type::Void);
    target.declare_extern_function("print_f", &[("n".to_string(), Type::Float)], Type::Void);
    target.declare_extern_function("rand", &[], Type::Float);
    target.declare_extern_function("arr", &[("s".to_string(), Type::Int)], Type::Int);
    target.declare_extern_function(
        "arr_set",
        &[
            ("a".to_string(), Type::Int),
            ("i".to_string(), Type::Int),
            ("v".to_string(), Type::Float),
        ],
        Type::Void,
    );
    target.declare_extern_function(
        "arr_get",
        &[("a".to_string(), Type::Int), ("i".to_string(), Type::Int)],
        Type::Float,
    );
    target.declare_extern_function("arr_free", &[("a".to_string(), Type::Int)], Type::Void);

    target.declare_extern_function("arr_len", &[("a".to_string(), Type::Int)], Type::Int);
    let mut director = Director::new(&mut target);

    let mut parser = Parser::new(&input);
    let program = parser.parse_program();
    println!("{}", parser.format_errors("test.a"));
    //dbg!(&program);
    director.process_program(&program);

    println!("Finished in: {}ms", t.elapsed().as_millis());
    println!("Running...\n");

    let _ = target.run_fn("main");

    //println!("\n{}\n", res.unwrap());
}
