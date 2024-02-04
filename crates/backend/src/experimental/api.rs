use std::fmt::Debug;

use frontend::ast::IRStmt;

pub trait Target: Debug {
    fn name(&self) -> &str;
}

pub trait Backend {
    type Output: Debug;
    type Target: self::Target + Default;

    fn compile(&mut self, ir_stream: Vec<IRStmt>) -> Self::Output;

    fn target(&self) -> Self::Target {
        Self::Target::default()
    }
}