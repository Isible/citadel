//! Parser for parsing citadel source IR into an AST

mod lexer;
mod tokens;
mod parser;
mod utils;

use std::{fs, io, str, path::Path};

use citadel_frontend::ir::irgen::IRStream;
pub use lexer::Lexer as IRLexer;
pub use parser::Parser as IRParser;
