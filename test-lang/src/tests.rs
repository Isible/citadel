#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use crate::{lexer::Lexer, tokens::Token};

    #[test]
    fn test_lexer() {
        let mut lexer = get_lexer_for_file("tests/test.tl");

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

        let mut index: usize = 0;

        loop {
            let tok = lexer.tokenize();
            assert_eq!(expected.get(index), Some(&tok));
            if tok == Token::Eof {
                break;
            }
            index += 1;
        }
    }

    fn get_lexer_for_file(file_path: &str) -> Lexer {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to read to string");
        Lexer::new(buf)
    }
}