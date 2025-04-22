use std::{collections::HashMap, fmt::Display};

use bumpalo::Bump;
use citadel_frontend::hir;

use super::{DataValue, Instruction};

pub struct LIRGenerator<'s> {
    ir: LIRStream<'s>,
    arena: &'s Bump,

    // in
    types: hir::TypeTable<'s>,
}

/// High-level IR stream that contains the ir stream
/// as well as a typetable. This is output by the [IRGenerator]
/// and used as an input for various optimizations.
#[derive(Debug, Default, Clone)]
pub struct LIRStream<'lir> {
    // data
    pub instructions: Vec<Instruction<'lir>>,
    pub data: Vec<DataValue>,

    // tracking
    pub labels: HashMap<&'lir str, usize>,
    pub entry_size: usize,
}

impl<'g> LIRGenerator<'g> {
    pub fn new(arena: &'g Bump, types: hir::TypeTable<'g>) -> Self {
        Self {
            ir: LIRStream::default(),
            arena,
            types,
        }
    }

    pub fn add_ins(&mut self, ins: Instruction<'g>) {
        self.ir.instructions.push(ins);
    }

    pub fn add_data_value(&mut self, data: DataValue) {
        self.ir.data.push(data);
    }

    pub fn add_label(&mut self, label_name: &'g str, label_index: usize) {
        self.ir.labels.insert(label_name, label_index);
    }

    pub fn set_entry_size(&mut self, size: usize) {
        self.ir.entry_size = size;
    }

    pub fn mut_stream_ref(&mut self) -> &mut LIRStream<'g> {
        &mut self.ir
    }

    pub fn stream_ref(&self) -> &LIRStream<'g> {
        &self.ir
    }

    pub fn stream(self) -> LIRStream<'g> {
        self.ir
    }
}

impl Display for LIRStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .instructions
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
