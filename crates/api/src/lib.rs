//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain

pub use citadel_backend as backend;
pub use citadel_frontend as frontend;
pub use citadel_middleend as middleend;

use std::{fs, io, path::PathBuf};

use citadel_backend::api::Backend;

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

pub struct Output<B>
where
    B: Backend,
{
    pub backend: B,
    pub stream: B::Output,
}

impl<B> Output<B>
where
    B: Backend,
{
    pub fn new(backend: B, stream: B::Output) -> Self {
        Self {
            backend,
            stream,
        }
    }

    pub fn to_file(self, path: PathBuf) -> io::Result<()> {
        if let Some(res) = self.backend.to_file(&self.stream) {
            return res;
        }

        let contents = if let Some(formatted) = self.backend.format(&self.stream) {
            formatted
        } else {
            self.stream
                .into_iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        };

        fs::write(path, contents)
    }
}
