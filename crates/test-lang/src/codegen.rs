//! Code generation module leveraging the backend to translate IR to assembly.

use backend::experimental::{
    api::Backend,
    asm::{elements::AsmElement, AsmBackend},
};
use frontend::ir::IRStmt;

#[derive(Debug)]
pub struct CodeGenerator {
    pub(crate) backend: AsmBackend,
    pub(crate) ir_stream: Vec<IRStmt>,
}

impl CodeGenerator {
    pub fn new(ir_stream: Vec<IRStmt>) -> Self {
        Self {
            backend: AsmBackend::default(),
            ir_stream,
        }
    }

    pub fn compile(mut self) -> Vec<AsmElement> {
        self.backend.compile(self.ir_stream)
    }
}
