#[cfg(test)]
mod tests {
    macro_rules! test_optimization {
        ($name:ident,$in:ty,$out:ty) => {
            struct $name;

            impl<'opt> Optimization<'opt> for $name {
                type InputIR = $in;

                type OutputIR = $out;

                fn stage_name(&self) -> &str {
                    unimplemented!("Test optimizations should never be used for an actual compiler. Affected optimization: {}", stringify!($name))
                }

                fn optimize(&self, input: Self::InputIR) -> Self::OutputIR {
                    input
                }
            }
        };
        ($name:ident,$in:ty,$out:ty,$ret_val:expr) => {
            struct $name;

            impl<'opt> Optimization<'opt> for $name {
                type InputIR = $in;

                type OutputIR = $out;

                fn stage_name(&self) -> &str {
                    unimplemented!("Test optimizations should never be used for an actual compiler. Affected optimization: {}", stringify!($name))
                }

                fn optimize(&self, _: Self::InputIR) -> Self::OutputIR {
                    $ret_val
                }
            }
        };
    }

    use citadel_frontend::ir::irgen::HIRStream;

    use crate::{experimental::api::Optimization, optimize};

    test_optimization!(Opt1, HIRStream<'opt>, Vec<i32>, vec![]);
    test_optimization!(Opt2, Vec<i32>, HIRStream<'opt>, HIRStream::default());
    test_optimization!(Opt3, HIRStream<'opt>, Vec<String>, vec!["ballz".into()]);

    #[test]
    fn test_macro() {
        let stream = HIRStream::default();
        let stream = optimize!(stream, Opt1, Opt2, Opt3);
        println!("Stream: {stream:?}");
    }
}
