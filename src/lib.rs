//Define library

extern crate nom;

mod parser;
mod runtime;

pub use self::parser::{program, Node};
pub use self::runtime::{Value, run};