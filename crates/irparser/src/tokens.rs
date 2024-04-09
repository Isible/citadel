//! Simple representation of symvols and idents of the IR

use std::fmt::Display;

use crate::util;

#[derive(Debug, PartialEq)]
pub enum Token {
    // --special-characters--
    /// $ - define a constant
    DollarSign,
    /// @ - define a function
    At,
    /// % - reference a variable or function
    PercentSign,
    /// ? - define a variable
    QuestionMark,
    /// = - assign a value to a varable/constant
    Assign,
    /// ' - the marker of a label
    Apostrophe,
    /// : - colon is used to mark a variety of things and labels in particular
    Colon,
    /// , - comma is used to list arguments
    Comma,
    /// . - dot is used for namespaces and methods
    Dot,

    /// Brackets
    /// ( - left parenthesis
    LParent,
    /// ) - right parenthesis
    RParent,
    /// [ - left square brackets
    LSquare,
    /// ] - right square brackets
    RSquare,
    /// { - left curly brackets
    LCurly,
    /// } - right curly brackets
    RCurly,

    // --keywords--
    /// sets the access of a variable to local
    Priv,
    /// sets the access of a variable to public
    Pub,
    /// marks a function as declared only meaning it gets initialized in a different module
    Decl,
    /// Call a function
    Call,
    /// Return a value
    Ret,
    /// Arithmetic Operations
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Break a loop
    Break,
    /// Jump to a label
    Jump,

    // --others--
    /// A raw literal that is not enclosed in a literal `l{...}` holder
    RawLit(Literal),
    /// A literal value. Can represent a string, integer or float
    Lit(Literal),
    /// An identifier like a function or variable name
    Ident(String),
    /// Eof - marks the end of a file
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Char(char),
    Array(Vec<Token>),
    Vector(Vec<Token>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::String(str) => format!("\"{}\"", str),
                Literal::Char(ch) => format!("'{}'", ch),
                Literal::Integer(int) => int.to_string(),
                Literal::Float(float) => float.to_string(),
                Literal::Boolean(bool) => bool.to_string(),
                Literal::Array(arr) => util::vec_to_arr_string(arr),
                Literal::Vector(vec) => util::vec_to_vec_string(vec),
            }
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Token::DollarSign => String::from("$"),
                Token::At => String::from("@"),
                Token::PercentSign => String::from("%"),
                Token::QuestionMark => String::from("?"),
                Token::Assign => String::from("="),
                Token::Colon => String::from(":"),
                Token::Comma => String::from(","),
                Token::Dot => String::from("."),
                Token::Apostrophe => String::from("'"),
                Token::Priv => String::from("priv"),
                Token::Pub => String::from("pub"),
                Token::Decl => String::from("decl"),
                Token::Lit(val) => format!("l{{{val}}}"),
                Token::RawLit(lit) => lit.to_string(),
                Token::Ident(val) => val.to_owned(),
                Token::Eof => String::from("EOF"),
                Token::Call => String::from("call"),
                Token::Ret => String::from("ret"),
                Token::Add => String::from("add"),
                Token::Sub => String::from("sub"),
                Token::Mul => String::from("mul"),
                Token::Div => String::from("div"),
                Token::Jump => String::from("jmp"),
                Token::Break => String::from("br"),
                Token::LParent => String::from("("),
                Token::RParent => String::from(")"),
                Token::LSquare => String::from("["),
                Token::RSquare => String::from("]"),
                Token::LCurly => String::from("{"),
                Token::RCurly => String::from("}"),
            }
        );
    }
}
