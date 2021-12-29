use super::types::AoType;
use std::fmt::Display;

pub enum AoStatus {
    Ok,
    Exit,
    Return(AoType),

    BadDataStack,

    CallStackOverflow,
    CallStackUnderflow,
    DataStackOverflow,
    DataStackUnderflow,

    SetValueInvalidType(String),
    SetValueInvalidTarget(String),

    InvalidOperation(String),

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
