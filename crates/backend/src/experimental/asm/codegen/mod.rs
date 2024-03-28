//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

pub mod util;

use citadel_frontend::ir::{CallExpr, FuncStmt, IRStmt, LabelStmt};

use crate::experimental::asm::elements::{AsmElement, Declaration, Directive, DirectiveType};

use super::elements::{Block, Instruction, Label, Literal, Opcode, Operand, Register};

pub struct CodeGenerator {
    pub out: Vec<AsmElement>,

    pub data: Vec<Declaration>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { out: Vec::new(), data: Vec::new() }
    }

    pub fn create_entry(&mut self) {
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
            content: vec![Declaration::Global("_start".to_string())],
        }));
    }

    pub fn gen_stmt(&mut self, node: &IRStmt) {
        match node {
            IRStmt::DeclaredFunction(node) => todo!(),
            IRStmt::Function(node) => todo!(),
            IRStmt::Variable(node) => todo!(),
            IRStmt::Label(node) => self.gen_label(node),
            IRStmt::Return(node) => todo!(),
            IRStmt::Break(node) => todo!(),
            IRStmt::Jump(node) => todo!(),
            IRStmt::Call(node) => self.gen_call(node),
            IRStmt::Expression(node) => todo!(),
        }
    }

    fn gen_call(&mut self, node: &CallExpr) {
        match node.name.as_str() {
            "print" => self.gen_print(node),
            _ => self.out.push(AsmElement::Instruction(Instruction {
                opcode: Opcode::Call,
                args: vec![Operand::Ident(node.name.clone())],
            })),
        }
    }

    fn gen_print(&mut self, node: &CallExpr) {
        let arg = util::string_from_lit(&node.args[0]);
        dbg!(&arg);
        let instructions = vec![
            util::gen_mov_ins(Register::Rax, Operand::Literal(Literal::Int(1))),
            util::gen_mov_ins(Register::Rdi, Operand::Literal(Literal::Int(1))),
            util::gen_mov_ins(Register::Rsi, Operand::Ident(String::from("testing"))),
            util::gen_mov_ins(Register::Rdx, Operand::Literal(Literal::Int(arg.len() as i32))),
            AsmElement::Instruction(Instruction {
                opcode: Opcode::Syscall,
                args: vec![],
            }),
        ];
        self.data.push(Declaration::DefineBytes(
            "testing".to_string(),
            arg.to_string(),
        ));
        self.out.extend(instructions);
    }

    fn gen_label(&mut self, node: &LabelStmt) {
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
            self.gen_stmt(stmt);
        }
    }
}
