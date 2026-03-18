use std::fs;
use std::path::{Path, PathBuf};

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

        let abyss = Abyss::new(source_code)
            .with_filename(main_file.to_str().unwrap())
            .disable_tast_print();

        let output = abyss.run_for_test();

        let actual_result = if !output.diagnostics.is_empty() {
            output.diagnostics.trim().to_string()
        } else {
            output.stdout.trim().to_string()
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
                println!("--- EXPECTED ---\n{}\n----------------", expected_result);
                println!("--- ACTUAL ---\n{}\n---------------\n", actual_result);
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
