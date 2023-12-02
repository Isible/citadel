#[cfg(test)]
mod tests {
    use crate::tokens::TokenType;

    #[test]
    fn test_bit_types() {
        let ttype = (
            TokenType::from_literal("i32".to_owned())
                .expect("Failed to get tokentype from literal"),
            TokenType::from_literal("f32".to_owned())
                .expect("Failed to get tokentype from literal"),
        );
        assert_eq!(ttype.0, TokenType::Int(32));
        assert_eq!(ttype.1, TokenType::Float(32));
    }
}
