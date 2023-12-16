#[cfg(test)]
mod test {
    use clutils::literal::LiteralString;

    use crate::{tokens::Token, util, parser::Parser};

    #[test]
    fn test_lexer() {
        let mut lexer = util::get_lexer_for_file("tests/lexer-test.tl");

        /*
        let expected = vec![
            Token::Fn,
            Token::Ident(String::from("main")),
            Token::LParent,
            Token::RParent,
            Token::LCurly,
            Token::Let,
            Token::Ident(String::from("x")),
            Token::Assign,
            Token::String(String::from("James")),
            Token::Semicolon,
            Token::Ident(String::from("print")),
            Token::LParent,
            Token::String(String::from("Hello, Mr.")),
            Token::RParent,
            Token::Semicolon,
            Token::Ident(String::from("print")),
            Token::LParent,
            Token::Ident(String::from("x")),
            Token::RParent,
            Token::Semicolon,
            Token::RCurly,
            Token::Eof,
        ];
        */

        let mut index: usize = 0;

        loop {
            let tok = lexer.tokenize();
            // assert_eq!(expected.get(index), Some(&tok));
            if tok == Token::Eof {
                break;
            }

            println!("{:?}", tok);
            index += 1;
        }
    }

    #[test]
    fn test_parser() {
        let mut lexer = util::get_lexer_for_file("tests/parser-test.tl");
        let mut parser = Parser::new(&mut lexer);

        println!("{:?}", parser.parse_stmt());
        println!("{:?}", parser.parse_stmt());
    }
}
