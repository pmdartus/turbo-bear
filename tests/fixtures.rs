use std::fs;

use test_generator::test_resources;
use turbo_bear::parser;

#[test_resources("tests/fixtures/*.tb")]
fn fixture(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let program = parser::parse(&input);
    insta::assert_debug_snapshot!(program);
}
