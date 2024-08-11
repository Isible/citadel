//! # Citadel - frontend
//! 
//! The frontend crate of the citadel project
//! 
//! For information on what exactly citadel is you should visit our [github-repository](https://github.com/Isible/citadel/blob/main/README.md)
//! 
//! This crate mainly provides an api for generating an Intermediary Representation <br>
//! in the form of an Abstract Syntax tree. Generating this tree is most easily achievable <br>
//! through [`crate::ir_gen`]. This provides a simple struct for generating the IR. <br>
//! However, since the IRGenerator is just a vector under the hood you can also easily implement your own. <br>
//! For help we recommend looking at our own implementation: [`crate::ir_gen`]

pub mod hir;
pub mod util;
mod tests;