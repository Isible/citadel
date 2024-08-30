//! Implements the default optimization for lowering the intermediary
//! representation from high- to low-level

use citadel_frontend::hir::irgen::HIRStream;

use crate::api::Optimization;

use super::irgen::LIRStream;

pub struct LowerIR;

impl<'opt> Optimization<'opt> for LowerIR {
    type InputIR = HIRStream<'opt>;

    type OutputIR = LIRStream;

    fn stage_name(&self) -> &str {
        "lower_ir"
    }

    fn optimize(&self, input: Self::InputIR) -> Self::OutputIR {
        todo!()
    }
}
