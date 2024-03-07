//! Interpreter for the citadel intermediary representation. Currently only
//! supports the high representation since the LIR is still work in progress.

use std::{error::Error, fs::File, io::Write};

use errors::InterpreterError;
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

use crate::tokens::Token;

mod env;
mod evaluator;
mod obj;

mod parser;

mod lexer;
mod tokens;

mod errors;
mod util;

fn main() -> Result<(), impl Error> {
    run()
}

fn run() -> Result<(), InterpreterError> {
    let mut lexer = Lexer::new(&"tests/main.cir".into()).unwrap_or_else(|err| panic!("{err}"));

    let mut parser = Parser::new(&mut lexer);

    let mut evaluator = Evaluator::new(&mut parser);

    evaluator.eval_program();

    Ok(())
}

fn generate_tokens(lexer: &mut Lexer) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.tokenize();
        if tok == Token::Eof {
            tokens.push(tok);
            break tokens;
        }
        tokens.push(tok);
        lexer.next_char();
    }
}

fn write_to_file(tokens: Vec<Token>) {
    let mut out_file = File::create("tests/out.cir").expect("Failed to create file");
    let mut buf = String::new();
    for token in tokens {
        buf.push_str(&token.to_string());
        buf.push(' ');
    }
    out_file
        .write_all(buf.as_bytes())
        .expect("Failed to write content to file");
}
