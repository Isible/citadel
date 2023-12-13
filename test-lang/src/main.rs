use std::env;

pub mod tokens;
pub mod lexer;
pub mod ast;
pub mod parser;
mod tests;
mod util;

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();

    let mut _lexer = match args.get(1) {
        Some(arg) => util::get_lexer_for_file(arg),
        None => todo!(),
    };
}
