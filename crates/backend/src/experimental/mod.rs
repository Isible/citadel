/*
 * An experimental assembly compiler
 */

use frontend::ir::IRStmt;

use self::{
    api::{Backend, Target},
    elements::AsmElement,
};

pub mod api;
pub mod code_gen;
pub mod compiler;
pub mod elements;
pub mod traits;
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

    fn compile(&mut self, ir_stream: &Vec<IRStmt>) -> Self::Output {
        util::compile_program(ir_stream)
    }
}
