use std::{env, fs, io, path::PathBuf, str};

use citadel_api::{backend::experimental::asm::{AsmBackend, TargetX86_64}, compile};
use citadel_irparser::{IRLexer, IRParser};

fn main() -> io::Result<()> {
    run()
}

fn run() -> io::Result<()> {
    let mut path = path_from_arg().expect("User needs to specify a path to the file containing the IR");
    let file_content = fs::read(&path)?;
    let lexer = IRLexer::new(str::from_utf8(&file_content).unwrap());
    let mut parser = IRParser::new(&lexer);
    let ir_stream = parser.parse_program();
    dbg!(&ir_stream);
    path.set_extension(".asm");
    compile!(AsmBackend::new(TargetX86_64), ir_stream);
    Ok(())
}

fn path_from_arg() -> Option<PathBuf> {
    let path = env::args().into_iter().nth(1)?;
    Some(path.into())
}