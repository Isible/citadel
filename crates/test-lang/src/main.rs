mod cli;

use std::io;

use cli::Args;

fn main() -> io::Result<()> {
    let args = Args::default();

    if args.chir {
        test_lang::compile_chir(args.input_file_path, args.output_path)
    } else {
        test_lang::compile_asm(args.input_file_path, args.output_path)
    }
}