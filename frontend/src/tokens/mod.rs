// TODO: think about moving this to a shared folder cuz it is used by the backend as well

use std::collections::VecDeque;

use clutils::literal::LiteralString;

use crate::{errors::results::InvalidLiteral, util};

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

    // --types--
    /// an integer (the u8 defines the bitwidth)
    Int(u8),
    /// a floating-point (the u8 defines the bidwith)
    Float(u8),

    // --keywords--
    /// sets the visibility of a variable to local
    Lcl,
    /// sets the visibility of a variable to public
    Pub,
    /// marks a function as abstract meaning it gets initialized in a different module
    Abst,

    // --others--
    /// A literal value. Can represent a string, integer or float
    Lit(String),
    /// An identifier like a function or variable name
    Ident(String),
}

impl Token {
    pub fn from_literal(literal: String) -> Result<Token, InvalidLiteral> {
        match literal {
            l if l == Self::DollarSign.literal() => Ok(Self::DollarSign),
            l if l == Self::Section.literal() => Ok(Self::Section),
            l if l == Self::At.literal() => Ok(Self::At),
            l if l == Self::PercentSign.literal() => Ok(Self::PercentSign),
            l if l == Self::QuestionMark.literal() => Ok(Self::QuestionMark),
            l if l == Self::Lcl.literal() => Ok(Self::Lcl),
            l if l == Self::Pub.literal() => Ok(Self::Pub),
            l if l == Self::Abst.literal() => Ok(Self::Abst),
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
                    }
                    'l' => {
                        literal_chars.pop_front();
                        literal_chars.pop_back();
                        let val: String = literal_chars.into_iter().collect();
                        return Ok(Token::Lit(val));
                    }
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

impl LiteralString for Token {
    fn literal(self: &Self) -> String {
        return match self {
            Token::DollarSign => String::from("$"),
            Token::Section => String::from("§"),
            Token::At => String::from("@"),
            Token::PercentSign => String::from("%"),
            Token::QuestionMark => String::from("?"),
            Token::Int(val) => format!("i{val}"),
            Token::Float(val) => format!("f{val}"),
            Token::Lcl => String::from("lcl"),
            Token::Pub => String::from("pub"),
            Token::Abst => String::from("abst"),
            Token::Lit(val) => format!("l{{{val}}}"),
            Token::Ident(val) => val.to_owned(),
        };
    }
}
