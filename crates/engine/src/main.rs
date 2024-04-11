use std::{fs, io, path::PathBuf};

use citadel_backend::experimental::{
    api::Backend,
    asm::{AsmBackend, TargetX86_64},
};
use citadel_irparser::{lexer::Lexer, parser};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn main() -> io::Result<()> {
    run()
}

fn run() -> io::Result<()> {
    let args = Args::parse();
    let file_content = fs::read(&args.file)?;
    let mut lexer = Lexer::new(std::str::from_utf8(&file_content).unwrap());
    let mut parser = parser::Parser::new(&mut lexer);
    let ir_stream = parser.parse_program();
    let backend = AsmBackend::new(TargetX86_64);
    let asm = backend.generate(ir_stream);
    let mut path = args.file.to_str().unwrap()[..args.file.to_str().unwrap().len() - 4].to_string();
    path.push('s');
    let buf: Vec<String> = asm.iter().map(|elem| elem.to_string()).collect();
    fs::write(path, buf.join("\n")).expect("Failed to write to file");
    Ok(())
}
