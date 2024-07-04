//! # Citadel - backend
//!
//! The backend crate of the citadel project
//!
//! For information on what exactly citadel is you should visit our [github-repository](https://github.com/Isible/citadel/blob/main/README.md)
//!
//! This crate provides an api for implementing new backends and targets for citadel. For an example, go to [experimental-asm](experimental/index.html)
//! **Important**: The api for the backend is still unstable and will still undergo a lot of changes.
//!
//! If you are writing a compiler and need low-level access rather than the [regular api](../api/index.html). You can also use the api provided by this crate.
//! For an example on how to do so, look at [WIP]

pub mod asm;
pub mod api;

