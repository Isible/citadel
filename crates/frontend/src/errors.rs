use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidLiteral(pub String);

impl Error for InvalidLiteral {}

impl Display for InvalidLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("The provided literal \"{}\" is invalid", self.0).as_str())
    }
}
