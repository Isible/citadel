/*
 * An experimental assembly compiler
 */

use self::{asm::AsmElement, exp_api::{Backend, Target}};

pub mod asm;
pub mod exp_api;

#[derive(Debug, Default)]
pub struct AsmTarget;

impl Target for AsmTarget {
    fn name(&self) -> &str {
        "assembly-target"
    }
}

#[derive(Debug)]
pub struct AsmBackend;

impl Backend for AsmBackend {
    type Output = Vec<AsmElement>;

    type Target = AsmTarget;

    fn compile(&mut self, ir_stream: frontend::ir_gen::IRGenerator) -> Self::Output {
        todo!()
    }
}

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }
}
