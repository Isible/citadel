//! This is the api for implementing a new backend
//! or compiling your IR.<br>
//! This api is still unstable, which is why it will
//! reside in the experimental module until it is
//! stabelized.

use std::fmt::Debug;

use citadel_frontend::ir::IRStmt;

pub trait Target: Debug + Default {
    fn name(&self) -> &str;
}

pub trait Backend: Debug + Default {
    type Output: Debug;
    type Target: self::Target + Default;

    fn generate(&self, ir_stream: Vec<IRStmt>) -> Self::Output;

    fn target(&self) -> Self::Target {
        Self::Target::default()
    }
}