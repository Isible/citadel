//! This is the api for implementing a new backend
//! or compiling your IR.<br>
//! This api is still unstable, which is why it will
//! reside in the experimental module until it is
//! stabelized.

use std::{fmt::{Debug, Display}, io};

use citadel_frontend::ir::{irgen::IRStream, IRStmt};

pub trait Target: Debug + Default + Copy + Clone {
    fn name(&self) -> &str;
}

/// # Introduction
/// 
/// The Backend trait. A Backend is the part
/// of a compiler responsible for generating
/// machine code or some other kind of low-level
/// representation for your code like assembly,
/// web-assembly...
/// 
/// Luckily you don't need to write these backends
/// yourself, but can use backends that are made by
/// the community. Popular backends are: \[WIP\]
/// 
/// # Implementation
/// 
/// For creating a new backend, simply implement
/// this type for your backend's struct.
/// 
/// The backend requires you to define 3 types.
/// These are: [Backend::Target], [Backend::Element], [Backend::Output]
/// 
/// ## [Backend::Target]
/// 
/// This type specifies the Target that the backend should compile to.
/// There are 3 different cases for filling this type.
/// 1. You are building a backend that only needs to work on one target.<p>
///    => you can set [Backend::Target] to this specific target.
/// 2. You are building a relativley small backend that should compile to
///    multiple targets<p>
///    => you should use a generic for the backend and then set [Backend::Target]
///    to that generic.
/// 3. You are building a larger backend that produces something like machine code or assembly.<p>
///    => Create a specific backend for your target or
///    target architecture that spits out machine code,
///    assembly or something like that.
/// 
/// ## [Backend::Output]
/// 
/// This is the type that gets returned after generating the code.
/// It should ideally be a [Vec], but can in fact be any type
/// that implements [IntoIterator].
/// 
/// **Example**: Vec<AsmElement> if your backend compiles to assembly
/// 
/// Note: The element in the iterator needs to implement Display
/// so the code can be outputted to a file easily.
/// 
/// ## [Backend::Element]
/// 
/// This is the type of the data contained in [Backend::Output].<p>
/// This is required since rust cannot yet ensure that the type in
/// the iterator implements Display.
/// 
/// **For example**: If [Backend::Output] is a `Vec<AsmElement>`, then [Backend::Element]
/// would be `AsmElement`
/// 
/// TODO: Trait methods
pub trait Backend: Debug {
    type Target: self::Target;
    // TODO: Inline element type into Output type once RFC 2289
    // (https://github.com/rust-lang/rust/issues/52662) is stabelized
    type Element: Display;
    type Output: IntoIterator<Item = Self::Element>;

    /// Main function of the backend. This will take in a stream
    /// of IRStmts and generate code based on them. The target for
    /// code generation is [`Backend::Target`]
    fn generate(&self, ir_stream: IRStream) -> Self::Output;

    /// This returns the target of the backend instance.
    /// 
    /// This method is required since backends are supposed
    /// to be able to handle multiple targets and generate
    /// code based on the selected target
    fn target(&self) -> Self::Target {
        Self::Target::default()
    }

    /// This is for outputting a file.
    /// If your backend compiles to something
    /// like assembly that is represented in
    /// plain text, you can return None.
    /// 
    /// If youre backend compiles to something
    /// like machine code however, you will need
    /// to handle file creation yourself using
    /// the citadel apis. In that case you should
    /// return Some(...) and the result of whether
    /// file creation was successful
    fn to_file(&self) -> Option<io::Result<()>> {
        None
    }
}