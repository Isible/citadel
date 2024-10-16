//! The assembly module of the backend containing an
//! experimental/prototype compiler for x86-64 assembly
//! leveraging the [backend api](api/index.html).

pub mod codegen;
pub mod elements;
mod tests;

use std::{collections::HashMap, path::Path};

use bumpalo::Bump;
use citadel_frontend::hir::irgen::HIRStream;
use codegen::CodeGenerator;
use elements::{DataValue, Instruction};

use crate::api::{Backend, CompiledDisplay, Target};

#[derive(Debug, Default, Clone, Copy)]
pub struct TargetX86_64;

impl Target for TargetX86_64 {
    fn name(&self) -> &str {
        "x86-64"
    }
}

#[derive(Debug)]
pub struct X86Backend<'b, T: Target> {
    target: T,
    arena: &'b Bump,
}

impl<'b, T: Target> X86Backend<'b, T> {
    pub fn new(target: T, arena: &'b Bump) -> Self {
        Self { target, arena }
    }
}

#[derive(Debug)]
pub struct CompileResult<'cr> {
    pub instructions: Vec<Instruction<'cr>>,
    pub data: Vec<DataValue>,

    pub labels: HashMap<&'cr str, usize>,
    pub entry_size: usize,
}

impl CompiledDisplay for CompileResult<'_> {
    fn as_string(&self) -> String {
        todo!()
    }
}

impl<'r, T: Target> Backend<'r> for X86Backend<'r, T> {
    type Output = CompileResult<'r>;
    type Target = T;

    fn target(&self) -> Self::Target {
        self.target
    }

    fn generate(&self, ir_stream: HIRStream<'r>) -> Self::Output {
        let mut gen = CodeGenerator::new(self.arena, ir_stream.types);
        gen.generate(ir_stream.stream);
        CompileResult {
            instructions: gen.instructions,
            data: gen.data,
            labels: gen.labels,
            entry_size: gen.entry_size,
        }
    }

    fn to_file<P>(&self, output: &Self::Output, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        todo!()
    }
}
