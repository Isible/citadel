//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain


macro_rules! compile {
    ($backend:ty, $target:ty, $ir_stream:expr) => {{
        let backend = <$backend>::default();
        let target = <$target>::default();

        backend.generate($ir_stream)
    }};
}

#[cfg(test)]
mod tests {
    use citadel_backend::experimental::{api::Backend, asm::{AsmBackend, AsmTarget}};

    #[test]
    fn test() {
        let ir_stream = vec![];
        let out = compile!(AsmBackend, AsmTarget, ir_stream);
    }
}