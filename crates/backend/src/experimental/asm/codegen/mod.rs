//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

pub mod util;

use std::{
    collections::{HashMap, HashSet}, f32::consts::E, vec
};

use citadel_frontend::ir::{
    self, ArithOpExpr, CallExpr, ExitStmt, FuncStmt, IRExpr, IRStmt, LabelStmt, ReturnStmt, VarStmt,
};

use crate::experimental::asm::elements::{
    AsmElement, Declaration, Directive, DirectiveType, Literal,
};

use super::elements::{Instruction, Label, Opcode, Operand, Register, Size, StdFunction};

pub const FUNCTION_ARG_REGISTERS_8: [Register; 6] = [
    Register::Al,
    Register::Bl,
    Register::Cl,
    Register::Dl,
    Register::R9b,
    Register::R10b,
];

pub const FUNCTION_ARG_REGISTERS_16: [Register; 6] = [
    Register::Ax,
    Register::Bx,
    Register::Cx,
    Register::Dx,
    Register::R9w,
    Register::R10w,
];

pub const FUNCTION_ARG_REGISTERS_32: [Register; 6] = [
    Register::Edi,
    Register::Esi,
    Register::Edx,
    Register::Ecx,
    Register::R9d,
    Register::R10d,
];

pub const FUNCTION_ARG_REGISTERS_64: [Register; 6] = [
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R9,
    Register::R10,
];
// TODO: Implement type suffixes for literals in the IR
#[derive(Default)]
pub struct CodeGenerator<'c> {
    pub out: Vec<AsmElement>,

    // Literals
    /// Read only data section
    pub rodata: Vec<Declaration>,
    /// Literal constant index
    pub lc_index: usize,

    pub defined_functions: HashSet<StdFunction>,
    pub symbol_table: HashMap<&'c str, i32>,

    pub stack_pointer: i32,
}

impl<'c> CodeGenerator<'c> {
    pub fn create_entry(&mut self) {
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
        }));
        self.out.push(AsmElement::Declaration(Declaration::Global(
            "_start".to_string(),
        )))
    }

    pub fn gen_stmt(&mut self, node: &'c IRStmt) {
        match node {
            IRStmt::DeclaredFunction(node) => todo!(),
            IRStmt::Function(node) => self.gen_function(node),
            IRStmt::Variable(node) => self.gen_variable(node),
            IRStmt::Label(node) => self.gen_label(node),
            IRStmt::Return(node) => self.gen_return(node),
            IRStmt::Exit(node) => self.gen_exit(node),
            IRStmt::Break(node) => todo!(),
            IRStmt::Jump(node) => todo!(),
            IRStmt::Call(node) => self.gen_call(node),
            _ => panic!("//TODO:")
        }
    }

    fn gen_expr(&mut self, node: &'c IRExpr) -> Operand {
        match &node {
            IRExpr::Literal(lit) => match lit {
                ir::Literal::Int32(val) => Operand::Literal(Literal::Int(*val)),
                int => todo!("Handle non-i32 literals here: {:?}", int),
            },
            IRExpr::Call(call) => {
                self.gen_call(call);
                let reg = Register::Rax;
                Operand::Register(reg)
            }
            IRExpr::ArithOp(arithop) => self.gen_arith_op(arithop, true),
            IRExpr::Ident(ident) => util::get_stack_location(
                *self
                    .symbol_table
                    .get(ident.0.as_str())
                    .unwrap_or_else(|| panic!("Could not find ident with name {ident:?}")),
            ),
        }
    }

    fn gen_call(&mut self, node: &'c CallExpr) {
        match node.name.as_str() {
            "print" => self.gen_print(node),
            _ => {
                self.gen_call_args(node);
                self.out.push(util::gen_call(&node.name))
            }
        }
    }

    fn gen_arith_op(&mut self, node: &'c ArithOpExpr, move_to_rax: bool) -> Operand {
        if move_to_rax {
            let left_expr = self.gen_expr(&*node.values.0);
            self.out.push(util::gen_mov_ins(
                Operand::Register(Register::Rax),
                left_expr,
            ))
        }
        let arith_op = match node.op {
            ir::Operator::Add => AsmElement::Instruction(Instruction {
                opcode: Opcode::Add,
                args: vec![
                    Operand::Register(Register::Rax),
                    match &*node.values.1 {
                        IRExpr::ArithOp(expr) => self.gen_arith_op(expr, false),
                        expr => self.gen_expr(expr),
                    },
                ],
            }),
            ir::Operator::Sub => todo!(),
            ir::Operator::Mul => todo!(),
            ir::Operator::Div => todo!(),
        };
        self.out.push(arith_op);
        Operand::Register(Register::Rax)
    }

    fn gen_return(&mut self, node: &'c ReturnStmt) {
        let val = self.gen_expr(&node.ret_val);
        self.out
            .push(util::gen_mov_ins(Operand::Register(Register::Rax), val));
        self.out.push(util::destroy_stackframe());
        self.out.push(util::gen_ret());
    }

    fn gen_exit(&mut self, node: &'c ExitStmt) {
        let expr = self.gen_expr(&node.exit_code);
        self.out
            .push(util::gen_mov_ins(Operand::Register(Register::Rdi), expr));
        self.out.push(util::gen_mov_ins(
            Operand::Register(Register::Rax),
            Operand::Literal(Literal::Int(60)),
        ));
        self.out.push(util::gen_syscall());
    }

    fn gen_variable(&mut self, node: &'c VarStmt) {
        let size = util::int_size(&node.name._type);
        let val = self.gen_expr(&node.val);
        self.out.push(util::gen_mov_ins(
            util::get_stack_location((self.stack_pointer - (size / 8) as i32).try_into().unwrap()),
            val,
        ));
        self.stack_pointer -= (size / 8) as i32;
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

        self.gen_args(node);

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

    fn gen_args(&mut self, node: &'c FuncStmt) {
        for (i, expr) in node.args.iter().enumerate() {
            let size = util::int_size(&expr._type);
            self.out.push(util::gen_mov_ins(
                util::get_stack_location((self.stack_pointer - (size / 8) as i32).try_into().unwrap()),
                Operand::Register(util::arg_regs_by_size(size as u8)[i]),
            ));
            self.stack_pointer -= (size / 8) as i32;
            self.symbol_table.insert(&expr.ident, self.stack_pointer);
        }
    }

    fn gen_call_args(&mut self, node: &'c CallExpr) {
        for (i, expr) in node.args.iter().enumerate() {
            let val = self.gen_expr(expr);
            self.out.push(util::gen_mov_ins(
                Operand::Register(util::arg_regs_by_size(val.size())[i]),
                val,
            ));
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
