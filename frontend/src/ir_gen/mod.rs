/* The Generator for converting source code to an IR AST
 * This will generate the ir and push it to a stream that
 * the represents the AST.
 */

use crate::ast::IRStmt;

pub struct IRGenerator {
    ast: Vec<IRStmt>,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self { ast: Vec::new() }
    }

    pub fn gen_ir(&mut self, node: IRStmt) {
        self.ast.push(node);
    }

    pub fn get_stream(&self) -> &Vec<IRStmt> {
        &self.ast
    }

    pub fn as_string(&self) -> String {
        let mut lit_stream = Vec::new();
        self.ast.iter().for_each(|stmt| {
            lit_stream.push(stmt.to_string());
            lit_stream.push("\n".into());
        });
        lit_stream.join("")
    }
}
