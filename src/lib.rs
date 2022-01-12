pub mod runtime;
pub mod serialization;

pub use runtime::*;
pub use serialization::AoAsmSerializer;

pub type AoProgram = Vec<Box<dyn runtime::opcode::AoOpcode>>;
