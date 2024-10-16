pub mod machine;

use std::{collections::HashMap, sync::Arc};

use bumpalo::Bump;
use citadel_frontend::hir::{self, IRStmt, Literal};

use super::elements::{DataValue, Instruction, Operand, Register};

pub struct CodeGenerator<'c> {
    // out
    pub instructions: Vec<Instruction<'c>>,
    pub data: Vec<DataValue>,

    // in
    types: hir::TypeTable<'c>,

    // tracking
    pub labels: HashMap<&'c str, usize>,
    pub entry_size: usize,

    // utils
    arena: &'c Bump,
}

impl<'c> CodeGenerator<'c> {
    pub fn new(arena: &'c Bump, types: hir::TypeTable<'c>) -> Self {
        Self {
            instructions: Vec::new(),
            data: Vec::new(),
            types,
            labels: HashMap::new(),
            arena,
            entry_size: 0
        }
    }

    pub fn generate(&mut self, ir_stream: Vec<IRStmt<'c>>) {
        for stmt in ir_stream {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&mut self, stmt: IRStmt<'c>) {
        match stmt {
            IRStmt::Entry(stmt) => self.gen_entry_stmt(stmt),
            IRStmt::DeclaredFunction(_) => todo!(),
            IRStmt::Function(stmt) => self.gen_function_stmt(stmt),
            IRStmt::Variable(_) => todo!(),
            IRStmt::Label(_) => todo!(),
            IRStmt::Return(_) => todo!(),
            IRStmt::Exit(stmt) => self.gen_exit_stmt(stmt),
            IRStmt::Jump(_) => todo!(),
            IRStmt::Call(stmt) => self.gen_call_stmt(stmt),
            IRStmt::Struct(_) => todo!(),
            IRStmt::Union(_) => todo!(),
        }
    }

    fn gen_entry_stmt(&mut self, stmt: hir::BlockStmt<'c>) {
        let entry = self.instructions.len();
        self.labels.insert("_start".into(), entry);
        for stmt in stmt.stmts {
            self.gen_stmt(stmt);
        }
        self.entry_size = self.instructions.len() - entry;
    }

    fn gen_exit_stmt(&mut self, stmt: hir::ExitStmt<'c>) {
        self.instructions.push(Instruction::MovI2R {
            val: 60,
            dest: Register::Rax,
        });
        let val = self.gen_expr(stmt.exit_code);
        self.instructions.push(Self::move_ins(val, Operand::Register(Register::Rdi)));
        self.instructions.push(Instruction::Syscall);
    }

    fn gen_expr(&mut self, expr: hir::IRExpr<'c>) -> Operand {
        match expr {
            hir::IRExpr::Call(call_expr) => todo!(),
            hir::IRExpr::Literal(expr, _) => self.gen_literal_expr(expr),
            hir::IRExpr::Ident(_) => todo!(),
            hir::IRExpr::BinOp(bin_op_expr) => todo!(),
            hir::IRExpr::StructInit(struct_init_expr) => todo!(),
        }
    }

    fn gen_literal_expr(&mut self, expr: hir::Literal<'c>) -> Operand {
        match expr {
            Literal::String(_) => todo!(),
            Literal::Char(expr) => Operand::Immediate(expr as i64),
            Literal::Float32(_) => todo!(),
            Literal::Float64(_) => todo!(),
            Literal::Bool(expr) => Operand::Immediate(if expr { 1 } else { 0 }),
            Literal::Int8(expr) => Operand::Immediate(expr as i64),
            Literal::Int16(expr) => Operand::Immediate(expr as i64),
            Literal::Int32(expr) => Operand::Immediate(expr as i64),
            Literal::Int64(expr) => Operand::Immediate(expr),
            Literal::Array(_, _) => todo!(),
            Literal::Vector(_) => todo!(),
        }
    }

    fn move_ins(val: Operand, dest: Operand) -> Instruction<'c> {
        match (val, dest) {
            (Operand::Register(val), Operand::Register(dest)) => Instruction::MovR2R { val, dest },
            (Operand::Immediate(val), Operand::Register(dest)) => Instruction::MovI2R { val, dest },
            (val, dest) => todo!("Move for: {val:?}, {dest:?}")
        }
    }
    
    fn gen_call_stmt(&mut self, stmt: hir::CallExpr<'c>) {
        self.instructions.push(Instruction::Call { func: stmt.name });
    }
    
    fn gen_function_stmt(&mut self, stmt: hir::FuncStmt<'c>) {
        let ins_index = self.instructions.len();
        for stmt in stmt.block.stmts {
            self.gen_stmt(stmt);
        }
        self.labels.insert(stmt.name.ident, ins_index);
    }
}
