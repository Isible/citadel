//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//! 
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

use frontend::ir::IRStmt;

use super::elements::{AsmElement, Declaration, Directive, DirectiveType};

pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn create_header(&self) -> AsmElement {
        AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
            content: vec![Declaration::Global("start".to_string())]
        })
    }

    pub fn compile_stmt(&mut self, node: &IRStmt) -> AsmElement {
        match node {
            IRStmt::DeclaredFunction(node) => todo!(),
            IRStmt::Function(node) => todo!(),
            IRStmt::Variable(node) => todo!(),
            IRStmt::Label(node) => todo!(),
            IRStmt::Return(node) => todo!(),
            IRStmt::Break(node) => todo!(),
            IRStmt::Jump(node) => todo!(),
            IRStmt::Call(node) => todo!(),
            IRStmt::Expression(node) => todo!(),
        }
    }
}