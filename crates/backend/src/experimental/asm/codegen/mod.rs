//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

pub mod util;

use std::collections::HashSet;

use citadel_frontend::ir::{
    self, CallExpr, FuncStmt, IRExpr, IRStmt, LabelStmt, ReturnStmt, VarStmt,
};

use crate::experimental::asm::elements::{
    AsmElement, Declaration, Directive, DirectiveType, Literal,
};

use super::elements::{self, DataSize, Instruction, Label, MemAddr, Opcode, Operand, Register, StdFunction};

#[derive(Default)]
pub struct CodeGenerator {
    pub out: Vec<AsmElement>,

    // Literals
    /// Read only data section
    pub rodata: Vec<Declaration>,
    /// Literal constant index
    pub lc_index: usize,

    pub defined_functions: HashSet<StdFunction>,

    pub stack_pointer: isize,
}

impl CodeGenerator {
    pub fn create_entry(&mut self) {
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
            content: vec![Declaration::Global("_start".to_string())],
        }));
    }

    pub fn gen_stmt(&mut self, node: &IRStmt) {
        match node {
            IRStmt::DeclaredFunction(node) => todo!(),
            IRStmt::Function(node) => self.gen_function(node),
            IRStmt::Variable(node) => self.gen_variable(node),
            IRStmt::Label(node) => self.gen_label(node),
            IRStmt::Return(node) => self.gen_return(node),
            IRStmt::Break(node) => todo!(),
            IRStmt::Jump(node) => todo!(),
            IRStmt::Call(node) => self.gen_call(node),
            IRStmt::Expression(IRExpr::Call(node)) => self.gen_call(node),
            IRStmt::Expression(node) => todo!("{:?}", node),
        }
    }

    fn gen_call(&mut self, node: &CallExpr) {
        match node.name.as_str() {
            "print" => self.gen_print(node),
            _ => self.out.push(util::gen_call(&node.name)),
        }
    }

    fn gen_return(&mut self, node: &ReturnStmt) {
        let ret_val = match &node.ret_val {
            IRExpr::Ident(ident) => ident.clone(),
            _ => todo!("Handle non-literal expressions here"),
        };
        self.out.push(util::gen_mov_ins(
            Operand::Register(Register::Rax),
            util::get_stack_location(self.stack_pointer as i32),
        ));
        self.out.push(util::destroy_stackframe());
    }

    fn gen_variable(&mut self, node: &VarStmt) {
        let size = match node.name._type.as_str() {
            "i8" => 1,
            "i16" => 2,
            "i32" => 4,
            "i64" => 8,
            "i128" => 16,
            typename => todo!("Compilation of type: {} is not implemented yet", typename),
        };
        let val = match &node.val {
            IRExpr::Literal(lit) => match lit {
                ir::Literal::Int64(val) => *val as i32,
                int => todo!("Handle non-i32 literals here: {:?}", int),
            },
            _ => todo!("Handle non-literal expressions here"),
        };
        self.out.push(util::gen_mov_ins(
            util::get_stack_location(self.stack_pointer as i32 - size),
            Operand::SizedLiteral(Literal::Int(val), DataSize::DWord),
        ));
        self.stack_pointer -= size as isize;
    }

    fn gen_function(&mut self, node: &FuncStmt) {
        self.out.push(AsmElement::Label(Label {
            name: node.name.ident.clone(),
        }));

        let stack_frame = util::create_stackframe();

        self.out.push(stack_frame.0);
        self.out.push(stack_frame.1);

        for stmt in &node.block.stmts {
            self.gen_stmt(stmt);
        }

        if node.name._type == "void" {
            self.out.push(util::destroy_stackframe());
        }
        self.out.push(util::gen_ret());
    }

    fn gen_print(&mut self, node: &CallExpr) {
        let arg: String = util::string_from_lit(&node.args[0]).into();
        dbg!(&arg);
        self.out.push(util::gen_mov_ins(
            Operand::Register(Register::Rsi),
            Operand::Ident(format!("LC{}", self.lc_index)),
        ));
        self.out.push(util::gen_mov_ins(
            Operand::Register(Register::Rdx),
            Operand::Literal(Literal::Int((arg.len() + 1) as i32)),
        ));
        self.out.push(util::gen_call("print"));
        self.rodata.push(Declaration::DefineBytes(
            format!("LC{}", self.lc_index),
            arg,
            0xa,
        ));
        self.lc_index += 1;
        self.defined_functions.insert(StdFunction::Print);
    }

    fn gen_label(&mut self, node: &LabelStmt) {
        match node.name.as_str() {
            "_entry" => {
                self.create_entry();
                self.out.push(AsmElement::Label(Label {
                    name: "_start".to_string(),
                }))
            }
            _ => self.out.push(AsmElement::Label(Label {
                name: node.name.clone(),
            })),
        }
        for stmt in &node.block.stmts {
            self.gen_stmt(stmt);
        }
    }
}
