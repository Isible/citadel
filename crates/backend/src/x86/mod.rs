//! The assembly module of the backend containing an
//! experimental/prototype compiler for x86-64 assembly
//! leveraging the [backend api](api/index.html).

pub mod codegen;
pub mod machine;
mod tests;

use std::{collections::HashMap, path::Path};

use bumpalo::Bump;
use citadel_frontend::hir::irgen::HIRStream;
use citadel_middleend::lir::{self, irgen::LIRStream};
use codegen::MachineGenerator;
use machine::{lir_to_machine, Instruction};
use object::write::Object;

use crate::api::{Backend, CompiledDisplay, Target};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TargetX86_64;

impl Target for TargetX86_64 {
    fn name(&self) -> &str {
        "x86-64"
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TargetX86_32;

impl Target for TargetX86_32 {
    fn name(&self) -> &str {
        "x86-32"
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

pub struct MachineStream<'machine> {
    pub instructions: Vec<Instruction<'machine>>,
    pub data: Vec<lir::DataValue>,

    // tracking
    pub labels: HashMap<&'machine str, usize>,
    pub entry_size: usize,
}

pub struct CompileResult<'cr> {
    obj: Object<'cr>,
}

impl<'cr> CompiledDisplay for CompileResult<'cr> {
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

    fn generate(&self, ir_stream: LIRStream<'r>) -> Self::Output {
        let mut gen = MachineGenerator::new(self.target);
        let machine_instructions = lir_to_machine(TargetX86_64, ir_stream.instructions);
        let machine_stream = MachineStream {
            instructions: machine_instructions,
            data: ir_stream.data,
            labels: ir_stream.labels,
            entry_size: ir_stream.entry_size,
        };
        gen.generate(machine_stream);
        CompileResult { obj: gen.obj }
    }

    fn to_file<P>(&self, output: &Self::Output, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        todo!()
    }
}
