use crate::ir::IRStmt;

pub trait IRCompiler {
    type Ast;

    fn gen_ir(&mut self, ast: Self::Ast) -> Vec<IRStmt>;
}