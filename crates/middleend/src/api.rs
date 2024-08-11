#[macro_export]
macro_rules! optimize {
    ($stream:expr,$($opt:expr),*) => {{
        let stream = $stream;
        $(
            let stream = $opt.optimize(stream);
        )*
        stream
    }};
}

pub trait Optimization<'opt> {
    type InputIR;
    type OutputIR;

    fn stage_name(&self) -> &str;

    fn optimize(&self, input: Self::InputIR) -> Self::OutputIR;
}
