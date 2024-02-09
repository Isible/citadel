use super::elements::AsmElement;

#[derive(Debug, Default)]
pub struct CodeGenerator {
    out: Vec<AsmElement>,
}

impl CodeGenerator {
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
        self.out.iter().map(|elem| elem.to_string()).collect()
    }
}
