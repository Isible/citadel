use std::fmt::Debug;

use frontend::ir::IRStmt;

pub trait Target: Debug + Default {
    fn name(&self) -> &str;
}

pub trait Backend: Debug + Default {
    type Output: Debug;
    type Target: self::Target + Default;

    // TOOD: decide if ir_stream should be a reference or not
    fn compile(&mut self, ir_stream: &Vec<IRStmt>) -> Self::Output;

    fn target(&self) -> Self::Target {
        Self::Target::default()
    }
}