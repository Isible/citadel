//! Api for implementing a new backend
//! This is not exlusively for assembly backends,
//! but will reside in the experimental module until
//! it's design is finalized

use std::fmt::Debug;

use frontend::ir::IRStmt;

pub trait Target: Debug + Default {
    fn name(&self) -> &str;
}

pub trait Backend: Debug + Default {
    type Output: Debug;
    type Target: self::Target + Default;

    fn compile(&mut self, ir_stream: Vec<IRStmt>) -> Self::Output;

    fn target(&self) -> Self::Target {
        Self::Target::default()
    }
}