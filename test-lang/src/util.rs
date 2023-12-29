use std::{fs::File, io::Read};

use crate::{lexer::Lexer, tokens::Token};

pub fn get_lexer_for_file(file_path: &str) -> Lexer {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read to string");
    Lexer::new(buf)
}

pub fn vec_to_string_list(starting_brace: char, vec: &Vec<Token>) -> String {
    let mut list_string = String::new();
    list_string.push(starting_brace);
    vec.iter().for_each(|tok| {
        list_string.push_str(tok.to_string().as_str());
        list_string.push_str(", ");
    });
    list_string.pop();
    list_string
}

pub fn vec_to_arr_string(vec: &Vec<Token>) -> String {
    let mut arr_string = vec_to_string_list('[', vec);
    arr_string.push(']');
    arr_string
}

pub fn vec_to_vec_string(vec: &Vec<Token>) -> String {
    let mut arr_string = vec_to_string_list('<', vec);
    arr_string.push('>');
    arr_string
}