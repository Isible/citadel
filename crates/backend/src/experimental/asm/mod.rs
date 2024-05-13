//! The assembly module of the backend containing an
//! experimental/prototype compiler for x86-64 assembly
//! leveraging the [backend api](api/index.html).

pub mod codegen;
pub mod elements;
pub mod util;

mod tests;

use citadel_frontend::ir::irgen::IRStream;

use crate::experimental::{
    api::{Backend, Target},
    asm::elements::AsmElement,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TargetX86_64;

impl Target for TargetX86_64 {
    fn name(&self) -> &str {
        "x86-64"
    }
}

#[derive(Debug, Default)]
pub struct AsmBackend<T: Target> {
    target: T,
}

impl<T: Target> AsmBackend<T> {
    pub fn new(target: T) -> Self {
        Self { target }
    }
}

impl<T: Target> Backend for AsmBackend<T> {
    type Element = AsmElement;
    type Output = Vec<AsmElement>;
    type Target = T;

    fn target(&self) -> Self::Target {
        self.target
    }

    fn generate(&self, ir_stream: IRStream) -> Self::Output {
        util::compile_program(ir_stream, self.target())
    }
}
