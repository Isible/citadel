#[cfg(test)]
mod tests {
    use crate::{compile_asm, compile_chir};

    #[test]
    fn test_compiler() {
        compile_chir(
            "tests/compiler-test.tl".into(),
            Some("build/chir/compiler-test.chir".into()),
        )
        .unwrap();
    }

    #[test]
    fn test_codegen() {
        compile_asm(
            "tests/codegen-test.tl".into(),
            Some("build/asm/codegen-test.asm".into()),
        )
        .unwrap();
    }
}
