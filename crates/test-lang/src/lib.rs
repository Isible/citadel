mod tests;
mod frontend;

use std::{fs, io, path::PathBuf};

use citadel_api::backend::experimental::asm::util;
use citadel_api::compile;
use citadel_api::backend::experimental::asm::{AsmBackend, TargetX86_64};

use frontend::{ast::Statement, lexer::Lexer, parser::Parser};

use crate::frontend::compiler::Compiler;

pub fn compile_asm(input_file_path: PathBuf, out_path: Option<PathBuf>) -> io::Result<()> {
    let ast = gen_ast(input_file_path)?;
    let ir_stream = Compiler.compile_program(&ast);
    let asm = compile!(AsmBackend::new(TargetX86_64), ir_stream);
    let buf = util::format(asm.stream);
    fs::write(match out_path {
        Some(path) => path,
        None => PathBuf::from("out.asm"),
    }, buf)?;
    Ok(())
}

pub fn compile_chir(input_file_path: PathBuf, out_path: Option<PathBuf>) -> io::Result<()> {
    let ast = gen_ast(input_file_path)?;
    dbg!(&ast);
    let ir_stream = Compiler.compile_program(&ast);
    let buf = ir_stream.to_string();
    fs::write(match out_path {
        Some(path) => path,
        None => PathBuf::from("out.chir"),
    }, buf)?;
    Ok(())
}

fn gen_ast(input_file_path: PathBuf) -> io::Result<Vec<Statement>> {
    let input = std::fs::read_to_string(input_file_path)?;
    let mut lexer = Lexer::new(&input);
    let mut parser = Parser::new(&mut lexer);
    let ast = parser.parse_program();
    Ok(ast)
}
