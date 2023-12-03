#[cfg(test)]
mod tests {
    use clutils::literal::LiteralString;

    use crate::tokens::Token;

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
}
