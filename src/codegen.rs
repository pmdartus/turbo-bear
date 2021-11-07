use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicTypeEnum, IntType},
    values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue},
    FloatPredicate, IntPredicate,
};

use crate::ast::ast::*;

struct CodeGen<'ctx> {
    program: &'ctx Program,
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn new(program: &'ctx Program, context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("exp");

        CodeGen {
            program,
            context,
            builder,
            module,
        }
    }

    fn get_type(&self, ty: &Ty) -> IntType<'ctx> {
        match ty.name.as_ref() {
            "int" | "u32" => self.context.i32_type(),
            _ => todo!("Unknown type {:?}", ty),
        }
    }

    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    fn build_logical(&self, op: &LogicalOp, left: &Expr, right: &Expr) -> AnyValueEnum {
        let lhs = self.build_expr(left).into_int_value();
        let rhs = self.build_expr(right).into_int_value();

        match op {
            LogicalOp::And => self.builder.build_and(lhs, rhs, "and").as_any_value_enum(),
            LogicalOp::Or => self.builder.build_or(lhs, rhs, "or").as_any_value_enum(),
        }
    }

    fn build_binary(&self, op: &BinaryOp, left: &Expr, right: &Expr) -> AnyValueEnum {
        let lhs = self.build_expr(left);
        let rhs = self.build_expr(right);

        match op {
            BinaryOp::Add => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_add(lhs, rhs, "tmpadd")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_add(lhs, rhs, "tmpadd")
                    .as_any_value_enum(),
                _ => panic!("Invalid add operation"),
            },
            BinaryOp::Subtract => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_sub(lhs, rhs, "tmpsub")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_sub(lhs, rhs, "tmpsub")
                    .as_any_value_enum(),
                _ => panic!("Invalid substract operation"),
            },
            BinaryOp::Multiply => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_mul(lhs, rhs, "tmpmul")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_mul(lhs, rhs, "tmpmul")
                    .as_any_value_enum(),
                _ => panic!("Invalid multiply operation"),
            },
            BinaryOp::Divide => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_unsigned_div(lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_div(lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                _ => panic!("Invalid divide operation"),
            },
            BinaryOp::Greater => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_compare(IntPredicate::UGT, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_compare(FloatPredicate::UGT, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                _ => panic!("Invalid compare operation"),
            },
            BinaryOp::GreaterEqual => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_compare(IntPredicate::UGE, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_compare(FloatPredicate::UGE, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                _ => panic!("Invalid compare operation"),
            },
            BinaryOp::Less => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_compare(IntPredicate::ULT, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_compare(FloatPredicate::ULT, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                _ => panic!("Invalid compare operation"),
            },
            BinaryOp::LessEqual => match (lhs, rhs) {
                (AnyValueEnum::IntValue(lhs), AnyValueEnum::IntValue(rhs)) => self
                    .builder
                    .build_int_compare(IntPredicate::ULE, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                (AnyValueEnum::FloatValue(lhs), AnyValueEnum::FloatValue(rhs)) => self
                    .builder
                    .build_float_compare(FloatPredicate::ULE, lhs, rhs, "tmpdiv")
                    .as_any_value_enum(),
                _ => panic!("Invalid compare operation"),
            },
        }
    }

    fn build_unary(&self, op: &UnaryOp, expr: &Expr) -> AnyValueEnum {
        let expr = self.build_expr(expr);

        match op {
            UnaryOp::Not => match expr {
                AnyValueEnum::IntValue(value) => self
                    .builder
                    .build_int_neg(value, "tmpneg")
                    .as_any_value_enum(),
                AnyValueEnum::FloatValue(value) => self
                    .builder
                    .build_float_neg(value, "tmpneg")
                    .as_any_value_enum(),
                _ => panic!("Invalid negate operation"),
            },
            UnaryOp::Minus => match expr {
                AnyValueEnum::IntValue(value) => self
                    .builder
                    .build_int_mul(
                        value,
                        self.context.i32_type().const_int(1, true),
                        "tmpminus",
                    )
                    .as_any_value_enum(),
                AnyValueEnum::FloatValue(value) => self
                    .builder
                    .build_float_mul(value, self.context.f32_type().const_float(-1.0), "tmpminus")
                    .as_any_value_enum(),
                _ => panic!("Invalid minus operation"),
            },
        }
    }

    fn build_call(&self, callee: &Expr, args: &[Box<Expr>]) -> AnyValueEnum {
        let ident = match &callee.kind {
            ExprKind::Ident(ident) => ident,
            _ => panic!("Unexpected callee, only accept identifier"),
        };

        match self.get_function(&ident.name) {
            Some(fn_value) => {
                let args = args
                    .iter()
                    .map(|arg| self.build_expr(arg).try_into().unwrap())
                    .collect::<Vec<BasicValueEnum>>();

                todo!()
            }
            None => panic!("Unknown fn with name {}", ident.name),
        }
    }

    fn build_lit(&self, lit: &Lit) -> AnyValueEnum {
        match lit.kind {
            LitKind::Int(value) => {
                AnyValueEnum::from(self.context.i32_type().const_int(value, false))
            }
            LitKind::Float(value) => AnyValueEnum::from(self.context.f32_type().const_float(value)),
            LitKind::Bool(value) => {
                let int_value = if value { 1 } else { 0 };
                AnyValueEnum::from(self.context.bool_type().const_int(int_value, false))
            }
        }
    }

    fn build_expr(&self, expr: &Expr) -> AnyValueEnum {
        match &expr.kind {
            ExprKind::Logical(op, left, right) => self.build_logical(op, left, right),
            ExprKind::Binary(op, left, right) => self.build_binary(op, left, right),
            ExprKind::Unary(op, expr) => self.build_unary(op, expr),
            ExprKind::Ident(_) => todo!(),
            ExprKind::Call(callee, args) => self.build_call(callee, args),
            ExprKind::Lit(lit) => self.build_lit(lit),
        }
    }

    fn build_fn(
        &self,
        ident: &Ident,
        params: &[(Ident, Ty)],
        return_ty: &Ty,
        block: &Block,
    ) -> FunctionValue {
        let return_type = self.get_type(return_ty);
        let params_type = params
            .iter()
            .map(|(_, ty)| self.get_type(ty).into())
            .collect::<Vec<BasicTypeEnum>>();

        let fn_type = return_type.fn_type(&params_type, false);
        let fn_value = self.module.add_function(&ident.name, fn_type, None);

        self.context.append_basic_block(fn_value, "entry");

        fn_value
    }

    fn build_module(&self) {
        for decl in &self.program.decls {
            match &decl.kind {
                TopLevelDeclKind::Fn(ident, params, return_ty, block) => {
                    let return_type = self.get_type(&return_ty);
                    let params_type = params
                        .iter()
                        .map(|(_, ty)| self.get_type(ty).into())
                        .collect::<Vec<BasicTypeEnum>>();

                    let fn_type = return_type.fn_type(&params_type, false);
                    let fn_value = self.module.add_function(&ident.name, fn_type, None);

                    self.context.append_basic_block(fn_value, "entry");

                    fn_value.verify(true);
                }
                _ => todo!("Unimplemented {:?}", decl),
            }
        }
    }
}

pub fn evaluate_program(program: &Program) {
    let context = Context::create();
    let code_gen = CodeGen::new(program, &context);

    code_gen.build_module();

    println!("{:#?}", code_gen.module.print_to_stderr())
}
