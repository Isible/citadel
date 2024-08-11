pub mod codegen;

use std::{fs::File, io::Write, path::PathBuf};

use citadel_frontend::hir::{irgen::HIRStream, IRStmt};

use crate::{
    api::Target,
    asm::{
        codegen::CodeGenerator,
        elements::{AsmElement, BuiltinFunction},
    },
};

use super::elements::{Declaration, Directive, DirectiveType, Operand};

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
fn add_data_section(data: Vec<Declaration>, _type: DirectiveType, out: &mut Vec<AsmElement>) {
    if !data.is_empty() {
        out.insert(0, AsmElement::Directive(Directive { _type }));
        for (i, decl) in data.into_iter().enumerate() {
            out.insert(i + 1, AsmElement::Declaration(decl));
        }
    }
}

fn gen_defined_functions(codegen: &mut CodeGenerator) {
    for func in codegen.defined_functions.clone() {
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

pub fn format(asm: &[AsmElement]) -> String{
    let mut out = String::new();
    for elem in asm{
        match elem {
            AsmElement::Directive(_) | AsmElement::Label(_) => (),
            _ => out.push_str("    "),
        }
        out.push_str(&elem.to_string());
        out.push('\n');
    }
    out
}

pub fn op_vec_to_string(vec: &[Operand]) -> String {
    let str: String = vec
        .iter()
        .map(|op| {
            let mut str = op.to_string();
            str.push(',');
            str
        })
        .collect();
    if !vec.is_empty() {
        (&str[..str.len() - 1]).into()
    } else {
        String::new()
    }
}

/*
pub fn decl_vec_to_string(vec: &Vec<Declaration>) -> String {
    let str: String = vec
        .iter()
        .map(|decl| {
            let mut str = decl.to_string();
            str.push('\n');
            str
        })
        .collect();
    (&str[..str.len() - 1]).into()
}
*/
