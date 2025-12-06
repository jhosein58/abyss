use abyss_codegen::{ctarget::ctarget::CTarget, director::Director};
use abyss_parser::parser::Parser;

fn main() {
    let input = r#"

        fn test (a: i32, b: &i32): i32 {

        }

       "#;

    println!("Code: \n{}", input);

    let mut parser = Parser::new(input);
    let program = parser.parse_program();

    println!("{}", parser.format_errors("test.ab"));

    let mut target = CTarget::new();
    let mut director = Director::new(&mut target);
    director.process_program(&program);
    println!("{}", director.generate_code());
}
