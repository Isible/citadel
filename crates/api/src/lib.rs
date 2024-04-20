//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain

pub use citadel_frontend as frontend;
pub use citadel_middleend as middleend;
pub use citadel_backend as backend;

use std::{fs, io, marker::PhantomData, path::PathBuf};

use citadel_backend::experimental::api::{Backend, Target};

#[macro_export]
macro_rules! compile {
    ($backend:expr, $ir_stream:expr) => {{
        use citadel_api::Output;
        use citadel_api::backend::experimental::api::Backend;

        Output::new($backend, $backend.generate($ir_stream))
    }};
}

pub struct Output<T, B>
where
    T: Target,
    B: Backend<Target = T>,
{
    pub backend: B,
    pub stream: B::Output,
    phantom: PhantomData<T>,
}

impl<T, B> Output<T, B>
where
    T: Target,
    B: Backend<Target = T>,
{
    pub fn new(backend: B, stream: B::Output) -> Self {
        Self {
            backend,
            stream,
            phantom: PhantomData::default(),
        }
    }

    pub fn to_file(self, path: PathBuf) -> io::Result<()> {
        if let Some(res) = self.backend.to_file() {
            return res;
        }
        let contents: String = self
            .stream
            .into_iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(path, contents)
    }
}
