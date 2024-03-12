//! # Citadel - test-lang
//! 
//! The test-lang crate of the citadel project
//! 
//! For information on what exactly citadel is you should visit our [github-repository](https://github.com/Isible/citadel/blob/main/README.md)
//! 
//! This crate provides a simple example for implemnting a compiler using the citadel toolchain

use std::path::PathBuf;

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
    let path = util::get_file_by_arg(PathBuf::from("tests/compiler-test.tl"));

    let name = path.to_string_lossy().to_string();
    let name = name[..name.len() - 3].to_string();

    let name: Vec<&str> = name.split('/').collect();

    let name = name.last().unwrap();

    let mut lexer = util::get_lexer_for_file(path);
    let mut parser = Parser::new(&mut lexer);
    let mut compiler = Compiler::new(&mut parser).expect("Failed to compile program since it was empty");

    compiler.compile_program();
    util::compiler_output(&compiler, format!("tests/build/{}.cir", name).into());

    let codegen = CodeGenerator::new(compiler.generator.get_stream());
    let asm_code = codegen.compile();
    util::asm_output(asm_code, "tests/build/out.asm".into())
}
