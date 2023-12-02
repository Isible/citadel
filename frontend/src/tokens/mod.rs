// TODO: think about moving this to a shared folder cuz it is used by the backend as well

use std::collections::VecDeque;

use clutils::literal::LiteralString;

use crate::errors::results::InvalidLiteral;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
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
}

impl TokenType {
    pub fn from_literal(literal: String) -> Result<TokenType, InvalidLiteral> {
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
                            'i' => Ok(TokenType::Int(bidwith)),
                            'f' => Ok(TokenType::Float(bidwith)),
                            _ => Err(InvalidLiteral(literal)),
                        };
                    }
                    'l' => {
                        literal_chars.pop_front();
                        literal_chars.pop_back();
                        let val: String = literal_chars.into_iter().collect();
                        return Ok(TokenType::Lit(val));
                    }
                    _ => return Err(InvalidLiteral(literal))
                }
            }
        }
    }
}

impl LiteralString for TokenType {
    fn literal(self: &Self) -> String {
        return match self {
            TokenType::DollarSign => String::from("$"),
            TokenType::Section => String::from("§"),
            TokenType::At => String::from("@"),
            TokenType::PercentSign => String::from("%"),
            TokenType::QuestionMark => String::from("?"),
            TokenType::Int(val) => format!("i{val}"),
            TokenType::Float(val) => format!("f{val}"),
            TokenType::Lcl => String::from("lcl"),
            TokenType::Pub => String::from("pub"),
            TokenType::Abst => String::from("abst"),
            TokenType::Lit(val) => format!("l{{{val}}}"),
        };
    }
}
