use crate::runtime::status::AoStatus;
use crate::runtime::types::AoType;

pub struct AoTypeBinOper {
    name: &'static str,

    bool_oper: Option<fn(bool, bool) -> bool>,
    int_oper: Option<fn(i32, i32) -> i32>,
    float_oper: Option<fn(f32, f32) -> f32>,
    ptr_oper: Option<fn(u32, u32) -> u32>,
    string_oper: Option<fn(&String, &String) -> String>,
}

impl AoTypeBinOper {
    fn throw(&self, left: AoType, right: AoType) -> AoStatus {
        AoStatus::InvalidOperation(format!("{} {} {}", left, self.name, right))
    }

    fn apply_bool(&self, left: bool, right: bool) -> Option<bool> {
        match self.bool_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    fn apply_int(&self, left: i32, right: i32) -> Option<i32> {
        match self.int_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    fn apply_float(&self, left: f32, right: f32) -> Option<f32> {
        match self.float_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    fn apply_ptr(&self, left: u32, right: u32) -> Option<u32> {
        match self.ptr_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    fn apply_string(&self, left: &String, right: &String) -> Option<String> {
        match self.string_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    pub fn apply(&self, left: AoType, right: AoType) -> AoStatus {
        match (&left, &right) {
            (AoType::AoBool(l), AoType::AoBool(r)) => {
                if let Some(res) = self.apply_bool(*l, *r) {
                    return AoStatus::Return(AoType::AoBool(res));
                }
            }
            (AoType::AoInt(l), AoType::AoInt(r)) => {
                if let Some(res) = self.apply_int(*l, *r) {
                    return AoStatus::Return(AoType::AoInt(res));
                }
            }
            (AoType::AoFloat(l), AoType::AoInt(r)) => {
                if let Some(res) = self.apply_float(*l, *r as f32) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoInt(l), AoType::AoFloat(r)) => {
                if let Some(res) = self.apply_float(*l as f32, *r) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoFloat(l), AoType::AoFloat(r)) => {
                if let Some(res) = self.apply_float(*l, *r) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoPtr(l), AoType::AoPtr(r)) => {
                if let Some(res) = self.apply_ptr(*l, *r) {
                    return AoStatus::Return(AoType::AoPtr(res));
                }
            }
            (AoType::AoPtr(l), AoType::AoInt(r)) => {
                if let Some(res) = self.apply_ptr(*l, *r as u32) {
                    return AoStatus::Return(AoType::AoPtr(res));
                }
            }
            (AoType::AoInt(l), AoType::AoPtr(r)) => {
                if let Some(res) = self.apply_int(*l, *r as i32) {
                    return AoStatus::Return(AoType::AoInt(res));
                }
            }
            (AoType::AoString(l), AoType::AoString(r)) => {
                if let Some(res) = self.apply_string(l, r) {
                    return AoStatus::Return(AoType::AoString(res));
                }
            }
            _ => (),
        };
        self.throw(left, right)
    }
}

pub static BIN_OPER_ADD: AoTypeBinOper = AoTypeBinOper {
    name: "+",
    bool_oper: Some(|l, r| l || r),
    int_oper: Some(|l, r| l + r),
    float_oper: Some(|l, r| l + r),
    ptr_oper: Some(|l, r| l + r),
    string_oper: Some(|l, r| format!("{}{}", l, r)),
};

pub static BIN_OPER_SUB: AoTypeBinOper = AoTypeBinOper {
    name: "-",
    bool_oper: None,
    int_oper: Some(|l, r| l - r),
    float_oper: Some(|l, r| l - r),
    ptr_oper: Some(|l, r| l - r),
    string_oper: None,
};

pub static BIN_OPER_MUL: AoTypeBinOper = AoTypeBinOper {
    name: "*",
    bool_oper: Some(|l, r| l && r),
    int_oper: Some(|l, r| l * r),
    float_oper: Some(|l, r| l * r),
    ptr_oper: Some(|l, r| l * r),
    string_oper: None,
};

pub static BIN_OPER_DIV: AoTypeBinOper = AoTypeBinOper {
    name: "/",
    bool_oper: None,
    int_oper: Some(|l, r| l / r),
    float_oper: Some(|l, r| l / r),
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_REM: AoTypeBinOper = AoTypeBinOper {
    name: "%",
    bool_oper: None,
    int_oper: Some(|l, r| l % r),
    float_oper: Some(|l, r| l % r),
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_BAND: AoTypeBinOper = AoTypeBinOper {
    name: "&",
    bool_oper: None,
    int_oper: Some(|l, r| l & r),
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_BOR: AoTypeBinOper = AoTypeBinOper {
    name: "|",
    bool_oper: None,
    int_oper: Some(|l, r| l | r),
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_BXOR: AoTypeBinOper = AoTypeBinOper {
    name: "^",
    bool_oper: None,
    int_oper: Some(|l, r| l ^ r),
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_SHL: AoTypeBinOper = AoTypeBinOper {
    name: "<<",
    bool_oper: None,
    int_oper: Some(|l, r| l << r),
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

pub static BIN_OPER_SHR: AoTypeBinOper = AoTypeBinOper {
    name: ">>",
    bool_oper: None,
    int_oper: Some(|l, r| l >> r),
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_ADD.apply(left, right),
            AoStatus::Return(AoType::AoBool(true))
        );

        // AoInt
        let left = AoType::AoInt(2);
        let right = AoType::AoInt(2);
        assert_eq!(
            BIN_OPER_ADD.apply(left, right),
            AoStatus::Return(AoType::AoInt(4))
        );

        // AoFloat
        let left = AoType::AoFloat(2.2);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_ADD.apply(left, right),
            AoStatus::Return(AoType::AoFloat(4.4))
        );

        // AoPtr
        let left = AoType::AoPtr(2);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_ADD.apply(left, right),
            AoStatus::Return(AoType::AoPtr(4))
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_ADD.apply(left, right),
            AoStatus::Return(AoType::AoString("HelloWorld".to_string()))
        );
    }

    #[test]
    fn test_sub() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_SUB.apply(left, right),
            AoStatus::InvalidOperation("true - false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(3);
        let right = AoType::AoInt(2);
        assert_eq!(
            BIN_OPER_SUB.apply(left, right),
            AoStatus::Return(AoType::AoInt(1))
        );

        // AoFloat
        let left = AoType::AoFloat(3.75);
        let right = AoType::AoFloat(2.25);
        assert_eq!(
            BIN_OPER_SUB.apply(left, right),
            AoStatus::Return(AoType::AoFloat(1.5))
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_SUB.apply(left, right),
            AoStatus::Return(AoType::AoPtr(1))
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_SUB.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" - \"World\"".to_string())
        );
    }

    #[test]
    fn test_mul() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_MUL.apply(left, right),
            AoStatus::Return(AoType::AoBool(false))
        );

        // AoInt
        let left = AoType::AoInt(3);
        let right = AoType::AoInt(3);
        assert_eq!(
            BIN_OPER_MUL.apply(left, right),
            AoStatus::Return(AoType::AoInt(9))
        );

        // AoFloat
        let left = AoType::AoFloat(3.5);
        let right = AoType::AoFloat(2.5);
        assert_eq!(
            BIN_OPER_MUL.apply(left, right),
            AoStatus::Return(AoType::AoFloat(8.75))
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(3);
        assert_eq!(
            BIN_OPER_MUL.apply(left, right),
            AoStatus::Return(AoType::AoPtr(9))
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_MUL.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" * \"World\"".to_string())
        );
    }

    #[test]
    fn test_div() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_DIV.apply(left, right),
            AoStatus::InvalidOperation("true / false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(3);
        let right = AoType::AoInt(2);
        assert_eq!(
            BIN_OPER_DIV.apply(left, right),
            AoStatus::Return(AoType::AoInt(1))
        );

        // AoFloat
        let left = AoType::AoFloat(3.0);
        let right = AoType::AoFloat(2.0);
        assert_eq!(
            BIN_OPER_DIV.apply(left, right),
            AoStatus::Return(AoType::AoFloat(1.5))
        );

        // AoPtr
        let left = AoType::AoPtr(2);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_DIV.apply(left, right),
            AoStatus::InvalidOperation("2p / 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_DIV.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" / \"World\"".to_string())
        );
    }

    #[test]
    fn test_rem() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_REM.apply(left, right),
            AoStatus::InvalidOperation("true % false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(5);
        let right = AoType::AoInt(2);
        assert_eq!(
            BIN_OPER_REM.apply(left, right),
            AoStatus::Return(AoType::AoInt(1))
        );

        // AoFloat
        let left = AoType::AoFloat(5.5);
        let right = AoType::AoFloat(2.0);
        assert_eq!(
            BIN_OPER_REM.apply(left, right),
            AoStatus::Return(AoType::AoFloat(1.5))
        );

        // AoPtr
        let left = AoType::AoPtr(5);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_REM.apply(left, right),
            AoStatus::InvalidOperation("5p % 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_REM.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" % \"World\"".to_string())
        );
    }

    #[test]
    fn test_band() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_BAND.apply(left, right),
            AoStatus::InvalidOperation("true & false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(4);
        let right = AoType::AoInt(6);
        assert_eq!(
            BIN_OPER_BAND.apply(left, right),
            AoStatus::Return(AoType::AoInt(4))
        );

        // AoFloat
        let left = AoType::AoFloat(3.3);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_BAND.apply(left, right),
            AoStatus::InvalidOperation("3.3f & 2.2f".to_string())
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_BAND.apply(left, right),
            AoStatus::InvalidOperation("3p & 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_BAND.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" & \"World\"".to_string())
        );
    }

    #[test]
    fn test_bor() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_BOR.apply(left, right),
            AoStatus::InvalidOperation("true | false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(4);
        let right = AoType::AoInt(6);
        assert_eq!(
            BIN_OPER_BOR.apply(left, right),
            AoStatus::Return(AoType::AoInt(6))
        );

        // AoFloat
        let left = AoType::AoFloat(3.3);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_BOR.apply(left, right),
            AoStatus::InvalidOperation("3.3f | 2.2f".to_string())
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_BOR.apply(left, right),
            AoStatus::InvalidOperation("3p | 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_BOR.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" | \"World\"".to_string())
        );
    }

    #[test]
    fn test_bxor() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_BXOR.apply(left, right),
            AoStatus::InvalidOperation("true ^ false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(4);
        let right = AoType::AoInt(6);
        assert_eq!(
            BIN_OPER_BXOR.apply(left, right),
            AoStatus::Return(AoType::AoInt(2))
        );

        // AoFloat
        let left = AoType::AoFloat(3.3);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_BXOR.apply(left, right),
            AoStatus::InvalidOperation("3.3f ^ 2.2f".to_string())
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_BXOR.apply(left, right),
            AoStatus::InvalidOperation("3p ^ 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_BXOR.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" ^ \"World\"".to_string())
        );
    }

    #[test]
    fn test_shl() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_SHL.apply(left, right),
            AoStatus::InvalidOperation("true << false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(2);
        let right = AoType::AoInt(1);
        assert_eq!(
            BIN_OPER_SHL.apply(left, right),
            AoStatus::Return(AoType::AoInt(4))
        );

        // AoFloat
        let left = AoType::AoFloat(3.3);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_SHL.apply(left, right),
            AoStatus::InvalidOperation("3.3f << 2.2f".to_string())
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_SHL.apply(left, right),
            AoStatus::InvalidOperation("3p << 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_SHL.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" << \"World\"".to_string())
        );
    }

    #[test]
    fn test_shr() {
        // AoBool
        let left = AoType::AoBool(true);
        let right = AoType::AoBool(false);
        assert_eq!(
            BIN_OPER_SHR.apply(left, right),
            AoStatus::InvalidOperation("true >> false".to_string())
        );

        // AoInt
        let left = AoType::AoInt(2);
        let right = AoType::AoInt(1);
        assert_eq!(
            BIN_OPER_SHR.apply(left, right),
            AoStatus::Return(AoType::AoInt(1))
        );

        // AoFloat
        let left = AoType::AoFloat(3.3);
        let right = AoType::AoFloat(2.2);
        assert_eq!(
            BIN_OPER_SHR.apply(left, right),
            AoStatus::InvalidOperation("3.3f >> 2.2f".to_string())
        );

        // AoPtr
        let left = AoType::AoPtr(3);
        let right = AoType::AoPtr(2);
        assert_eq!(
            BIN_OPER_SHR.apply(left, right),
            AoStatus::InvalidOperation("3p >> 2p".to_string())
        );

        // AoString
        let left = AoType::AoString("Hello".to_string());
        let right = AoType::AoString("World".to_string());
        assert_eq!(
            BIN_OPER_SHR.apply(left, right),
            AoStatus::InvalidOperation("\"Hello\" >> \"World\"".to_string())
        );
    }
}
