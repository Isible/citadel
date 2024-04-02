#[cfg(test)]
mod tests {
    use crate::ir::{
        irgen::IRGenerator, BlockStmt, DeclFuncStmt, IRExpr, IRStmt, IRTypedIdent, LabelStmt,
        Literal, ReturnStmt,
    };

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::new();

        // abstract function
        code_gen.gen_ir(IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent {
                ident: "myFuncName".into(),
                _type: "void".into(),
            },
            args: Vec::new(),
        }));

        dbg!("Generated IR: {:#?}", code_gen.get_stream());
    }

    #[test]
    fn test_ir_to_string() {
        let mut code_gen = IRGenerator::new();

        code_gen.gen_ir(IRStmt::Label(LabelStmt {
            name: "myLabel".into(),
            block: BlockStmt {
                stmts: vec![IRStmt::Return(ReturnStmt {
                    ret_val: IRExpr::Literal(Literal::String("test".into())),
                })],
            },
        }));

        println!("{}", code_gen.as_string());

        assert_eq!(
            code_gen.as_string(),
            "'myLabel: {\n    ret l{\"test\"}\n}\n"
        )
    }
}
