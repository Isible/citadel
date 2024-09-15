use std::collections::HashMap;

use bumpalo::Bump;
use citadel_frontend::hir::{self, irgen::HIRStream, IRStmt, Literal};

use super::elements::{DataValue, Instruction, Register};

pub struct CodeGenerator<'c> {
    // out
    pub instructions: Vec<Instruction>,
    pub data: Vec<DataValue>,

    // in
    types: hir::TypeTable<'c>,

    // tracking
    labels: HashMap<&'c str, usize>,

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
            IRStmt::Function(_) => todo!(),
            IRStmt::Variable(_) => todo!(),
            IRStmt::Label(_) => todo!(),
            IRStmt::Return(_) => todo!(),
            IRStmt::Exit(stmt) => self.gen_exit_stmt(stmt),
            IRStmt::Jump(_) => todo!(),
            IRStmt::Call(_) => todo!(),
            IRStmt::Struct(_) => todo!(),
            IRStmt::Union(_) => todo!(),
        }
    }

    fn gen_entry_stmt(&mut self, stmt: hir::BlockStmt<'c>) {
        self.labels.insert("_start", self.instructions.len());
        for stmt in stmt.stmts {
            self.gen_stmt(stmt);
        }
    }
    
    fn gen_exit_stmt(&mut self, stmt: hir::ExitStmt<'_>) {
        self.instructions.push(Instruction::MovI2R { val: 60, dest: Register::Rax });
        self.instructions.push(Instruction::MovI2R { val: match stmt.exit_code {
            hir::IRExpr::Literal(Literal::Int32(i), _) => i as i64,
            expr => todo!("{:?}", expr),
        }, dest: Register::Rdi });
        self.instructions.push(Instruction::Syscall);
    }
}
