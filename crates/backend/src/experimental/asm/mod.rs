//! The assembly module of the backend containing an
//! experimental/prototype compiler for x86-64 assembly
//! leveraging the [backend api](api/index.html).

pub mod codegen;
pub mod compiler;
pub mod util;
pub mod elements;

mod tests;

use frontend::ir::IRStmt;

use crate::experimental::{
    api::{Backend, Target},
    asm::elements::AsmElement,
};

#[derive(Debug, Default)]
pub struct AsmTarget;

impl Target for AsmTarget {
    fn name(&self) -> &str {
        "assembly-x86-64"
    }
}

#[derive(Debug, Default)]
pub struct AsmBackend;

impl Backend for AsmBackend {
    type Output = Vec<AsmElement>;

    type Target = AsmTarget;

    fn compile(&mut self, ir_stream: Vec<IRStmt>) -> Self::Output {
        util::compile_program(ir_stream)
    }
}
