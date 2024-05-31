use std::{fs::File, io::Write, path::PathBuf};

use citadel_frontend::ir::{irgen::HIRStream, IRStmt};

use crate::experimental::{
    api::Target,
    asm::{codegen::CodeGenerator, elements::{AsmElement, BuiltinFunction}},
};

use super::elements::{Directive, DirectiveType};

pub fn compile_program(input: HIRStream, _target: impl Target) -> Vec<AsmElement> {
    let mut codegen = CodeGenerator::new(input.types);

    gen_code(&input.stream, &mut codegen);

    gen_defined_functions(&mut codegen);

    let rodata = codegen.rodata;
    let data = codegen.data;
    let mut out = codegen.out;

    // Add data sections
    add_data_section(rodata, DirectiveType::Rodata, &mut out);
    add_data_section(data, DirectiveType::Data, &mut out);

    out
}

fn gen_code<'c>(input: &'c [IRStmt<'c>], codegen: &mut CodeGenerator<'c>) {
    for stmt in input.iter() {
        codegen.gen_stmt(stmt);
    }
}

// TODO: Optimize insertion at front
fn add_data_section(data: Vec<super::elements::Declaration>, _type: DirectiveType, out: &mut Vec<AsmElement>) {
    if !data.is_empty() {
        out.insert(
            0,
            AsmElement::Directive(Directive {
                _type,
            }),
        );
        for (i, decl) in data.into_iter().enumerate() {
            out.insert(i + 1, AsmElement::Declaration(decl));
        }
    }
}

fn gen_defined_functions(codegen: &mut CodeGenerator) {
    for func in codegen.defined_functions.clone() {
        dbg!("Generating function");
        func.generate(codegen);
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
