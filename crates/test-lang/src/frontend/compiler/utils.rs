use citadel_api::frontend::ir::{self, IRTypedIdent};

use crate::frontend::ast::Type;

use super::TypedIdent;

pub fn compile_typed_ident<'c>(typed_ident: TypedIdent<'c>) -> IRTypedIdent<'c> {
    IRTypedIdent { ident: ir::Ident(typed_ident.ident), _type: compile_type(typed_ident._type) }
}

pub fn compile_type<'c>(_type: Type<'c>) -> ir::Type<'c> {
    match _type {
        Type::Ident(id) => ir::Type::Ident(ir::Ident(id)),
        Type::Array(_, _) => todo!(),
    }
}