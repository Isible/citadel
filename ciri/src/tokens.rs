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

    // --types--
    /// an integer (the u8 defines the bitwidth)
    IntType(u8),
    /// a floating-point (the u8 defines the bidwith)
    FloatType(u8),
    /// array type Box<Token> defines the type. Only Float, Int, ArrayType, VectorType are vaild for this.
    /// u8 defines the array size
    ArrayType(Box<Token>, u8),
    ///
    VectorType(Box<Token>),

    // --keywords--
    /// sets the access of a variable to local
    Lcl,
    /// sets the access of a variable to public
    Pub,
    /// marks a function as abstract meaning it gets initialized in a different module
    Abst,
    /// Call a function
    Call,
    /// Return a value
    Ret,

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
                Token::Apostrophe => String::from("'"),
                Token::IntType(val) => format!("i{val}"),
                Token::FloatType(val) => format!("f{val}"),
                Token::Lcl => String::from("lcl"),
                Token::Pub => String::from("pub"),
                Token::Abst => String::from("abst"),
                Token::Lit(val) => format!("l{{{val}}}"),
                Token::RawLit(lit) => lit.to_string(),
                Token::Ident(val) => val.to_owned(),
                Token::Eof => String::from("EOF"),
                Token::ArrayType(tok, len) => format!("[{}; {}]", (**tok), len),
                Token::VectorType(tok) => format!("<{}>", (**tok)),
                Token::Call => String::from("call"),
                Token::Ret => String::from("ret"),
            }
        );
    }
}
