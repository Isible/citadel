#[cfg(test)]
mod tests {
    use clutils::literal::LiteralString;

    use crate::{tokens::Token, ir_gen::IRGenerator};

    #[test]
    fn test_from_lit() {
        let ttype = (
            Token::from_literal("i32".to_owned()).expect("Failed to get tokentype from literal"),
            Token::from_literal("f32".to_owned()).expect("Failed to get tokentype from literal"),
            Token::from_literal("abst".to_owned()).expect("Failed to get tokentype from literal"),
            Token::from_literal("l{\"test\"}".to_owned())
                .expect("Failed to get tokentype from literal"),
            Token::from_literal("TestIdent".to_owned())
                .expect("Failed to get tokentype from literal"),
        );
        assert_eq!(ttype.0, Token::Int(32));
        assert_eq!(ttype.1, Token::Float(32));
        assert_eq!(ttype.2, Token::Abst);
        assert_eq!(ttype.3, Token::Lit(String::from("\"test\"")));
        assert_eq!(ttype.4, Token::Ident(String::from("TestIdent")));
    }

    #[test]
    fn test_to_lit() {
        let lits = (
            String::from("§"),
            String::from("i32"),
            String::from("l{\"test\"}"),
            String::from("TestIdent"),
        );
        assert_eq!(lits.0, Token::Section.literal());
        assert_eq!(lits.1, Token::Int(32).literal());
        assert_eq!(lits.2, Token::Lit("\"test\"".to_owned()).literal());
        assert_eq!(lits.3, Token::Ident("TestIdent".to_owned()).literal());
    }

    #[test]
    fn test_ir_gen() {
        let mut code_gen = IRGenerator::new();

        // abstract function
        code_gen.gen_ir(Token::Abst);
        // function declaration
        code_gen.gen_ir(Token::At);
        // function name
        code_gen.gen_ir(Token::Ident(String::from("myFuncName")));
        // return type
        code_gen.gen_ir(Token::Int(8));

        dbg!("Generated IR: {:#?}", code_gen.get_stream());

        assert_eq!(code_gen.get_stream(), &vec![Token::Abst, Token::At, Token::Ident(String::from("myFuncName")), Token::Int(8)])
    }

    #[test]
    fn test_ir_to_lit() {
        let mut code_gen = IRGenerator::new();

        code_gen.gen_ir(Token::DollarSign);
        code_gen.gen_ir(Token::Ident(String::from("myVarName")));
        code_gen.gen_ir(Token::Lcl);
        code_gen.gen_ir(Token::Int(32));
        code_gen.gen_ir(Token::Assign);
        code_gen.gen_ir(Token::Lit(String::from("1008")));

        dbg!("IR: {:#?}", code_gen.as_string());

        assert_eq!(code_gen.as_string(), String::from("$ myVarName lcl i32 = l{1008}"));
    }
}
