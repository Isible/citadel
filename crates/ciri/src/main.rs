use std::{error::Error, fs::File, io::Write};

use errors::{InterpreterError, InvalidArgError};
use lexer::Lexer;

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
    let args = std::env::args().collect::<Vec<String>>();
    // get arg at pos 1 since arg 0 is the directory
    let first_arg = match args.get(1) {
        Some(arg) => arg,
        None => return Err(InterpreterError(Box::from(InvalidArgError(1)))),
    };

    let mut lexer = match Lexer::new(first_arg) {
        Ok(lexer) => lexer,
        Err(err) => return Err(InterpreterError(Box::from(err))),
    };

    let tokens = generate_tokens(&mut lexer);

    write_to_file(tokens);

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
