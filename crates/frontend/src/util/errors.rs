//! Errors that can be returned by the frontend.

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidLiteralError(pub String);

impl Error for InvalidLiteralError {}

impl Display for InvalidLiteralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("The provided literal \"{}\" is invalid", self.0).as_str())
    }
}
