use std::fmt::{Debug, Display};

pub(crate) struct InvalidKeyError<T: Debug>(T);

impl<T: Debug> Display for InvalidKeyError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The given key: {:?} can not be found", self.0)
    }
}