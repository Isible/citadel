//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain

#[macro_export]
macro_rules! compile {
    ($backend:ty, $target:ty, $compiler:ty, $ast:expr) => {{
        use citadel_frontend::api::IRCompiler;
        use citadel_backend::experimental::api::Backend;

        let backend = <$backend>::default();
        let target = <$target>::default();

        let ir_stream = <$compiler>::default().gen_ir($ast);

        backend.generate(ir_stream)
    }};
}