#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            AbstFuncStmt, BlockStmt, Expression, LabelStmt, Literal, ReturnStmt, Statement,
            TypedIdent,
        },
        ir_gen::IRGenerator,
    };

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::new();

        // abstract function
        code_gen.gen_ir(Statement::AbstractFunction(AbstFuncStmt {
            name: TypedIdent {
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

        code_gen.gen_ir(Statement::Label(LabelStmt {
            name: "ballz".into(),
            block: BlockStmt {
                stmts: vec![Statement::Return(ReturnStmt {
                    ret_val: Expression::Literal(Literal::String("deez".into())),
                })],
            },
        }));

        println!("{}", code_gen.as_string());

        assert_eq!(code_gen.as_string(), "'ballz: {\nret l{\"deez\"}\n}")
    }
}
