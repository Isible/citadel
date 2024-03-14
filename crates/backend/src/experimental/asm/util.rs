use std::{fs::File, io::Write, path::PathBuf};

use citadel_frontend::ir::IRStmt;

use crate::experimental::asm::{codegen::CodeGenerator, elements::AsmElement};

pub fn compile_program(input: Vec<IRStmt>) -> Vec<AsmElement> {
    let mut compiler = CodeGenerator::new();
    for stmt in input {
        compiler.compile_stmt(&stmt);
    }
    compiler.out
}

pub fn compiler_output(asm: Vec<AsmElement>, location: PathBuf) {
    let buf: Vec<String> = asm.iter().map(|elem| elem.to_string()).collect();
    let mut file = File::create(&location).unwrap_or_else(|err| {
        panic!(
            "Failed to create a new file at {}, error: {}",
            location.display(),
            err
        )
    });
    file.write_all(buf.join("\n").as_bytes())
        .expect("Failed to write to file");
}
