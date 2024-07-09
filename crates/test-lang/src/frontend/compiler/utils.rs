use citadel_api::frontend::ir::{self, IRTypedIdent};

use crate::frontend::ast::Type;

use super::TypedIdent;

pub fn compile_typed_ident<'c>(typed_ident: TypedIdent<'c>) -> IRTypedIdent<'c> {
    IRTypedIdent {
        ident: typed_ident.ident,
        _type: compile_type(typed_ident._type),
    }
}

pub fn compile_typed_idents<'c>(typed_idents: &[TypedIdent<'c>]) -> Vec<IRTypedIdent<'c>> {
    let mut ir_typed_idents = Vec::new();
    for typed_ident in typed_idents {
        ir_typed_idents.push(compile_typed_ident(*typed_ident))
    }
    ir_typed_idents
}

pub fn compile_type<'c>(_type: Type<'c>) -> ir::Type<'c> {
    match _type {
        Type::Ident(id) => ir::Type::Ident(id),
        Type::Array(_, _) => todo!(),
    }
}

#[macro_export]
macro_rules! no_ctx {
    ($expr:expr) => {{
        ($expr, None)
    }};
}
