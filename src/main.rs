use std::{fs, path::PathBuf, process};

use clap::Parser;
use turbo_bear::{codegen, parser};

#[derive(Parser, Debug)]
#[clap(
    name = "turbo-bear",
    author = "Pierre-Marie Dartus <pm@dartus.fr>",
    about = "Compiler for the turbo bear language",
    version = env!("CARGO_PKG_VERSION")
)]
struct Opts {
    /// The file to compile
    #[clap(parse(from_os_str))]
    input: PathBuf,

    /// Print the parsed AST
    #[clap(long)]
    parse: bool,

    /// Print the generated LLVM IR
    #[clap(long)]
    llvm_ir: bool,
}

fn main() {
    let opts = Opts::parse();

    let source = match fs::read_to_string(&opts.input) {
        Ok(src) => src,
        Err(_) => {
            eprintln!("Failed to read file {}", opts.input.to_string_lossy());
            process::exit(1);
        }
    };

    let program = match parser::parse(&source) {
        Ok(program) => program,
        Err(errors) => {
            eprintln!("{:#?}", errors);
            process::exit(1);
        }
    };

    if opts.parse {
        println!("{:#?}", program);
        return;
    }

    // evaluate_program(&program);
}
