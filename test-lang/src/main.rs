use std::env;

use compiler::Compiler;
use parser::Parser;

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
mod tests;
pub mod tokens;
mod util;

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();

    let mut name: String;

    let mut lexer = match args.get(1) {
        Some(arg) => {
            name = arg.clone();
            util::get_lexer_for_file(arg)
        }
        None => panic!("Need to specify a file to compile"),
    };

    name.pop();
    name.pop();
    name.pop();

    let new_name: Vec<&str> = name.split("/").collect();

    let name = new_name.last().unwrap();

    let mut parser = Parser::new(&mut lexer);
    let mut compiler = Compiler::new(&mut parser).expect("Failed to compile program since it was empty");
    compiler.compile_program();
    util::compiler_output(&compiler, &format!("tests/output/{}.cir", name))
}
