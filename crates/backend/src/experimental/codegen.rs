//! The AsmGenerator is the equivalent to IRGenerator but for assembly instructions.
//! Under the hood it also uses a vector to represent asm elements. This is purely
//! a helper struct, meaning apis will only require the vector of asm elements.
//! This means you can also implement your own version of this easily.

use std::fmt::Display;

use super::elements::AsmElement;

#[derive(Debug, Default)]
pub struct AsmGenerator {
    out: Vec<AsmElement>,
}

impl AsmGenerator {
    pub fn generate(&mut self, elem: AsmElement) {
        self.out.push(elem);
    }

    pub fn get_out_ref(&self) -> &Vec<AsmElement> {
        &self.out
    }

    pub fn stream(self) -> Vec<AsmElement> {
        self.out
    }
}

impl Display for AsmGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.out
                .iter()
                .map(|elem| elem.to_string())
                .collect::<String>()
        )
    }
}
