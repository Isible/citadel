#[cfg(test)]
mod test {
    use crate::{parser::Parser, tokens::Token, util, compiler::Compiler};

    #[test]
    fn test_lexer() {
        let mut lexer = util::get_lexer_for_file("tests/lexer-test.tl".into());

        let expected = [
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
    }

    #[test]
    fn test_parser() {
        // TODO: Write proper tests
        let mut lexer = util::get_lexer_for_file("tests/parser-test.tl".into());
        let mut parser = Parser::new(&mut lexer);

        let mut ast = Vec::new();

        loop {
            let stmt = match parser.parse_stmt() {
                Ok(stmt) => stmt,
                Err(_) => break,
            };
            ast.push(stmt);
            parser.next_token();
        }

        dbg!("{}", ast);
    }

    #[test]
    fn test_compiler() {
        // TODO: Write proper tests
        let mut lexer = util::get_lexer_for_file("tests/compiler-test.tl".into());
        let mut parser = Parser::new(&mut lexer);
        let mut compiler = Compiler::new(&mut parser).expect("Failed to compile program because file was empty");
        compiler.compile_program();
        util::compiler_output(&compiler, "tests/build/test.chir".into());
    }
}
