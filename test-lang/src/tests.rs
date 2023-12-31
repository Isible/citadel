#[cfg(test)]
mod test {
    use crate::{parser::Parser, tokens::Token, util, compiler::Compiler};

    #[test]
    fn test_lexer() {
        let mut lexer = util::get_lexer_for_file("tests/lexer-test.tl");

        let expected = vec![
            Token::Fn,
            Token::Ident(String::from("main")),
            Token::LParent,
            Token::RParent,
            Token::Colon,
            Token::Ident(String::from("void")),
            Token::LCurly,
            Token::Let,
            Token::Ident(String::from("x")),
            Token::Assign,
            Token::String(String::from("James")),
            Token::Semicolon,
            Token::Ident(String::from("puts")),
            Token::LParent,
            Token::String(String::from("Hello, Mr.")),
            Token::RParent,
            Token::Semicolon,
            Token::Ident(String::from("puts")),
            Token::LParent,
            Token::Ident(String::from("x")),
            Token::RParent,
            Token::Semicolon,
            Token::RCurly,
            Token::Eof,
        ];

        let mut index: usize = 0;

        loop {
            let tok = lexer.tokenize();
            match tok {
                Token::Comment(_) => (),
                _ => {
                    assert_eq!(expected.get(index), Some(&tok));
                    index += 1;
                }
            }
            if tok == Token::Eof {
                break;
            }

            println!("{:?}", tok);
        }
    }

    #[test]
    fn test_parser() {
        let mut lexer = util::get_lexer_for_file("tests/parser-test.tl");
        let mut parser = Parser::new(&mut lexer);

        println!("{:?}", parser.parse_stmt());
    }

    #[test]
    fn test_compiler() {
        let mut lexer = util::get_lexer_for_file("tests/compiler-test.tl");
        let mut parser = Parser::new(&mut lexer);
        let mut compiler = Compiler::new(&mut parser).expect("Failed to compile program because file was empty");
        compiler.compile_program();
        util::compiler_output(&compiler, "tests/build/test.cir");
    }
}
