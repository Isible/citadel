/*
 * An experimental assembly compiler
 */

use frontend::ast::IRStmt;

use self::{elements::AsmElement, api::{Backend, Target}};

pub mod api;
pub mod code_gen;
pub mod compiler;
pub mod elements;

#[derive(Debug, Default)]
pub struct AsmTarget;

impl Target for AsmTarget {
    fn name(&self) -> &str {
        "assembly-target"
    }
}

#[derive(Debug, Default)]
pub struct AsmBackend;

impl Backend for AsmBackend {
    type Output = Vec<AsmElement>;

    type Target = AsmTarget;

    fn compile(&mut self, _ir_stream: Vec<IRStmt>) -> Self::Output {
        todo!()
    }
}
