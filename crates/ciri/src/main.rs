//! Interpreter for the citadel intermediary representation. Currently only
//! supports the high representation since the LIR is still work in progress.

//use std::{fs, io};

//use clap::Parser as _;
//use cli::Args;
//use evaluator::Evaluator;

//use citadel_irparser::{lexer::Lexer, parser::Parser};

//mod env;
//mod evaluator;
//mod obj;

//mod errors;
//mod util;
//mod cli;

fn main() {
}
/*
fn run() -> io::Result<()> {
    let args = Args::parse();
    let file_content = fs::read(&args.file)?;
    let mut lexer = Lexer::new(std::str::from_utf8(&file_content).unwrap());

    let mut parser = Parser::new(&mut lexer);

    let mut evaluator = Evaluator::new(&mut parser);

    Ok(())
}
*/