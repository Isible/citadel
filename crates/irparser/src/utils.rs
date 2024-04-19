use crate::tokens::Token;

#[macro_export]
macro_rules! expect_tok {
    ($tok:expr,$pat:pat,$fail:expr) => {{
        if !matches!($tok, $pat) {
            $fail($tok);
            false
        } else {
            true
        }
    }};
}

#[macro_export]
macro_rules! parser_error {
    ($($arg:tt)+) => {{
        use colored::Colorize;

        eprintln!("{}: {}", "Parser Error".red(), format_args!($($arg)+))
    }};
}

pub fn trim_lit_char<'tok>(ch: &'tok str) -> Token<'tok> {
    Token::LitChar(trim_lit(ch))
}

pub fn trim_lit_int<'tok>(int: &'tok str) -> Token<'tok> {
    let int = trim_lit(int);
    Token::LitInt(int)
}

pub fn trim_lit_str<'tok>(str: &'tok str) -> Token<'tok> {
    let int = trim_lit(str);
    Token::LitString(int)
}

#[inline(always)]
fn trim_lit(lit: &str) -> &str {
    &lit[3..lit.len()-2]
}