//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain

pub use citadel_backend as backend;
pub use citadel_frontend as frontend;
pub use citadel_middleend as middleend;

use std::{fs, io, path::{Path, PathBuf}};

use citadel_backend::api::{Backend, CompiledDisplay};

#[macro_export]
macro_rules! compile {
    ($backend:expr, $clir_stream:expr) => {{
        use citadel_api::backend::api::Backend;
        use citadel_api::Output;

        Output::new($backend, $backend.generate($clir_stream))
    }};
}

#[macro_export]
macro_rules! optimize {
    ($stream:expr,$($opt:expr),*) => {{
        let stream = $stream;
        $(
            let stream = $opt.optimize(stream);
        )*
        stream
    }};
}

pub struct Output<'o, B>
where
    B: Backend<'o>,
{
    pub backend: B,
    pub stream: B::Output,
}

impl<'o, B> Output<'o, B>
where
    B: Backend<'o>,
{
    pub fn new(backend: B, stream: B::Output) -> Self {
        Self {
            backend,
            stream,
        }
    }

    pub fn to_file<P>(self, path: P) -> io::Result<()>
    where P: AsRef<Path> {
        self.backend.to_file(&self.stream, path)
    }
}
