use std::cmp::PartialEq;
use std::fmt::Display;

use super::types::AoType;

/// Status in the runtime.
#[derive(Debug, PartialEq)]
pub enum AoStatus {
    /// The program is running or the operation executed successfully.
    Ok,
    /// The program is finished.
    Exit,
    /// The operation returned a value.
    Return(AoType),

    /// The data stack not match the expected type.
    BadDataStack,

    /// The call stack is full.
    CallStackOverflow,
    /// The call stack is empty.
    CallStackUnderflow,
    /// The data stack is full.
    DataStackOverflow,
    /// The data stack is empty.
    DataStackUnderflow,

    /// Try to set the type-restricted register to a different type value.
    SetValueInvalidType(String),
    /// Try to write to a read-only register or immediate value.
    SetValueInvalidTarget(String),

    /// Attempt to perform an incompatible operation between two types.
    InvalidOperation(String),

    /// Internal error.
    InternalError,
}

impl Display for AoStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoStatus::Ok => write!(formatter, "Ok"),
            AoStatus::Exit => write!(formatter, "Exit"),
            AoStatus::Return(v) => write!(formatter, "Return({})", v),

            AoStatus::BadDataStack => write!(formatter, "Bad Data Stack"),

            AoStatus::CallStackOverflow => write!(formatter, "Call Stack Overflow"),
            AoStatus::CallStackUnderflow => write!(formatter, "Call Stack Underflow"),
            AoStatus::DataStackOverflow => write!(formatter, "Data Stack Overflow"),
            AoStatus::DataStackUnderflow => write!(formatter, "Data Stack Underflow"),

            AoStatus::SetValueInvalidType(v) => write!(formatter, "Set Value Invalid Type({})", v),
            AoStatus::SetValueInvalidTarget(v) => {
                write!(formatter, "Set Value Invalid Target({})", v)
            }

            AoStatus::InvalidOperation(v) => write!(formatter, "Invalid Operation({})", v),

            AoStatus::InternalError => write!(formatter, "Internal Error"),
        }
    }
}
