#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate lazy_static;

mod ast;
mod codegen;
mod grammar;

use ast::{parse_expression, parse_program};
use codegen::evaluate_expression;

fn main() {
    let program = parse_program("let a = 1 + 2 * 4;").unwrap();
    println!("{:#?}", program);
    // evaluate_expression(program);
}
