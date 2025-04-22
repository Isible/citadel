mod codegen;

use bumpalo::Bump;
use citadel_frontend::hir::irgen::HIRStream;

use crate::api::Optimization;

use super::irgen::{LIRGenerator, LIRStream};

pub struct ClirOptimization<'opt> {
    pub arena: &'opt Bump,
}

impl<'opt> ClirOptimization<'opt> {
    pub fn new(arena: &'opt Bump) -> Self {
        Self {
            arena,
        }
    }
}

impl<'clir> Optimization for ClirOptimization<'clir> {
    type InputIR = HIRStream<'clir>;

    type OutputIR = LIRStream<'clir>;

    fn name(&self) -> &str {
        "HIR to LIR optimization"
    }

    fn optimize(&self, input: Self::InputIR) -> Self::OutputIR {
        let mut gen = LIRGenerator::new(self.arena, input.types);
        gen.generate(input.stream);
        gen.stream()
    }
}
