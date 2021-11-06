use turbo_bear::{codegen::evaluate_program, parser::parse_program};

fn main() {
    let program = parse_program("fn foo(a: int) -> int {}").unwrap();
    evaluate_program(&program);
}
