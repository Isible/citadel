/* The Generator for converting source code to an IR AST
 * This will generate the ir and push it to a stream that
 * the represents the AST.
 */

use crate::ir::IRStmt;

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

    pub fn get_stream_ref(&self) -> &Vec<IRStmt> {
        &self.ast
    }

    pub fn get_stream(self) -> Vec<IRStmt> {
        self.ast
    }

    pub fn as_string(&self) -> String {
        self.ast.iter().map(|stmt| stmt.to_string()).collect()
    }
}
