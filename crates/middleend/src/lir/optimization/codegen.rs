use citadel_frontend::hir::{self, IRStmt, Literal};

use crate::lir::{irgen::LIRGenerator, Immediate, Instruction, Operand, Register};

impl<'c> LIRGenerator<'c> {
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
        let entry = self.stream_ref().instructions.len();
        self.add_label("main".into(), entry);
        for stmt in stmt.stmts {
            self.gen_stmt(stmt);
        }
        self.set_entry_size(self.stream_ref().instructions.len() - entry);
    }

    fn gen_exit_stmt(&mut self, stmt: hir::ExitStmt<'c>) {
        //self.instructions.push(Instruction::MovI2R {
        //    val: 60,
        //    dest: Register::Rax,
        //});
        let val = self.gen_expr(stmt.exit_code);
        self.add_ins(Self::mov_ins(Operand::Immediate(Immediate::Int64(60)), Operand::Register(Register::R0)));
        self.add_ins(Self::mov_ins(val, Operand::Register(Register::R5)));
        self.add_ins(Instruction::Syscall);
        dbg!(&self.stream_ref().instructions);
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
            Literal::Char(expr) => Operand::Immediate(Immediate::Int32(expr as i32)),
            Literal::Float32(_) => todo!(),
            Literal::Float64(_) => todo!(),
            Literal::Bool(expr) => Operand::Immediate(Immediate::Int32(if expr { 1 } else { 0 })),
            Literal::Int8(expr) => Operand::Immediate(Immediate::Int64(expr as i64)),
            Literal::Int16(expr) => Operand::Immediate(Immediate::Int64(expr as i64)),
            Literal::Int32(expr) => Operand::Immediate(Immediate::Int64(expr as i64)),
            Literal::Int64(expr) => Operand::Immediate(Immediate::Int64(expr)),
            Literal::Array(_, _) => todo!(),
            Literal::Vector(_) => todo!(),
        }
    }

    fn mov_ins(val: Operand, dest: Operand) -> Instruction<'c> {
        match (val, dest) {
            (Operand::Register(val), Operand::Register(dest)) => Instruction::MovR2R { val, dest },
            (Operand::Immediate(val), Operand::Register(dest)) => Instruction::MovI2R { val, dest },
            (val, dest) => todo!("Move for: {val:?}, {dest:?}")
        }
    }
    
    fn gen_call_stmt(&mut self, stmt: hir::CallExpr<'c>) {
        self.add_ins(Instruction::Call { func: stmt.name });
    }
    
    fn gen_function_stmt(&mut self, stmt: hir::FuncStmt<'c>) {
        let ins_index = self.stream_ref().instructions.len();
        for stmt in stmt.block.stmts {
            self.gen_stmt(stmt);
        }
        self.add_label(stmt.name.ident, ins_index);
    }
}
