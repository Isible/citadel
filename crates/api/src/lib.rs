//! Greetings traveler, this is the nonexistent api wrapper for the citadel toolchain

use std::{fs, io, marker::PhantomData, path::PathBuf};

use citadel_backend::experimental::api::{Backend, Target};

#[macro_export]
macro_rules! compile {
    ($backend:expr, $ir_stream:expr) => {{
        use citadel_api::Output;
        use citadel_backend::experimental::api::Backend;
        use citadel_frontend::api::IRCompiler;

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
            .collect();
        fs::write(path, contents)
    }
}
