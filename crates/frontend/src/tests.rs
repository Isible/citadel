#[cfg(test)]
mod tests {
    use crate::ir::{
        self, irgen::IRGenerator, BlockStmt, DeclFuncStmt, IRExpr, IRStmt, IRTypedIdent, Ident, LabelStmt, Literal, ReturnStmt
    };

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::default();

        // abstract function
        let binding = IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent {
                ident: Ident("myFuncName"),
                _type: ir::Type::Ident(Ident("void")),
            },
            args: Vec::new(),
        });
        code_gen.gen_ir(binding);

        dbg!("Generated IR: {:#?}", code_gen.stream());
    }

    #[test]
    fn test_ir_to_string() {
        let mut code_gen = IRGenerator::default();

        let binding = IRStmt::Label(LabelStmt {
            name: Ident("myLabel"),
            block: BlockStmt {
                stmts: vec![IRStmt::Return(ReturnStmt {
                    ret_val: IRExpr::Literal(Literal::String("test".into()), ir::Type::Array(&ir::Type::Ident(ir::Ident("i8")), 4)),
                })],
            },
        });
        code_gen.gen_ir(binding);

        println!("{}", code_gen.stream_ref().to_string());

        assert_eq!(
            code_gen.stream().to_string(),
            "'myLabel: {\n    ret l{\"test\"}\n}"
        )
    }
}
