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

        panic!("{}: {}", "Parser Error".red(), format_args!($($arg)+));
    }};
}

#[inline(always)]
pub fn trim_lit_char<'tok>(ch: &'tok str) -> Token<'tok> {
    Token::LitChar(&ch[2..ch.len() - 1])
}

#[inline(always)]
pub fn trim_lit_int<'tok>(int: &'tok str) -> Token<'tok> {
    Token::LitInt(&int[2..int.len() - 1])
}

#[inline(always)]
pub fn trim_lit_str<'tok>(str: &'tok str) -> Token<'tok> {
    Token::LitString(&str[3..str.len() - 2])
}
