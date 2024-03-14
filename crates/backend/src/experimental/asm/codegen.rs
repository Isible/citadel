//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

use citadel_frontend::ir::{CallExpr, IRStmt, LabelStmt};

use crate::experimental::asm::elements::{AsmElement, Declaration, Directive, DirectiveType};

use super::elements::{Block, Instruction, InstructionType, Label, Operand};

pub struct CodeGenerator {
    pub out: Vec<AsmElement>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { out: Vec::new() }
    }

    pub fn create_entry(&mut self) {
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
            content: vec![Declaration::Global("_start".to_string())],
        }));
    }

    pub fn compile_stmt(&mut self, node: &IRStmt) {
        match node {
            IRStmt::DeclaredFunction(node) => todo!(),
            IRStmt::Function(node) => todo!(),
            IRStmt::Variable(node) => todo!(),
            IRStmt::Label(node) => self.compile_label(node),
            IRStmt::Return(node) => todo!(),
            IRStmt::Break(node) => todo!(),
            IRStmt::Jump(node) => todo!(),
            IRStmt::Call(node) => self.compile_call(node),
            IRStmt::Expression(node) => todo!(),
        }
    }

    fn compile_call(&mut self, node: &CallExpr) {
        self.out.push(AsmElement::Instruction(Instruction {
            _type: InstructionType::Call,
            args: vec![Operand::Ident(node.name.clone())],
        }));
    }

    fn compile_label(&mut self, node: &LabelStmt) {
        match node.name.as_str() {
            "_entry" => {
                self.create_entry();
                self.out.push(AsmElement::Label(Label {
                    name: "_start".to_string(),
                    block: Block { elements: vec![] },
                }))
            }
            _ => self.out.push(AsmElement::Label(Label {
                name: node.name.clone(),
                block: Block { elements: vec![] },
            })),
        }
        for stmt in &node.block.stmts {
            self.compile_stmt(stmt);
        }
    }
}
