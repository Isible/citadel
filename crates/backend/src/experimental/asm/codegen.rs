//! This module is used for generating assembly code.
//! It's implementation is very similar to the IRGenerator
//! from [frontend::ir::generator](../../../../frontend/ir/irgen/index.html).

use std::fmt::Display;

use crate::experimental::asm::elements::AsmElement;

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
