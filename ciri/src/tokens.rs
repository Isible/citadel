use std::{fmt::Display, collections::VecDeque};

use crate::{util, errors::InvalidLiteral};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    // --special-characters--
    /// $ - define a constant
    DollarSign,
    /// § reference a function
    Section,
    /// @ - define a function
    At,
    /// % - reference a variable
    PercentSign,
    /// ? - define a variable
    QuestionMark,
    /// = - assign a value to a varable/constant
    Assign,

    // --types--
    /// an integer (the u8 defines the bitwidth)
    Int(u8),
    /// a floating-point (the u8 defines the bidwith)
    Float(u8),
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

    // --others--
    /// A literal value. Can represent a string, integer or float
    Lit(String),
    /// An identifier like a function or variable name
    Ident(String),
    /// An Array literal
    Array(Vec<Token>),
    /// A Vector literal
    Vector(Vec<Token>),
}

impl Token {
    pub fn from_literal(literal: String) -> Result<Token, InvalidLiteral> {
        match literal {
            l if l == Self::DollarSign.to_string() => Ok(Self::DollarSign),
            l if l == Self::Section.to_string() => Ok(Self::Section),
            l if l == Self::At.to_string() => Ok(Self::At),
            l if l == Self::PercentSign.to_string() => Ok(Self::PercentSign),
            l if l == Self::QuestionMark.to_string() => Ok(Self::QuestionMark),
            l if l == Self::Assign.to_string() => Ok(Self::Assign),
            l if l == Self::Lcl.to_string() => Ok(Self::Lcl),
            l if l == Self::Pub.to_string() => Ok(Self::Pub),
            l if l == Self::Abst.to_string() => Ok(Self::Abst),
            _ => {
                // Try parsing an int, float or literal and return an error if that fails
                let mut literal_chars: VecDeque<char> = literal.chars().collect();
                let prefix = match literal_chars.pop_front() {
                    Some(prefix) => prefix,
                    None => return Err(InvalidLiteral(literal)),
                };
                match prefix {
                    'i' | 'f' => {
                        let literal_without_prefix: String = literal_chars.into_iter().collect();
                        let bidwith = match literal_without_prefix.parse() {
                            Ok(bwidth) => bwidth,
                            Err(_) => return Err(InvalidLiteral(literal)),
                        };
                        return match prefix {
                            'i' => Ok(Token::Int(bidwith)),
                            'f' => Ok(Token::Float(bidwith)),
                            _ => Err(InvalidLiteral(literal)),
                        };
                    },
                    'l' => {
                        literal_chars.pop_front();
                        literal_chars.pop_back();
                        let val: String = literal_chars.into_iter().collect();
                        return Ok(Token::Lit(val));
                    },
                    _ => {
                        if prefix.is_alphabetic() && util::is_valid_ident(&literal) {
                            return Ok(Token::Ident(literal))
                        }
                        Err(InvalidLiteral(literal))
                    }
                }
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            Token::DollarSign => String::from("$"),
            Token::Section => String::from("§"),
            Token::At => String::from("@"),
            Token::PercentSign => String::from("%"),
            Token::QuestionMark => String::from("?"),
            Token::Assign => String::from("="),
            Token::Int(val) => format!("i{val}"),
            Token::Float(val) => format!("f{val}"),
            Token::Lcl => String::from("lcl"),
            Token::Pub => String::from("pub"),
            Token::Abst => String::from("abst"),
            Token::Lit(val) => format!("l{{{val}}}"),
            Token::Ident(val) => val.to_owned(),
            Token::ArrayType(tok, len) => format!("[{}; {}]", (**tok), len),
            Token::VectorType(tok) => format!("<{}>", (**tok)),
            Token::Array(arr) => util::vec_to_arr_string(arr),
            Token::Vector(vec) => util::vec_to_vec_string(vec),
        });
    }
}