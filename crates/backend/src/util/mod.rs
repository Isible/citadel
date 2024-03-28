//! Utilites and general purpose functions for the backend

use std::fmt::Debug;

use crate::experimental::asm::elements::{Declaration, Operand};

pub(crate) trait VecDisplay: Debug {
    fn to_string(&self) -> String;
}

impl VecDisplay for Vec<Operand> {
    fn to_string(&self) -> String {
        let str: String = self
            .iter()
            .map(|op| {
                let mut str = op.to_string();
                str.push(',');
                str
            })
            .collect();
        if self.len() > 0 {
            (&str[..str.len() - 1]).into()
        } else {
            String::new()
        }
    }
}

impl VecDisplay for Vec<Declaration> {
    fn to_string(&self) -> String {
        let str: String = self
            .iter()
            .map(|decl| {
                let mut str = decl.to_string();
                str.push('\n');
                str
            })
            .collect();
        (&str[..str.len() - 1]).into()
    }
}
