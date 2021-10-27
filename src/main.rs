#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate lazy_static;

mod ast;
mod grammar;
mod codegen;

use ast::parse_expression;
use codegen::evaluate_expression;

fn main() {
    let expression = parse_expression("1 + 2 * 4");
    evaluate_expression(expression);
}
