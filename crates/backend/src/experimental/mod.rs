//! The experimental section of the citadel backend conatining the
//! api for the backend and the target as well as
//! an experimental assembly compiler for the x86-64 architecture.
//! This is only used as a prototype for laying out the api and gathering
//! information on how to design the actual backends and compilers.

use frontend::ir::IRStmt;

use self::{
    api::{Backend, Target},
    elements::AsmElement,
};

pub mod api;
pub mod codegen;
pub mod compiler;
pub mod elements;
pub mod util;

mod tests;

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
