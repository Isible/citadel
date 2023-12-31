use std::{fmt::{Debug, Display, write}, error::Error};

macro_rules! impl_error {
    ($ty:ty) => {
        impl Error for $ty {}
    };
}

#[derive(Debug)]
pub(crate) struct InvalidKeyError<T: Debug>(pub(crate) T);

impl<T: Debug> Error for InvalidKeyError<T> {}

impl<T: Debug> Display for InvalidKeyError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The given key: {:?} can not be found", self.0)
    }
}

// first field (0) is the arg index that is missing
#[derive(Debug)]
pub(crate) struct InvalidArgError(pub(crate) usize);

impl_error!(InvalidArgError);

impl Display for InvalidArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Arg at pos: {} is missing or invalid.", self.0)
    }
}

#[derive(Debug)]
pub(crate) struct LexerError(pub(crate) String);

impl_error!(LexerError);

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub(crate) struct InterpreterError(pub(crate) Box<dyn Error>);

impl_error!(InterpreterError);

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub(crate) struct InvalidLiteral(pub(crate) String);

impl_error!(InvalidLiteral);

impl Display for InvalidLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot get Token from invalid literal: {}", self.0)
    }
}