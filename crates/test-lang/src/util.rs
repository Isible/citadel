use std::{
    env::args,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use citadel_backend::experimental::asm::elements::AsmElement;
use citadel_frontend::ir::IRStmt;

use crate::{ast::Statement, compiler::Compiler, lexer::Lexer, tokens::Token};

pub fn get_lexer_for_file(file_path: PathBuf) -> Lexer {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read to string");
    Lexer::new(buf)
}

pub fn compiler_output(stream: Vec<IRStmt>, location: PathBuf) {
    let buf = stream.iter().map(|ir| ir.to_string()).collect::<Vec<String>>().join("\n");
    let mut file = File::create(&location).unwrap_or_else(|err| {
        panic!(
            "Failed to create a new file at {}, error: {err}",
            location.display()
        )
    });
    file.write_all(buf.as_bytes())
        .expect("Failed to write to file");
}

pub fn asm_output(stream: Vec<AsmElement>, location: PathBuf) {
    let buf = stream
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    let mut file = File::create(&location).unwrap_or_else(|err| {
        panic!(
            "Failed to create a new file at {}, error: {err}",
            location.display()
        )
    });
    file.write_all(buf.as_bytes())
        .expect("Failed to write to file");
}

pub fn get_next_tok(lexer: &mut Lexer) -> Token {
    loop {
        let tok = lexer.tokenize();
        if let Some(tok) = tok {
            return tok;
        }
    }
}

pub fn file_by_arg(default: PathBuf) -> PathBuf {
    let args = args().collect::<Vec<String>>();
    if let Some(arg) = args.get(1) {
        PathBuf::from(arg)
    } else {
        default
    }
}
