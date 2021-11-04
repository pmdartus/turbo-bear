use inkwell::{builder::Builder, context::Context, module::Module, values::IntValue};

use crate::ast::ast::*;

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

    fn build_expression(&self, expression: &Expr) -> IntValue {
        match &expression.kind {

            ExprKind::Binary (op, left, right ) => {
                let lhs = self.build_expression(left);
                let rhs = self.build_expression(right);

                match op {
                    BinaryOp::Add => self.builder.build_int_add(lhs, rhs, "add"),
                    BinaryOp::Subtract => self.builder.build_int_sub(lhs, rhs, "add"),
                    BinaryOp::Multiply => self.builder.build_int_mul(lhs, rhs, "add"),
                    BinaryOp::Divide => self.builder.build_int_signed_div(lhs, rhs, "add"),

                    BinaryOp::Greater => todo!(),
                    BinaryOp::GreaterEqual => todo!(),
                    BinaryOp::Less => todo!(),
                    BinaryOp::LessEqual => todo!(),
                }
            },
            ExprKind::Unary(op, expr) => {
                let value = self.build_expression(expression);

                match op {
                    UnaryOp::Not => todo!(),
                    UnaryOp::Minus => self.builder.build_int_neg(value, "negate"),
                }
            },
            ExprKind::Lit(_) => todo!(),
            ExprKind::Logical(_, _, _) => todo!(),
            ExprKind::Ident(_) => todo!(),
            
        }
    }

    fn build_module(&self, expression: &Expr) {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("expr", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);
        let result = self.build_expression(expression);

        self.builder.build_return(Some(&result));

    }
}


pub fn evaluate_expression(expression: Expr) {
    let context = Context::create();
    let code_gen = CodeGen::new(&context);
    
    code_gen.build_module(&expression);

    println!("{:#?}", code_gen.module.print_to_stderr())
}