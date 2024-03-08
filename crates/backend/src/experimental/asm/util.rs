use std::{fs::File, io::Write, path::PathBuf};

use frontend::ir::IRStmt;

use crate::experimental::asm::{compiler::Compiler, elements::AsmElement};

pub fn compile_program(input: Vec<IRStmt>) -> Vec<AsmElement> {
    let mut compiler = Compiler {};
    let mut program = Vec::new();
    program.push(compiler.create_header());
    program.extend(
        input
            .into_iter()
            .map(|stmt| compiler.compile_stmt(&stmt))
            .collect::<Vec<AsmElement>>(),
    );
    program
}

pub fn compiler_output(asm: Vec<AsmElement>, location: PathBuf) {
    let buf: String = asm.iter().map(|elem| elem.to_string()).collect();
    let mut file = File::create(&location).unwrap_or_else(|err| {
        panic!(
            "Failed to create a new file at {}, error: {}",
            location.display(),
            err
        )
    });
    file.write_all(buf.as_bytes())
        .expect("Failed to write to file");
}
