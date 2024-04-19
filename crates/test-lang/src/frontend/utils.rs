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