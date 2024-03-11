//! Interpreter for the citadel intermediary representation. Currently only
//! supports the high representation since the LIR is still work in progress.

use errors::InterpreterError;
use evaluator::Evaluator;

use citadel_irparser::{lexer::Lexer, parser::Parser};

mod env;
mod evaluator;
mod obj;

mod errors;

fn main() -> Result<(), InterpreterError> {
    run()
}

fn run() -> Result<(), InterpreterError> {
    let mut lexer = Lexer::new(&"tests/main.chir".into()).unwrap_or_else(|err| panic!("{err}"));

    let mut parser = Parser::new(&mut lexer);

    let mut evaluator = Evaluator::new(&mut parser);

    evaluator.eval_program();

    Ok(())
}