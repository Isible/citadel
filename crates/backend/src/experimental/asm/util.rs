use std::{collections::{HashSet, VecDeque}, fs::File, io::Write, path::PathBuf};

use citadel_frontend::ir::IRStmt;

use crate::experimental::{
    api::Target,
    asm::{codegen::CodeGenerator, elements::AsmElement},
};

use super::elements::{Directive, DirectiveType, StdFunction};

pub fn compile_program(input: Vec<IRStmt>, _target: impl Target) -> Vec<AsmElement> {
    let mut codegen = CodeGenerator::default();

    gen_code(&input, &mut codegen);

    let data = codegen.rodata;
    let defined_functions = codegen.defined_functions;
    let mut out = codegen.out;

    dbg!(&defined_functions);

    // Add data section
    add_data_section(data, &mut out);

    gen_defined_functions(defined_functions, &mut out);

    out
}

fn gen_code<'c>(input: &'c Vec<IRStmt>, codegen: &mut CodeGenerator<'c>) {
    for stmt in input.iter() {
        codegen.gen_stmt(&stmt);
    }
}

fn add_data_section(data: Vec<super::elements::Declaration>, out: &mut Vec<AsmElement>) {
    if !data.is_empty() {
        out.insert(
            0,
            AsmElement::Directive(Directive {
                _type: DirectiveType::Rodata,
            }),
        );
        for (i, decl) in data.into_iter().enumerate() {
            out.insert(i+1, AsmElement::Declaration(decl));
        }
    }
}

fn gen_defined_functions(defined_functions: HashSet<StdFunction>, out: &mut Vec<AsmElement>) {
    for func in defined_functions {
        dbg!("Generating function");
        out.extend(func.generate());
    }
}

pub fn compiler_output(asm: String, location: PathBuf) {
    let mut file = File::create(&location).unwrap_or_else(|err| {
        panic!(
            "Failed to create a new file at {}, error: {}",
            location.display(),
            err
        )
    });
    file.write_all(asm.as_bytes())
        .expect("Failed to write to file");
}

pub fn format(asm: Vec<AsmElement>) -> String {
    let mut out = String::new();
    for elem in asm {
        match elem {
            AsmElement::Directive(_) | AsmElement::Label(_) => (),
            _ => out.push_str("    "),
        }
        out.push_str(&elem.to_string());
        out.push('\n');
    }
    out
}
