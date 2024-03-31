use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

use citadel_frontend::ir::IRStmt;

use crate::experimental::asm::{codegen::CodeGenerator, elements::AsmElement};

use super::elements::{Directive, DirectiveType, StdFunction};

pub fn compile_program(input: Vec<IRStmt>) -> Vec<AsmElement> {
    let mut codegen = CodeGenerator::default();

    gen_code(input, &mut codegen);

    let data = codegen.rodata;
    let defined_functions = codegen.defined_functions;
    let mut out = codegen.out;

    dbg!(&defined_functions);

    // Add data section
    add_data_section(data, &mut out);

    gen_defined_functions(defined_functions, &mut out);

    out
}

fn gen_code(input: Vec<IRStmt>, codegen: &mut CodeGenerator) {
    for stmt in input {
        codegen.gen_stmt(&stmt);
    }
}

fn add_data_section(data: Vec<super::elements::Declaration>, out: &mut Vec<AsmElement>) {
    if !data.is_empty() {
        out.insert(0, AsmElement::Directive(Directive {
            _type: DirectiveType::Rodata,
            content: data,
        }));
    }
}

fn gen_defined_functions(defined_functions: HashSet<StdFunction>, out: &mut Vec<AsmElement>) {
    for func in defined_functions {
        dbg!("Generating function");
        out.extend(func.generate());
    }
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
