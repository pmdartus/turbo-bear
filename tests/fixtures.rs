use std::fs;

use test_generator::test_resources;
use turbo_bear::parser::parse_program;

#[test_resources("tests/fixtures/*.tb")]
fn fixture(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let program = parse_program(&input);
    insta::assert_debug_snapshot!(program);
}