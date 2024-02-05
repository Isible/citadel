use super::elements::AsmElement;

pub struct CodeGenerator {
    out: Vec<AsmElement>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { out: Vec::new() }
    }

    pub fn generate(&mut self, elem: AsmElement) {
        self.out.push(elem);
    }

    pub fn get_out_ref(&self) -> &Vec<AsmElement> {
        &self.out
    }

    pub fn get_stream(self) -> Vec<AsmElement> {
        self.out
    }

    pub fn as_string(&self) -> String {
        let mut lit_stream = Vec::new();
        self.out.iter().for_each(|elem| {
            lit_stream.push(elem.to_string());
            lit_stream.push("\n".into());
        });
        lit_stream.join("")
    }
}
