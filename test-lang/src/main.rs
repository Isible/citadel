use std::{fs::File, io::Read};

use lexer::Lexer;
use tokens::Token;

pub mod lexer;
mod tests;
pub mod tokens;

fn main() {
    run();
}

fn run() {
    let mut file = File::open("tests/test.tl").expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read to string");
    let mut lexer = Lexer::new(buf);
    loop {
        let tok = lexer.tokenize();
        println!("Token: {:?}", tok);
        if tok == Token::Eof {
            break;
        }
    }
}
