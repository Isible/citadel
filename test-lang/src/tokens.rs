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

    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    Semicolon,
    Equals,

    LParent,
    RParent,
    LCurly,
    RCurly,

    Eof,
}
