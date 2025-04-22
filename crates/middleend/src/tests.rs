#[cfg(test)]
mod tests {
    macro_rules! test_optimization {
        ($name:ident,$in:ty,$out:ty,$ret_val:expr) => {
            struct $name;

            impl<'opt> Optimization for $name {
                type InputIR = $in;

                type OutputIR = $out;

                fn name(&self) -> &str {
                    unimplemented!("Test optimizations should never be used for an actual compiler. Affected optimization: {}", stringify!($name))
                }

                fn optimize(&self, input: Self::InputIR) -> Self::OutputIR {
                    $ret_val(input)
                }
            }
        };
    }

    use crate::{api::Optimization, optimize};

    test_optimization!(IntToBool, Vec<i32>, Vec<bool>, |input: Vec<i32>| input
        .iter()
        .map(|i| if *i == 0 { false } else { true })
        .collect());
    test_optimization!(BoolToStr, Vec<bool>, Vec<String>, |input: Vec<bool>| input
        .iter()
        .map(|b| if *b { "1" } else { "0" }.to_string())
        .collect());
    test_optimization!(StrToInt, Vec<String>, Vec<i32>, |input: Vec<String>| input
        .iter()
        .map(|str| str.parse().unwrap())
        .collect());

    #[test]
    fn test_optimize_macro() {
        let stream = vec![0, 1, 1, 1, 0];
        let optimized_stream = optimize!(stream.clone(), IntToBool, BoolToStr, StrToInt);
        assert_eq!(optimized_stream, stream);
    }
}
