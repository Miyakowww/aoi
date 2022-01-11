pub mod runtime;
pub mod serialization;

pub use self::runtime::opcode::{opcodes, AoArg, AoArgLowerCase, AoOpcode};
pub use self::runtime::status::AoStatus;
pub use self::runtime::types::AoType;
pub use self::runtime::vm::AoVM;
pub use self::serialization::AoAsmSerializer;

pub type AoProgram = Vec<Box<dyn runtime::opcode::AoOpcode>>;
