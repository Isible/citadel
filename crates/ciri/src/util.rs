use std::{env::args, path::PathBuf};

pub fn get_file_by_arg(default: PathBuf) -> PathBuf {
    let args = args().collect::<Vec<String>>();
    if let Some(arg) = args.get(1) {
        PathBuf::from(arg)
    } else {
        default
    }
}
