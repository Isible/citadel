//! Interpreter for the citadel intermediary representation. Currently only
//! supports the high representation since the LIR is still work in progress.

use std::io;

use evaluator::Evaluator;

use citadel_irparser::{lexer::Lexer, parser::Parser};

mod env;
mod evaluator;
mod obj;

mod errors;
mod util;

fn main() -> io::Result<()> {
    run()
}

fn run() -> io::Result<()> {
    let mut lexer = Lexer::new(util::get_file_by_arg("crates/ciri/tests/main.chir".into()))?;

    let mut parser = Parser::new(&mut lexer);

    let mut evaluator = Evaluator::new(&mut parser);

    evaluator.eval_program();

    Ok(())
}
