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
mod tests;
pub mod tokens;
mod util;

fn main() {
    run();
}

fn run() {
    let path = util::file_by_arg(PathBuf::from("tests/compiler-test.tl"));

    let mut lexer = util::get_lexer_for_file(path);
    let mut parser = Parser::new(&mut lexer);
    let ast = parser.parse_program().expect("Failed to parse program");
    let compiler = Compiler::default();

    let codegen = CodeGenerator::new(compiler.compile_program(ast));
    let asm_code = codegen.generate();
    util::asm_output(asm_code, "tests/build/out.asm".into())
}
