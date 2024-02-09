use frontend::ir::IRStmt;

use super::{code_gen::CodeGenerator, elements::AsmElement};

pub struct Compiler {
    input: Vec<IRStmt>,
    generator: CodeGenerator,
}

impl Compiler {
    pub fn new(input: Vec<IRStmt>) -> Self {
        Self {
            input,
            generator: CodeGenerator::default(),
        }
    }

    pub fn compile_program(&mut self) -> Vec<AsmElement> {
        todo!()
    }
}