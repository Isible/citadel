pub trait Optimization {
    type InputIR;
    type OutputIR;

    fn name(&self) -> &str;

    fn optimize(&self, input: Self::InputIR) -> Self::OutputIR;
}
