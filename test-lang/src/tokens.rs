#[derive(Debug)]
pub enum Token {
    Let,
    Func,
    If,
    While,

    Ident(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),

    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    Semicolon,

    Eof,
}