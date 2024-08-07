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
    let parser_arena = Bump::new();
    let mut parser = Parser::new(&lexer, &parser_arena);
    let ast = parser.parse_program();
    let compiler_arena = Bump::new();
    let ir_stream = Compiler::compile_program(ast, parser.functions(), &compiler_arena);
    compile!(AsmBackend::new(TargetX86_64), ir_stream)
        .to_file(out_path.unwrap_or(PathBuf::from("build/asm/out.asm")))
}

pub fn compile_chir(input_file_path: PathBuf, out_path: Option<PathBuf>) -> io::Result<()> {
    let input = std::fs::read_to_string(input_file_path)?;
    let lexer = Lexer::new(&input);
    let arena = Bump::new();
    let mut parser = Parser::new(&lexer, &arena);
    let ast = parser.parse_program();
    let compiler_arena = Bump::new();
    let ir_stream = Compiler::compile_program(ast, parser.functions(), &compiler_arena);
    fs::write(
        out_path.unwrap_or(PathBuf::from("build/chir/out.chir")),
        ir_stream.to_string(),
    )
}
