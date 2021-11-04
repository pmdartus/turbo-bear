use std::fs;

use crate::parser::parse_program;

#[test]
fn parse_fixture_tests() {
    insta::glob!("fixtures/*.tb", |path| {
        let input = fs::read_to_string(path).unwrap();
        let program = parse_program(&input);
        insta::assert_debug_snapshot!(program);
    });
}