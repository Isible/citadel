//! The Generator for converting source code to an IR AST
//! This will generate the ir and push it to a stream that
//! the represents the AST. You can implement this yourself
//! if you don't want to use the provided generator or need
//! specific capabilities

use std::collections::HashMap;

use crate::{ir::IRStmt, util::CompositeDataType};

use super::{IRTypedIdent, Ident};

pub type TypeTable<'t> = HashMap<Ident<'t>, (CompositeDataType, Vec<IRTypedIdent<'t>>)>;

#[derive(Default)]
pub struct IRGenerator<'s> {
    ir: IRStream<'s>,
}

#[derive(Debug, Default)]
pub struct IRStream<'s> {
    pub stream: Vec<IRStmt<'s>>,
    pub types: TypeTable<'s>,
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

    pub fn stream_ref(&self) -> &IRStream<'g> {
        &self.ir
    }

    pub fn stream(self) -> IRStream<'g> {
        self.ir
    }
}

impl IRStream<'_> {
    pub fn as_string(&self) -> String {
        self.stream
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
