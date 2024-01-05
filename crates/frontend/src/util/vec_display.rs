use std::fmt::Debug;

pub trait VecDisplay: Debug {
    fn to_string(&self) -> String;
}