#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use citadel_frontend::ir::IRStmt;
    use citadel_irparser::{lexer::Lexer, parser::Parser};

    use crate::experimental::{api::Backend, asm::{util, AsmBackend}};

    #[test]
    fn test_asm_compiler() {
        let backend = AsmBackend::default();
        let asm_code = backend.generate(gen_ir_stream(&"tests/main.chir".into()));
        util::compiler_output(asm_code, PathBuf::from("tests/out/out.asm"));
    }

    fn gen_ir_stream(path: &PathBuf) -> Vec<IRStmt> {
        let mut lexer = Lexer::new(path.into()).unwrap();
        let mut parser = Parser::new(&mut lexer);
        parser.parse_program()
    }
}
