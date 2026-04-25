use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use abyss::abyss::Abyss;

fn find_all_main_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_all_main_files(&path, files);
            } else if path.file_name().unwrap() == "main.a" {
                files.push(path);
            }
        }
    }
}

#[test]
fn run_all_e2e_tests() {
    let tests_dir = Path::new("tests");
    let mut test_files = Vec::new();

    find_all_main_files(tests_dir, &mut test_files);

    if test_files.is_empty() {
        println!("No test files found in {:?}", tests_dir);
        return;
    }

    let mut passed = 0;
    let mut failed = 0;

    for main_file in test_files {
        let dir = main_file.parent().unwrap();
        let expected_file = dir.join("expected.txt");

        let source_code = fs::read_to_string(&main_file).expect("Failed to read main.a");

        let captured_output = Arc::new(Mutex::new(String::new()));

        let out_print = Arc::clone(&captured_output);
        let out_printiln = Arc::clone(&captured_output);
        let out_printfln = Arc::clone(&captured_output);
        let out_printbln = Arc::clone(&captured_output);

        let mut abyss = Abyss::new(source_code)
            .with_filename(main_file.to_str().unwrap())
            .with_host_function("print", 1, vec![false], move |args, heap| {
                let mut offset = args[0] as usize;
                while offset + 4 <= heap.len() {
                    let mut val: u32 = 0;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            heap.as_ptr().add(offset),
                            &mut val as *mut u32 as *mut u8,
                            4,
                        );
                    }
                    let char_val = u32::from_le(val);
                    if char_val == 0 {
                        break;
                    }
                    if let Some(c) = std::char::from_u32(char_val) {
                        out_print.lock().unwrap().push(c);
                    }
                    offset += 4;
                }
                0
            })
            .with_host_function("printiln", 1, vec![false], move |args, _heap| {
                let val = args[0] as i32;
                out_printiln.lock().unwrap().push_str(&format!("{}\n", val));
                0
            })
            .with_host_function("printfln", 1, vec![false], move |args, _heap| {
                let val = f64::from_bits(args[0] as u64);
                out_printfln.lock().unwrap().push_str(&format!("{}\n", val));
                0
            })
            .with_host_function("printbln", 1, vec![false], move |args, _heap| {
                let val = args[0] != 0;
                out_printbln.lock().unwrap().push_str(&format!("{}\n", val));
                0
            })
            .disable_tast_print();

        let output = abyss.run_for_test();

        let actual_result = if !output.diagnostics.is_empty() {
            output.diagnostics.trim().to_string()
        } else {
            captured_output.lock().unwrap().trim().to_string()
        };

        if expected_file.exists() {
            let expected_result = fs::read_to_string(&expected_file)
                .expect("Failed to read expected.txt")
                .trim()
                .to_string();

            if actual_result == expected_result {
                passed += 1;
                println!("✅ PASSED: {:?}", dir);
            } else {
                failed += 1;
                println!("❌ FAILED: {:?}", dir);

                let expected_lines: Vec<&str> = expected_result.lines().collect();
                let actual_lines: Vec<&str> = actual_result.lines().collect();
                let max_len = std::cmp::max(expected_lines.len(), actual_lines.len());

                for i in 0..max_len {
                    let exp = expected_lines.get(i).copied().unwrap_or("<Missing Line>");
                    let act = actual_lines.get(i).copied().unwrap_or("<Missing Line>");

                    if exp != act {
                        println!("  -> Mismatch found at line {}:", i + 1);
                        println!("     Expected: {:?}", exp);
                        println!("     Actual:   {:?}", act);
                        break;
                    }
                }
                println!("--------------------------------------------------");
            }
        } else {
            fs::write(&expected_file, actual_result).expect("Failed to write expected.txt");
            println!("📸 CREATED SNAPSHOT: {:?}", dir);
            passed += 1;
        }
    }

    println!("\nTest Summary: {} Passed, {} Failed", passed, failed);

    assert_eq!(
        failed, 0,
        "Some tests failed! See the output above for details."
    );
}
