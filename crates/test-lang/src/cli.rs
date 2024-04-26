use clap::Parser;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct Args {
    #[clap(help = "Path to the input file")]
    pub(super) input_file_path: PathBuf,

    #[clap(short, help = "Path to the output file")]
    pub(super) output_path: Option<PathBuf>,

    #[clap(long, help = "Output the ir", default_value = "false")]
    pub(super) chir: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self::parse()
    }
}
