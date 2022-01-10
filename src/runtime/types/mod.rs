mod bin_oper;

use std::fmt::Display;
use std::ops::*;

use super::status::AoStatus;
use bin_oper::*;

/// The data type of the AOI virtual machine.
#[derive(Clone, Debug, PartialEq)]
pub enum AoType {
    /// Boolean
    AoBool(bool),
    /// Integer, i32 in rust
    AoInt(i32),
    /// Float, f32 in rust
    AoFloat(f32),
    /// Pointer
    AoPtr(u32),
    /// String
    AoString(String),
}

impl AoType {
    /// Create a default AoType.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::default(), AoType::AoInt(0));
    /// ```
    pub fn default() -> AoType {
        AoType::AoInt(0)
    }
}

impl Display for AoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoType::AoBool(v) => write!(f, "{}", v),
            AoType::AoInt(v) => write!(f, "{}", v),
            AoType::AoFloat(v) => write!(f, "{}f", v),
            AoType::AoPtr(v) => write!(f, "{}p", v),
            AoType::AoString(v) => write!(f, "\"{}\"", v),
        }
    }
}

macro_rules! impl_oper {
    ( $tr:ident, $fn:ident, $op:ident ) => {
        impl $tr for AoType {
            type Output = AoStatus;

            fn $fn(self, other: AoType) -> AoStatus {
                $op.apply(self, other)
            }
        }
    };
}

impl_oper!(Add, add, BIN_OPER_ADD);
impl_oper!(Sub, sub, BIN_OPER_SUB);
impl_oper!(Mul, mul, BIN_OPER_MUL);
impl_oper!(Div, div, BIN_OPER_DIV);
impl_oper!(Rem, rem, BIN_OPER_REM);
impl_oper!(BitAnd, bitand, BIN_OPER_BAND);
impl_oper!(BitOr, bitor, BIN_OPER_BOR);
impl_oper!(BitXor, bitxor, BIN_OPER_BXOR);
impl_oper!(Shl, shl, BIN_OPER_SHL);
impl_oper!(Shr, shr, BIN_OPER_SHR);

macro_rules! impl_from {
    ( $at:ident, $rt:ty ) => {
        impl From<$rt> for AoType {
            fn from(t: $rt) -> AoType {
                AoType::$at(t)
            }
        }
    };
}

impl_from!(AoBool, bool);
impl_from!(AoInt, i32);
impl_from!(AoFloat, f32);
impl_from!(AoPtr, u32);
impl_from!(AoString, String);

impl From<&str> for AoType {
    fn from(t: &str) -> AoType {
        AoType::AoString(t.to_string())
    }
}
