use std::{fs::File, io::Read};

use crate::lexer::Lexer;

pub fn get_lexer_for_file(file_path: &str) -> Lexer {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read to string");
    Lexer::new(buf)
}