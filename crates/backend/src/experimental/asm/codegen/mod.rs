//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

pub mod util;

use std::{borrow::Borrow, collections::{HashMap, HashSet}};

use citadel_frontend::ir::{
    self, CallExpr, FuncStmt, IRExpr, IRStmt, LabelStmt, ReturnStmt, VarStmt,
};

use crate::experimental::asm::elements::{
    AsmElement, Declaration, Directive, DirectiveType, Literal,
};

use super::elements::{DataSize, Instruction, Label, Opcode, Operand, Register, StdFunction};

pub const FUNCTIONS_ARG_REGISTERS: [Register; 6] = [
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R9,
    Register::R10,
];

#[derive(Default)]
pub struct CodeGenerator<'c> {
    pub out: Vec<AsmElement>,

    // Literals
    /// Read only data section
    pub rodata: Vec<Declaration>,
    /// Literal constant index
    pub lc_index: usize,

    pub defined_functions: HashSet<StdFunction>,
    pub symbol_table: HashMap<&'c str, isize>,

    pub stack_pointer: isize,
}

impl<'c> CodeGenerator<'c> {
    pub fn create_entry(&mut self) {
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
            content: vec![Declaration::Global("_start".to_string())],
        }));
    }

    pub fn gen_stmt(&mut self, node: &'c IRStmt) {
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
            util::get_stack_location(*self.symbol_table.get(ret_val.as_str()).unwrap(),),
        ));
        self.out.push(util::destroy_stackframe());
        self.out.push(util::gen_ret());
    }

    fn gen_variable(&mut self, node: &'c VarStmt) {
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
                ir::Literal::Int32(val) => *val as i32,
                int => todo!("Handle non-i32 literals here: {:?}", int),
            },
            _ => todo!("Handle non-literal expressions here"),
        };
        self.out.push(util::gen_mov_ins(
            util::get_stack_location((self.stack_pointer as i32 - size).try_into().unwrap()),
            Operand::SizedLiteral(Literal::Int(val), DataSize::DWord),
        ));
        self.stack_pointer -= size as isize;
        self.symbol_table
            .insert(&node.name.ident, self.stack_pointer);
    }

    fn gen_function(&mut self, node: &'c FuncStmt) {
        self.out.push(AsmElement::Label(Label {
            name: node.name.ident.clone(),
        }));

        let stack_frame = util::create_stackframe();

        self.out.push(stack_frame.0);
        self.out.push(stack_frame.1);

        for stmt in &node.block.stmts {
            self.gen_stmt(stmt);
        }
        if let Some(elem) = self.out.last() {
            match elem {
                AsmElement::Instruction(Instruction {
                    opcode: Opcode::Ret,
                    args: _,
                }) => (),
                _ => {
                    if node.name._type == "void" {
                        self.out.push(util::destroy_stackframe());
                    }
                    self.out.push(util::gen_ret());
                }
            }
        }
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

    fn gen_label(&mut self, node: &'c LabelStmt) {
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
