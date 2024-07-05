mod frontend;
mod tests;

use std::{fs, io, path::PathBuf};

use bumpalo::Bump;
use citadel_api::backend::asm::{AsmBackend, TargetX86_64};
use citadel_api::compile;

use frontend::{lexer::Lexer, parser::Parser};

use crate::frontend::compiler::Compiler;

pub fn compile_asm(input_file_path: PathBuf, out_path: Option<PathBuf>) -> io::Result<()> {
    let input = std::fs::read_to_string(input_file_path)?;
    let lexer = Lexer::new(&input);
    let arena = Bump::new();
    let mut parser = Parser::new(&lexer, &arena);
    let ast = parser.parse_program();
    let ir_stream = Compiler.compile_program(&ast);
    let asm = compile!(AsmBackend::new(TargetX86_64), ir_stream);
    asm.to_file(match out_path {
        Some(path) => path,
        None => PathBuf::from("out.asm"),
    })?;
    Ok(())
}

pub fn compile_chir(input_file_path: PathBuf, out_path: Option<PathBuf>) -> io::Result<()> {
    let input = std::fs::read_to_string(input_file_path)?;
    let lexer = Lexer::new(&input);
    let arena = Bump::new();
    let mut parser = Parser::new(&lexer, &arena);
    let ast = parser.parse_program();
    dbg!(&ast);
    let ir_stream = Compiler.compile_program(&ast);
    let buf = ir_stream.to_string();
    fs::write(
        match out_path {
            Some(path) => path,
            None => PathBuf::from("out.chir"),
        },
        buf,
    )?;
    Ok(())
}
