use inkwell::{builder::Builder, context::Context, module::Module, values::IntValue};

use crate::ast::Expression;

// use inkwell::{OptimizationLevel, context::Context, execution_engine::JitFunction, targets::{Target, InitializationConfig}};

// type SumFunc = unsafe extern "C" fn(u64) -> u64;

// fn main() {
//     let context = Context::create();
//     let builder = context.create_builder();
//     let module = context.create_module("sum");
//     Target::initialize_webassembly(&InitializationConfig::default());
//     let target_machine = target;
//     let exec_egine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

//     let i64_type = context.i64_type();
//     let fn_type = i64_type.fn_type(&[i64_type.into()], false);
//     let function = module.add_function("sum", fn_type, None);
//     let basic_block = context.append_basic_block(function, "entry");

//     builder.position_at_end(basic_block);

//     let x = function.get_nth_param(0).unwrap().into_int_value();

//     builder.build_return(Some(&x));

//     println!("{}", module.print_to_string());

//     // unsafe {
//     //     let res: JitFunction<SumFunc> = exec_egine.get_function("sum").unwrap();
//     //     let output = res.call(123u64);
//     //     print!("{}", output);
//     // };

//     // println!("Hello, world!");
// }

struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("exp");

        CodeGen {
            context,
            builder,
            module
        }
    }

    fn build_expression(&self, expression: &Expression) -> IntValue {
        match &expression.expression {
            crate::ast::Expr::Logical { .. } => todo!(),

            crate::ast::Expr::Binary { operator, left, right } => {
                let lhs = self.build_expression(left);
                let rhs = self.build_expression(right);

                match operator {
                    crate::ast::BinaryOperator::Add => self.builder.build_int_add(lhs, rhs, "add"),
                    crate::ast::BinaryOperator::Subtract => self.builder.build_int_sub(lhs, rhs, "add"),
                    crate::ast::BinaryOperator::Multiply => self.builder.build_int_mul(lhs, rhs, "add"),
                    crate::ast::BinaryOperator::Divide => self.builder.build_int_signed_div(lhs, rhs, "add"),

                    crate::ast::BinaryOperator::Greater => todo!(),
                    crate::ast::BinaryOperator::GreaterEqual => todo!(),
                    crate::ast::BinaryOperator::Less => todo!(),
                    crate::ast::BinaryOperator::LessEqual => todo!(),
                }
            },
            crate::ast::Expr::Unary { operator, expression } => {
                let value = self.build_expression(expression);

                match operator {
                    crate::ast::UnaryOperator::Not => todo!(),
                    crate::ast::UnaryOperator::Minus => self.builder.build_int_neg(value, "negate"),
                }
            },
            crate::ast::Expr::Integer(integer) => {
                self.context.i32_type().const_int(integer.value.into(), false)
            },
            crate::ast::Expr::Float(_) => todo!(),
            crate::ast::Expr::Boolean(_) => todo!(),
        }
    }

    fn build_module(&self, expression: &Expression) {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("expr", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);
        let result = self.build_expression(expression);

        self.builder.build_return(Some(&result));

    }
}


pub fn evaluate_expression(expression: Expression) {
    let context = Context::create();
    let code_gen = CodeGen::new(&context);
    
    code_gen.build_module(&expression);

    println!("{:#?}", code_gen.module.print_to_stderr())
}