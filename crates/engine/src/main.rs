use std::{fs, path::PathBuf};

use citadel_backend::experimental::{api::Backend, asm::AsmBackend};
use citadel_irparser::{lexer::Lexer, parser};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn main() {
    run();
}

fn run() {
    let args = Args::parse();
    let mut lexer = Lexer::new(&args.file).expect("Failed to find file");
    let mut parser = parser::Parser::new(&mut lexer);
    let ir_stream = parser.parse_program();
    let backend = AsmBackend::default();
    let asm = backend.generate(ir_stream);
    let mut path = args.file.to_str().unwrap()[..args.file.to_str().unwrap().len() - 4].to_string();
    path.push('s');
    let buf: Vec<String> = asm.iter().map(|elem| elem.to_string()).collect();
    fs::write(path, buf.join("\n")).expect("Failed to write to file");
}
