use crate::runtime::status::AoStatus;
use crate::runtime::types::AoType;

pub(crate) struct AoTypeBinOper {
    name: &'static str,

    bool_oper: Option<fn(bool, bool) -> bool>,
    int_oper: Option<fn(i32, i32) -> i32>,
    float_oper: Option<fn(f32, f32) -> f32>,
    ptr_oper: Option<fn(u32, u32) -> u32>,
    string_oper: Option<fn(&String, &String) -> String>,
}

impl AoTypeBinOper {
    pub fn apply(&self, left: AoType, right: AoType) -> AoStatus {
        match (&left, &right) {
            (AoType::AoBool(l), AoType::AoBool(r)) => {
                if let Some(res) = self.bool_oper.map(|oper| oper(*l, *r)) {
                    return AoStatus::Return(AoType::AoBool(res));
                }
            }
            (AoType::AoInt(l), AoType::AoInt(r)) => {
                if let Some(res) = self.int_oper.map(|oper| oper(*l, *r)) {
                    return AoStatus::Return(AoType::AoInt(res));
                }
            }
            (AoType::AoFloat(l), AoType::AoInt(r)) => {
                if let Some(res) = self.float_oper.map(|oper| oper(*l, *r as f32)) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoInt(l), AoType::AoFloat(r)) => {
                if let Some(res) = self.float_oper.map(|oper| oper(*l as f32, *r)) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoFloat(l), AoType::AoFloat(r)) => {
                if let Some(res) = self.float_oper.map(|oper| oper(*l, *r)) {
                    return AoStatus::Return(AoType::AoFloat(res));
                }
            }
            (AoType::AoPtr(l), AoType::AoPtr(r)) => {
                if let Some(res) = self.ptr_oper.map(|oper| oper(*l, *r)) {
                    return AoStatus::Return(AoType::AoPtr(res));
                }
            }
            (AoType::AoPtr(l), AoType::AoInt(r)) => {
                if let Some(res) = self.ptr_oper.map(|oper| oper(*l, *r as u32)) {
                    return AoStatus::Return(AoType::AoPtr(res));
                }
            }
            (AoType::AoInt(l), AoType::AoPtr(r)) => {
                if let Some(res) = self.int_oper.map(|oper| oper(*l, *r as i32)) {
                    return AoStatus::Return(AoType::AoInt(res));
                }
            }
            (AoType::AoString(l), AoType::AoString(r)) => {
                if let Some(res) = self.string_oper.map(|oper| oper(l, r)) {
                    return AoStatus::Return(AoType::AoString(res));
                }
            }
            _ => (),
        };
        AoStatus::InvalidOperation(format!("{} {} {}", left, self.name, right))
    }
}

static BIN_OPER_NONE: AoTypeBinOper = AoTypeBinOper {
    name: "",
    bool_oper: None,
    int_oper: None,
    float_oper: None,
    ptr_oper: None,
    string_oper: None,
};

macro_rules! bop {
    ( $name:ident, $dname:tt, $( $fname:ident : $fvalue:expr ),*, ) => {
        #[allow(clippy::needless_update)]
        pub(crate) static $name: AoTypeBinOper = AoTypeBinOper {
            name: stringify!($dname),
            $(
                $fname: $fvalue,
            )*
            ..BIN_OPER_NONE
        };
    };
}

macro_rules! op {
    ( $op:tt ) => {
        Some(|l, r| l $op r)
    };
}

bop!(BIN_OPER_ADD, +,
    bool_oper: op!(||),
    int_oper: op!(+),
    float_oper: op!(+),
    ptr_oper: op!(+),
    string_oper: Some(|l, r| format!("{}{}", l, r)),
);

bop!(BIN_OPER_SUB, -,
    int_oper: op!(-),
    float_oper: op!(-),
    ptr_oper: op!(-),
);

bop!(BIN_OPER_MUL, *,
    bool_oper: op!(&&),
    int_oper: op!(*),
    float_oper: op!(*),
    ptr_oper: op!(*),
);

bop!(BIN_OPER_DIV, /,
    int_oper: op!(/),
    float_oper: op!(/),
);

bop!(BIN_OPER_REM, %,
    int_oper: op!(%),
    float_oper: op!(%),
);

bop!(BIN_OPER_BAND, &,
    int_oper: op!(&),
);

bop!(BIN_OPER_BOR, |,
    int_oper: op!(|),
);

bop!(BIN_OPER_BXOR, ^,
    int_oper: op!(^),
);

bop!(BIN_OPER_SHL, <<,
    int_oper: op!(<<),
);

bop!(BIN_OPER_SHR, >>,
    int_oper: op!(>>),
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_op {
        ( $op:ident, $lv:expr, $rv:expr, $res:expr ) => {
            test_op!(
                $op,
                AoString,
                $lv.to_string(),
                $rv.to_string(),
                $res.to_string()
            );
        };
        ( $op:ident, $t:ident, $lv:expr, $rv:expr, $res:expr ) => {
            let left = AoType::$t($lv);
            let right = AoType::$t($rv);
            assert_eq!($op.apply(left, right), AoStatus::Return(AoType::$t($res)));
        };
    }

    macro_rules! test_op_fail {
        ( $op:ident, $lv:expr, $rv:expr, $msg:expr ) => {
            test_op_fail!($op, AoString, $lv.to_string(), $rv.to_string(), $msg);
        };
        ( $op:ident, $t:ident, $lv:expr, $rv:expr, $msg:expr ) => {
            let left = AoType::$t($lv);
            let right = AoType::$t($rv);
            assert_eq!(
                $op.apply(left, right),
                AoStatus::InvalidOperation($msg.to_string())
            );
        };
    }

    #[test]
    fn test_add() {
        test_op!(BIN_OPER_ADD, AoBool, true, false, true);
        test_op!(BIN_OPER_ADD, AoInt, 2, 2, 4);
        test_op!(BIN_OPER_ADD, AoFloat, 2.2, 2.2, 4.4);
        test_op!(BIN_OPER_ADD, AoPtr, 2, 2, 4);
        test_op!(BIN_OPER_ADD, "Hello", "World", "HelloWorld");
    }

    #[test]
    fn test_sub() {
        test_op!(BIN_OPER_SUB, AoInt, 3, 2, 1);
        test_op!(BIN_OPER_SUB, AoFloat, 3.75, 2.25, 1.5);
        test_op!(BIN_OPER_SUB, AoPtr, 3, 2, 1);

        test_op_fail!(BIN_OPER_SUB, AoBool, true, false, "true - false");
        test_op_fail!(BIN_OPER_SUB, "Hello", "World", "\"Hello\" - \"World\"");
    }

    #[test]
    fn test_mul() {
        test_op!(BIN_OPER_MUL, AoBool, true, false, false);
        test_op!(BIN_OPER_MUL, AoInt, 3, 3, 9);
        test_op!(BIN_OPER_MUL, AoFloat, 3.5, 2.5, 8.75);
        test_op!(BIN_OPER_MUL, AoPtr, 3, 3, 9);

        test_op_fail!(BIN_OPER_MUL, "Hello", "World", "\"Hello\" * \"World\"");
    }

    #[test]
    fn test_div() {
        test_op!(BIN_OPER_DIV, AoInt, 3, 2, 1);
        test_op!(BIN_OPER_DIV, AoFloat, 3.0, 2.0, 1.5);

        test_op_fail!(BIN_OPER_DIV, AoBool, true, false, "true / false");
        test_op_fail!(BIN_OPER_DIV, AoPtr, 2, 2, "2p / 2p");
        test_op_fail!(BIN_OPER_DIV, "Hello", "World", "\"Hello\" / \"World\"");
    }

    #[test]
    fn test_rem() {
        test_op!(BIN_OPER_REM, AoInt, 5, 2, 1);
        test_op!(BIN_OPER_REM, AoFloat, 5.5, 2.0, 1.5);

        test_op_fail!(BIN_OPER_REM, AoBool, true, false, "true % false");
        test_op_fail!(BIN_OPER_REM, AoPtr, 5, 2, "5p % 2p");
        test_op_fail!(BIN_OPER_REM, "Hello", "World", "\"Hello\" % \"World\"");
    }

    #[test]
    fn test_band() {
        test_op!(BIN_OPER_BAND, AoInt, 0b110, 0b100, 0b100);

        test_op_fail!(BIN_OPER_BAND, AoBool, true, false, "true & false");
        test_op_fail!(BIN_OPER_BAND, AoFloat, 3.3, 2.2, "3.3f & 2.2f");
        test_op_fail!(BIN_OPER_BAND, AoPtr, 3, 2, "3p & 2p");
        test_op_fail!(BIN_OPER_BAND, "Hello", "World", "\"Hello\" & \"World\"");
    }

    #[test]
    fn test_bor() {
        test_op!(BIN_OPER_BOR, AoInt, 0b110, 0b100, 0b110);

        test_op_fail!(BIN_OPER_BOR, AoBool, true, false, "true | false");
        test_op_fail!(BIN_OPER_BOR, AoFloat, 3.3, 2.2, "3.3f | 2.2f");
        test_op_fail!(BIN_OPER_BOR, AoPtr, 3, 2, "3p | 2p");
        test_op_fail!(BIN_OPER_BOR, "Hello", "World", "\"Hello\" | \"World\"");
    }

    #[test]
    fn test_bxor() {
        test_op!(BIN_OPER_BXOR, AoInt, 0b110, 0b100, 0b010);

        test_op_fail!(BIN_OPER_BXOR, AoBool, true, false, "true ^ false");
        test_op_fail!(BIN_OPER_BXOR, AoFloat, 3.3, 2.2, "3.3f ^ 2.2f");
        test_op_fail!(BIN_OPER_BXOR, AoPtr, 3, 2, "3p ^ 2p");
        test_op_fail!(BIN_OPER_BXOR, "Hello", "World", "\"Hello\" ^ \"World\"");
    }

    #[test]
    fn test_shl() {
        test_op!(BIN_OPER_SHL, AoInt, 0b010, 1, 0b100);

        test_op_fail!(BIN_OPER_SHL, AoBool, true, false, "true << false");
        test_op_fail!(BIN_OPER_SHL, AoFloat, 3.3, 2.2, "3.3f << 2.2f");
        test_op_fail!(BIN_OPER_SHL, AoPtr, 3, 2, "3p << 2p");
        test_op_fail!(BIN_OPER_SHL, "Hello", "World", "\"Hello\" << \"World\"");
    }

    #[test]
    fn test_shr() {
        test_op!(BIN_OPER_SHR, AoInt, 0b010, 1, 0b001);

        test_op_fail!(BIN_OPER_SHR, AoBool, true, false, "true >> false");
        test_op_fail!(BIN_OPER_SHR, AoFloat, 3.3, 2.2, "3.3f >> 2.2f");
        test_op_fail!(BIN_OPER_SHR, AoPtr, 3, 2, "3p >> 2p");
        test_op_fail!(BIN_OPER_SHR, "Hello", "World", "\"Hello\" >> \"World\"");
    }
}
