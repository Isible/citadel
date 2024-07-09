//! List of possible tokens in the language

use logos::Logos;

#[derive(Debug, PartialEq, Clone, Logos)]
#[logos(skip r#"(?:\/\/[^\n]*|\t|\s|\f|\n)*"#)]
pub enum Token<'tok> {
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("loop")]
    Loop,
    #[token("return")]
    Return,
    #[token("use")]
    Use,
    #[token("type")]
    Type,

    #[regex(r#""(?:\\.|[^\\"])*""#)]
    LitString(&'tok str),
    #[regex(r"-?[0-9]+")]
    LitInt(&'tok str),
    #[regex("-?[0-9]+\\.[0-9]+")]
    LitFloat(&'tok str),
    #[regex(r#"'[^\\']'"#)]
    LitChar(&'tok str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'tok str),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Divide,
    #[token("*")]
    Multiply,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token("==")]
    Equals,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,

    #[token("(")]
    LParent,
    #[token(")")]
    RParent,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
}
