use turbo_bear::parser::parse_program;

fn main() {
    let program = parse_program("let a = 1 + 2 * 4;").unwrap();
    println!("{:#?}", program);
    // evaluate_expression(program);
}
