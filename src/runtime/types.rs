use super::status::AoStatus;
use std::fmt::Display;

pub enum AoType {
    AoBool(bool),
    AoInt(i32),
    AoFloat(f32),
    AoPtr(u32),
    AoString(String),
}

impl AoType {
    pub fn default() -> AoType {
        AoType::AoInt(0)
    }

    pub fn clone(&self) -> AoType {
        match self {
            AoType::AoBool(b) => AoType::AoBool(*b),
            AoType::AoInt(i) => AoType::AoInt(*i),
            AoType::AoFloat(f) => AoType::AoFloat(*f),
            AoType::AoPtr(p) => AoType::AoPtr(*p),
            AoType::AoString(s) => AoType::AoString(s.clone()),
        }
    }
}

impl From<bool> for AoType {
    fn from(t: bool) -> AoType {
        AoType::AoBool(t)
    }
}

impl From<i32> for AoType {
    fn from(t: i32) -> AoType {
        AoType::AoInt(t)
    }
}

impl From<f32> for AoType {
    fn from(t: f32) -> AoType {
        AoType::AoFloat(t)
    }
}

impl From<u32> for AoType {
    fn from(t: u32) -> AoType {
        AoType::AoPtr(t)
    }
}

impl From<String> for AoType {
    fn from(t: String) -> AoType {
        AoType::AoString(t)
    }
}

impl From<&str> for AoType {
    fn from(t: &str) -> AoType {
        AoType::AoString(t.to_string())
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

pub struct AoTypeBinOper {
    pub name: &'static str,

    pub bool_oper: Option<fn(bool, bool) -> bool>,
    pub int_oper: Option<fn(i32, i32) -> i32>,
    pub float_oper: Option<fn(f32, f32) -> f32>,
    pub ptr_oper: Option<fn(u32, u32) -> u32>,
    pub string_oper: Option<fn(&String, &String) -> String>,
}

impl AoTypeBinOper {
    pub fn throw(&self, left: AoType, right: AoType) -> AoStatus {
        AoStatus::InvalidOperation(format!("{} {} {}", left, self.name, right))
    }

    pub fn apply_bool(&self, left: bool, right: bool) -> Option<bool> {
        match self.bool_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    pub fn apply_int(&self, left: i32, right: i32) -> Option<i32> {
        match self.int_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    pub fn apply_float(&self, left: f32, right: f32) -> Option<f32> {
        match self.float_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    pub fn apply_ptr(&self, left: u32, right: u32) -> Option<u32> {
        match self.ptr_oper {
            Some(oper) => Some(oper(left, right)),
            None => None,
        }
    }

    pub fn apply_string(&self, left: &String, right: &String) -> Option<String> {
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

pub static BIN_OPER_MOD: AoTypeBinOper = AoTypeBinOper {
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
