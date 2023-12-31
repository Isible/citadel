use std::error::Error;

use errors::{InvalidArgError, InterpreterError};
use lexer::Lexer;

mod evaluator;
mod env;
mod obj;

mod parser;

mod tokens;
mod lexer;

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
    lexer.tokenize();

    Ok(())
}