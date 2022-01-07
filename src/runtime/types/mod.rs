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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoType::AoBool(v) => write!(formatter, "{}", v),
            AoType::AoInt(v) => write!(formatter, "{}", v),
            AoType::AoFloat(f) => write!(formatter, "{}f", f),
            AoType::AoPtr(p) => write!(formatter, "{}p", p),
            AoType::AoString(s) => write!(formatter, "\"{}\"", s),
        }
    }
}

// region Operator

impl Add for AoType {
    type Output = AoStatus;

    fn add(self, other: AoType) -> AoStatus {
        BIN_OPER_ADD.apply(self, other)
    }
}

impl Sub for AoType {
    type Output = AoStatus;

    fn sub(self, other: AoType) -> AoStatus {
        BIN_OPER_SUB.apply(self, other)
    }
}

impl Mul for AoType {
    type Output = AoStatus;

    fn mul(self, other: AoType) -> AoStatus {
        BIN_OPER_MUL.apply(self, other)
    }
}

impl Div for AoType {
    type Output = AoStatus;

    fn div(self, other: AoType) -> AoStatus {
        BIN_OPER_DIV.apply(self, other)
    }
}

impl Rem for AoType {
    type Output = AoStatus;

    fn rem(self, other: AoType) -> AoStatus {
        BIN_OPER_REM.apply(self, other)
    }
}

impl BitAnd for AoType {
    type Output = AoStatus;

    fn bitand(self, other: AoType) -> AoStatus {
        BIN_OPER_BAND.apply(self, other)
    }
}

impl BitOr for AoType {
    type Output = AoStatus;

    fn bitor(self, other: AoType) -> AoStatus {
        BIN_OPER_BOR.apply(self, other)
    }
}

impl BitXor for AoType {
    type Output = AoStatus;

    fn bitxor(self, other: AoType) -> AoStatus {
        BIN_OPER_BXOR.apply(self, other)
    }
}

impl Shl for AoType {
    type Output = AoStatus;

    fn shl(self, other: AoType) -> AoStatus {
        BIN_OPER_SHL.apply(self, other)
    }
}

impl Shr for AoType {
    type Output = AoStatus;

    fn shr(self, other: AoType) -> AoStatus {
        BIN_OPER_SHR.apply(self, other)
    }
}

// endregion

// region From

impl From<bool> for AoType {
    /// Create a AoBool from a bool.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from(true), AoType::AoBool(true));
    /// ```
    fn from(t: bool) -> AoType {
        AoType::AoBool(t)
    }
}

impl From<i32> for AoType {
    /// Create a AoInt from an i32.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from(1), AoType::AoInt(1));
    /// ```
    fn from(t: i32) -> AoType {
        AoType::AoInt(t)
    }
}

impl From<f32> for AoType {
    /// Create a AoFloat from a f32.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from(1.0), AoType::AoFloat(1.0));
    /// ```
    fn from(t: f32) -> AoType {
        AoType::AoFloat(t)
    }
}

impl From<u32> for AoType {
    /// Create a AoPtr from a u32.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from(1_u32), AoType::AoPtr(1));
    /// ```
    fn from(t: u32) -> AoType {
        AoType::AoPtr(t)
    }
}

impl From<String> for AoType {
    /// Create a AoString from a String.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from("hello".to_string()), AoType::AoString("hello".to_string()));
    /// ```
    fn from(t: String) -> AoType {
        AoType::AoString(t)
    }
}

impl From<&str> for AoType {
    /// Create a AoString from a &str.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoType::from("Hello World"), AoType::AoString("Hello World".to_string()));
    fn from(t: &str) -> AoType {
        AoType::AoString(t.to_string())
    }
}

// endregion
