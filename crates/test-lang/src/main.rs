use std::env;

use codegen::CodeGenerator;
use compiler::Compiler;
use parser::Parser;

pub mod ast;
pub mod codegen;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod tokens;
mod tests;
mod util;

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();

    let name: &str;

    let mut lexer = match args.get(1) {
        Some(arg) => {
            name = arg;
            util::get_lexer_for_file(arg)
        }
        None => panic!("Need to specify a file to compile"),
    };

    let name = name[..name.len() - 3].to_string();

    let name: Vec<&str> = name.split('/').collect();

    let name = name.last().unwrap();

    let mut parser = Parser::new(&mut lexer);
    let mut compiler = Compiler::new(&mut parser).expect("Failed to compile program since it was empty");

    compiler.compile_program();
    util::compiler_output(&compiler, &format!("tests/build/{}.cir", name));

    let mut codegen = CodeGenerator::new(compiler.generator.get_stream());
    let asm_code = codegen.compile();
    dbg!("{}", &asm_code);
}
