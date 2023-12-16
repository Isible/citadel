use clutils::literal::LiteralString;

use crate::util;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Fn,
    If,
    Loop,

    Ident(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Vector(Vec<Token>),

    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    Semicolon,
    Equals,
    Colon,
    Comma,

    Comment(String),

    LParent,
    RParent,
    LCurly,
    RCurly,

    Eof,
}

impl LiteralString for Token {
    fn literal(self: &Self) -> String {
        match self {
            Token::Let => "let".into(),
            Token::Fn => "fn".into(),
            Token::If => "if".into(),
            Token::Loop => "loop".into(),
            Token::Ident(ident) => ident.into(),
            Token::Integer(int) => int.to_string(),
            Token::Float(float) => float.to_string(),
            Token::String(string) => format!("\"{}\"", string),
            Token::Boolean(boolean) => boolean.to_string(),
            Token::Vector(vec) => util::vec_to_vec_string(vec),
            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::Divide => "/".into(),
            Token::Multiply => "*".into(),
            Token::Assign => "=".into(),
            Token::Semicolon => ";".into(),
            Token::Equals => "==".into(),
            Token::Colon => ":".into(),
            Token::Comma => ",".into(),
            Token::Comment(comment) => format!("#{}", comment),
            Token::LParent => "(".into(),
            Token::RParent => ")".into(),
            Token::LCurly => "{".into(),
            Token::RCurly => "}".into(),
            Token::Eof => "Eof".into(),
        }
    }
}
