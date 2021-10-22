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
