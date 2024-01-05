#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            AbstFuncStmt, BlockStmt, IRExpr, LabelStmt, Literal, ReturnStmt, IRStmt,
            IRTypedIdent,
        },
        ir_gen::IRGenerator,
    };

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::new();

        // abstract function
        code_gen.gen_ir(IRStmt::AbstractFunction(AbstFuncStmt {
            name: IRTypedIdent {
                ident: "myFuncName".into(),
                _type: "void".into(),
            },
            args: Vec::new(),
            is_local: false,
        }));

        dbg!("Generated IR: {:#?}", code_gen.get_stream());
    }

    #[test]
    fn test_ir_to_string() {
        let mut code_gen = IRGenerator::new();

        code_gen.gen_ir(IRStmt::Label(LabelStmt {
            name: "ballz".into(),
            block: BlockStmt {
                stmts: vec![IRStmt::Return(ReturnStmt {
                    ret_val: IRExpr::Literal(Literal::String("deez".into())),
                })],
            },
        }));

        println!("{}", code_gen.as_string());

        assert_eq!(code_gen.as_string(), "'ballz: {\nret l{\"deez\"}\n}")
    }
}
