use lexer::Lexer;

pub mod lexer;
pub mod tokens;

fn main() {
    run();
}

fn run() {
    let mut lexer = Lexer::new(String::from("1000 10002")).expect("Lexer errored");
    println!("Token: {:?}", lexer.tokenize().expect("Failed to tokenize"));
    println!("Token: {:?}", lexer.tokenize().expect("Failed to tokenize"))
}
