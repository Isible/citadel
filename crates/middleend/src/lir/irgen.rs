use std::fmt::Display;

#[derive(Debug)]
pub struct LIRStream<'lir> {
    pub stream: Vec<super::Instruction>,
    //pub types: TypeTable<'hir>,
}

impl Display for LIRStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .stream
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
