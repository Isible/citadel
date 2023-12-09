use lexer::Lexer;

pub mod lexer;
pub mod tokens;
mod tests;

fn main() {
    run();
}

fn run() {
    let mut lexer = Lexer::new(String::from("      11000 10002"));
    println!("Token: {:?}", lexer.tokenize());
    println!("Token: {:?}", lexer.tokenize());
}