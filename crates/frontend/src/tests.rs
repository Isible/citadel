#[cfg(test)]
mod tests {
    use crate::hir::{
        self, irgen::IRGenerator, DeclFuncStmt, IRExpr, IRStmt, IRTypedIdent, LabelStmt, Literal,
        ReturnStmt, INT8_T,
    };

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::default();

        // abstract function
        let binding = IRStmt::DeclaredFunction(DeclFuncStmt {
            name: IRTypedIdent {
                ident: "myFuncName",
                _type: hir::Type::Ident("void"),
            },
            args: Vec::new(),
        });
        code_gen.gen_ir(binding);

        dbg!("Generated IR: {:#?}", code_gen.stream());
    }

    #[test]
    fn test_ir_to_string() {
        let mut code_gen = IRGenerator::default();

        let ir_stream = [
            IRStmt::Label(LabelStmt { name: "myLabel" }),
            IRStmt::Return(ReturnStmt {
                ret_val: IRExpr::Literal(
                    Literal::String("test".into()),
                    hir::Type::Array(&hir::Type::Ident(INT8_T), 4),
                ),
            }),
        ];

        for ir_stmt in ir_stream {
            code_gen.gen_ir(ir_stmt);
        }

        println!("{}", code_gen.stream_ref().to_string());

        assert_eq!(
            code_gen.stream().to_string(),
            "'myLabel: {\n    ret l{\"test\"}\n}"
        )
    }
}
