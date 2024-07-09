//! The Generator for converting source code to an IR AST
//! This will generate the ir and push it to a stream that
//! the represents the AST. You can implement this yourself
//! if you don't want to use the provided generator or need
//! specific capabilities

use std::{collections::HashMap, fmt::Display};

use crate::{ir::IRStmt, util::CompositeDataType};

use super::{IRTypedIdent, Ident};

pub type TypeTable<'t> = HashMap<Ident<'t>, (CompositeDataType, Vec<IRTypedIdent<'t>>)>;

#[derive(Default)]
pub struct IRGenerator<'s> {
    ir: HIRStream<'s>,
}

/// High-level IR stream that contains the ir stream
/// as well as a typetable. This is output by the [IRGenerator]
/// and used as an input for various optimizations.
#[derive(Debug, Default)]
pub struct HIRStream<'hir> {
    pub stream: Vec<IRStmt<'hir>>,
    pub types: TypeTable<'hir>,
}

impl<'g> IRGenerator<'g> {
    pub fn gen_ir(&mut self, node: IRStmt<'g>) {
        match &node {
            IRStmt::Struct(node) => {
                self.ir
                    .types
                    .insert(node.name, (CompositeDataType::Struct, node.fields.clone()));
            }
            IRStmt::Union(node) => {
                self.ir
                    .types
                    .insert(node.name, (CompositeDataType::Union, node.variants.clone()));
            }
            _ => (),
        }
        self.ir.stream.push(node);
    }

    pub fn mut_stream_ref(&mut self) -> &mut HIRStream<'g> {
        &mut self.ir
    }

    pub fn stream_ref(&self) -> &HIRStream<'g> {
        &self.ir
    }

    pub fn stream(self) -> HIRStream<'g> {
        self.ir
    }
}

impl<'hir> HIRStream<'hir> {
    pub fn stream_ref(&self) -> &Vec<IRStmt<'hir>> {
        &self.stream
    }

    pub fn mut_stream_ref(&mut self) -> &mut Vec<IRStmt<'hir>> {
        &mut self.stream
    }
}

impl Display for HIRStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .stream
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
