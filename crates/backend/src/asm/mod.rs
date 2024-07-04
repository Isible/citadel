//! The assembly module of the backend containing an
//! experimental/prototype compiler for x86-64 assembly
//! leveraging the [backend api](api/index.html).

pub mod codegen;
pub mod elements;
pub mod util;

use citadel_frontend::ir::irgen::HIRStream;

use crate::{
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
    type Output = Vec<AsmElement>;
    type Target = T;

    fn target(&self) -> Self::Target {
        self.target
    }

    fn generate(&self, ir_stream: HIRStream) -> Self::Output {
        util::compile_program(ir_stream, self.target())
    }
}
