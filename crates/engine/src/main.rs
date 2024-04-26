use std::{fs, io, str, path::PathBuf};

use citadel_api::backend::experimental::asm::{AsmBackend, TargetX86_64};
use citadel_api::compile;
use citadel_irparser::{IRLexer, IRParser};
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
    let lexer = IRLexer::new(str::from_utf8(&file_content).unwrap());
    let mut parser = IRParser::new(&lexer);
    let ir_stream = parser.parse_program();
    dbg!(&ir_stream);
    let mut path = args.file;
    path.set_extension(".asm");
    compile!(AsmBackend::new(TargetX86_64), ir_stream).to_file(path)?;
    Ok(())
}
