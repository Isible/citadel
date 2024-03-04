use std::{fs::File, io::{Read, Write}};

use crate::{lexer::Lexer, tokens::Token, compiler::Compiler};

pub fn get_lexer_for_file(file_path: &str) -> Lexer {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read to string");
    Lexer::new(buf)
}

pub fn compiler_output(compiler: &Compiler, location: &str) {
    let buf = compiler.generator.as_string();
    let mut file = File::create(&location).unwrap_or_else(|_| panic!("Failed to create a new file at {location}"));
    file.write_all(buf.as_bytes()).expect("Failed to write to file");
}

pub fn get_next_tok(lexer: &mut Lexer) -> Token {
    loop {
        let tok = lexer.tokenize();
        if let Some(tok) = tok {
            return tok;
        }
    }
}