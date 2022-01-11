use std::fmt::Display;

use super::status::AoStatus;
use super::types::*;
use super::vm::AoVM;

/// The argument of the opcode.
#[derive(Debug, PartialEq)]
pub enum AoArg {
    /// Pointer to the bottom of the stack frame.
    DSB,
    /// Pointer to the top of the stack.
    DST,
    /// Program counter.
    PC,
    /// Pointer for accessing data.
    DP,
    /// Register for calculation.
    CA,
    /// Stack.
    DS,
    /// Global variables.
    GVS,
    /// Immediate value.
    Imm(AoType),
}

impl AoArg {
    /// Returns the value of the argument.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    /// use aoi::runtime::vm::AoVM;
    ///
    /// let vm = AoVM::new(|_, _| None, 0);
    /// assert_eq!(AoArg::CA.get_value(&vm), AoType::default());
    /// ```
    pub fn get_value(&self, vm: &AoVM) -> AoType {
        match self {
            AoArg::DSB => AoType::AoPtr(vm.dsb as u32),
            AoArg::DST => AoType::AoPtr(vm.ds.len() as u32),
            AoArg::PC => AoType::AoPtr(vm.pc),
            AoArg::DP => AoType::AoPtr(vm.dp),
            AoArg::CA => vm.ca.clone(),
            AoArg::DS => vm.ds[vm.dp as usize].clone(),
            AoArg::GVS => vm.gvs[vm.dp as usize].clone(),
            AoArg::Imm(value) => value.clone(),
        }
    }

    /// Sets the value of the argument.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    /// use aoi::runtime::vm::AoVM;
    ///
    /// let mut vm = AoVM::new(|_, _| None, 0);
    /// AoArg::CA.set_value(&mut vm, AoType::AoPtr(0x12345678));
    /// assert_eq!(vm.ca, AoType::AoPtr(0x12345678));
    /// ```
    pub fn set_value(&self, vm: &mut AoVM, value: AoType) -> AoStatus {
        match self {
            AoArg::DSB => match value {
                AoType::AoPtr(p) => {
                    vm.dsb = p;
                    AoStatus::Ok
                }
                _ => AoStatus::SetValueInvalidType(format!("cannot set DSB to {}", value)),
            },
            AoArg::DST => AoStatus::SetValueInvalidTarget("cannot set DST".to_string()),
            AoArg::PC => match value {
                AoType::AoPtr(p) => {
                    vm.pc = p;
                    AoStatus::Ok
                }
                _ => {
                    AoStatus::SetValueInvalidType("cannot set PC to non-pointer value".to_string())
                }
            },
            AoArg::DP => match value {
                AoType::AoPtr(p) => {
                    vm.dp = p;
                    AoStatus::Ok
                }
                _ => {
                    AoStatus::SetValueInvalidType("cannot set DP to non-pointer value".to_string())
                }
            },
            AoArg::CA => {
                vm.ca = value;
                AoStatus::Ok
            }
            AoArg::DS => {
                vm.ds[vm.dp as usize] = value;
                AoStatus::Ok
            }
            AoArg::GVS => {
                vm.gvs[vm.dp as usize] = value;
                AoStatus::Ok
            }
            AoArg::Imm(_) => {
                AoStatus::SetValueInvalidTarget("cannot set immediate value".to_string())
            }
        }
    }
}

macro_rules! impl_from {
    ( $at:ident, &str ) => {
        impl From<&str> for AoArg {
            fn from(t: &str) -> AoArg {
                AoArg::Imm(AoType::$at(t.to_string()))
            }
        }
    };
    ( $at:ident, $rt:ty ) => {
        impl From<$rt> for AoArg {
            fn from(t: $rt) -> AoArg {
                AoArg::Imm(AoType::$at(t))
            }
        }
    };
}

impl_from!(AoBool, bool);
impl_from!(AoInt, i32);
impl_from!(AoFloat, f32);
impl_from!(AoPtr, u32);
impl_from!(AoString, String);
impl_from!(AoString, &str);

impl Display for AoArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoArg::DSB => write!(f, "dsb"),
            AoArg::DST => write!(f, "dst"),
            AoArg::PC => write!(f, "pc"),
            AoArg::DP => write!(f, "dp"),
            AoArg::CA => write!(f, "ca"),
            AoArg::DS => write!(f, "ds"),
            AoArg::GVS => write!(f, "gvs"),
            AoArg::Imm(v) => write!(f, "{}", v),
        }
    }
}

#[allow(non_camel_case_types)]
pub enum AoArgLowerCase {
    dsb,
    dst,
    pc,
    dp,
    ca,
    ds,
    gvs,
    imm(AoType),
}

impl AoArgLowerCase {
    pub fn to_aoarg(&self) -> AoArg {
        match self {
            AoArgLowerCase::dsb => AoArg::DSB,
            AoArgLowerCase::dst => AoArg::DST,
            AoArgLowerCase::pc => AoArg::PC,
            AoArgLowerCase::dp => AoArg::DP,
            AoArgLowerCase::ca => AoArg::CA,
            AoArgLowerCase::ds => AoArg::DS,
            AoArgLowerCase::gvs => AoArg::GVS,
            AoArgLowerCase::imm(v) => AoArg::Imm(v.clone()),
        }
    }
}

/// Aoi opcode.
pub enum AoOpCode {
    /// No operation.
    NOP,

    /// Call a function.
    CALL(u32),
    /// Return from a function.
    RET,
    /// Jump to an address.
    JMP(u32),
    /// Jump to an address if CA is true.
    JT(u32),
    /// Jump to an address if CA is false.
    JF(u32),

    /// Move the value.
    MOV(AoArg, AoArg),
    /// Interrupt.
    INT(u8),

    /// Push a value onto the stack.
    PUSH(AoArg),
    /// Pop a value from the stack and move it to the CA or ignore it.
    POP(bool),

    /// Add the value to the CA.
    ADD(AoArg),
    /// Subtract the value from the CA.
    SUB(AoArg),
    /// Multiply the value to the CA.
    MUL(AoArg),
    /// Divide the value from the CA.
    DIV(AoArg),
    /// Remainder the value from the CA.
    REM(AoArg),
    /// Increment the CA.
    INC,
    /// Decrement the CA.
    DEC,

    /// Logical and the value to the CA.
    AND(AoArg),
    /// Logical or the value to the CA.
    OR(AoArg),
    /// Logical xor the value to the CA.
    XOR(AoArg),
    /// Logical not the value to the CA.
    NOT,

    /// Bitwise and the value to the CA.
    BAND(AoArg),
    /// Bitwise or the value to the CA.
    BOR(AoArg),
    /// Bitwise xor the value to the CA.
    BXOR(AoArg),
    /// Bitwise not the value to the CA.
    BNOT,

    /// Shift left the value to the CA.
    SHL(AoArg),
    /// Shift right the value to the CA.
    SHR(AoArg),

    /// Compare if the CA is equal to the value.
    EQU(AoArg),
    /// Compare if the CA is not equal to the value.
    NEQ(AoArg),
    /// Compare if the CA is greater than the value.
    GT(AoArg),
    /// Compare if the CA is less than the value.
    LT(AoArg),
    /// Compare if the CA is greater than or equal to the value.
    GE(AoArg),
    /// Compare if the CA is less than or equal to the value.
    LE(AoArg),

    /// Cast the CA to the AoInt type.
    CSI,
    /// Cast the CA to the AoFloat type.
    CSF,
    /// Cast the CA to the AoPtr type.
    CSP,
    /// Cast the CA to the AoString type.
    CSS,

    /// Check if the CA is AoBool.
    ISB,
    /// Check if the CA is AoInt.
    ISI,
    /// Check if the CA is AoFloat.
    ISF,
    /// Check if the CA is AoPtr.
    ISP,
    /// Check if the CA is AoString.
    ISS,

    /// Set the DP to the DSB + offset to access the argument.
    ARG(u32),
    /// Set the DSB to the DST - argc to create a new stack frame.
    CNF(u32),
}

impl AoOpCode {
    pub fn execute(&self, vm: &mut AoVM) -> AoStatus {
        match self {
            AoOpCode::NOP => (),

            AoOpCode::CALL(addr) => {
                if vm.cs.len() >= 100000 {
                    return AoStatus::CallStackOverflow;
                }
                vm.cs.push(vm.pc);
                vm.pc = *addr;
            }
            AoOpCode::RET => {
                if vm.cs.is_empty() {
                    return AoStatus::CallStackUnderflow;
                }

                let dsb = vm.dsb - 1;
                if let AoType::AoPtr(ptr) = vm.ds[dsb as usize] {
                    vm.dsb = ptr;
                    vm.ds.resize_with(dsb as usize, AoType::default);
                    vm.pc = vm.cs.pop().unwrap();
                } else {
                    return AoStatus::BadDataStack;
                }
            }
            AoOpCode::JMP(addr) => {
                vm.pc = *addr;
            }
            AoOpCode::JT(addr) => {
                if match vm.ca {
                    AoType::AoBool(b) => b,
                    AoType::AoInt(i) => i != 0,
                    AoType::AoFloat(f) => f != 0.0,
                    _ => false,
                } {
                    vm.pc = *addr;
                }
            }
            AoOpCode::JF(addr) => {
                if !match vm.ca {
                    AoType::AoBool(b) => b,
                    AoType::AoInt(i) => i != 0,
                    AoType::AoFloat(f) => f != 0.0,
                    _ => false,
                } {
                    vm.pc = *addr;
                }
            }

            AoOpCode::MOV(dst, src) => match dst.set_value(vm, src.get_value(vm)) {
                AoStatus::Ok => (),
                err => return err,
            },
            AoOpCode::INT(id) => {
                if *id == 0 {
                    return AoStatus::Exit;
                }

                let mut args: Vec<AoType> = Vec::new();
                for arg in vm.ds[vm.dsb as usize..].iter() {
                    args.push(arg.clone());
                }

                if let Some(value) = (vm.interrupt)(*id, args) {
                    vm.ca = value;
                };

                let dsb = vm.dsb - 1;
                if let AoType::AoPtr(ptr) = vm.ds[dsb as usize] {
                    vm.dsb = ptr;
                    vm.ds.resize_with(dsb as usize, AoType::default);
                } else {
                    return AoStatus::BadDataStack;
                }
            }

            AoOpCode::PUSH(src) => {
                if !vm.push(src.get_value(vm)) {
                    return AoStatus::DataStackOverflow;
                }
            }
            AoOpCode::POP(save) => {
                let value = vm.pop();
                if let Some(value) = value {
                    if *save {
                        vm.ca = value;
                    }
                } else {
                    return AoStatus::DataStackUnderflow;
                }
            }

            AoOpCode::ADD(src) => {
                let res = vm.ca.clone() + src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::SUB(src) => {
                let res = vm.ca.clone() - src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::MUL(src) => {
                let res = vm.ca.clone() * src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::DIV(src) => {
                let res = vm.ca.clone() / src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::REM(src) => {
                let res = vm.ca.clone() % src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::INC => match vm.ca {
                AoType::AoInt(i) => {
                    vm.ca = AoType::AoInt(i + 1);
                }
                AoType::AoFloat(f) => {
                    vm.ca = AoType::AoFloat(f + 1.0);
                }
                _ => return AoStatus::InvalidOperation(format!("inc {}", vm.ca)),
            },
            AoOpCode::DEC => match vm.ca {
                AoType::AoInt(i) => {
                    vm.ca = AoType::AoInt(i - 1);
                }
                AoType::AoFloat(f) => {
                    vm.ca = AoType::AoFloat(f - 1.0);
                }
                _ => return AoStatus::InvalidOperation(format!("dec {}", vm.ca)),
            },

            AoOpCode::AND(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
                    vm.ca = AoType::AoBool(*l && *r);
                } else {
                    return AoStatus::InvalidOperation(format!("{} && {}", left, right));
                }
            }
            AoOpCode::OR(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
                    vm.ca = AoType::AoBool(*l || *r);
                } else {
                    return AoStatus::InvalidOperation(format!("{} || {}", left, right));
                }
            }
            AoOpCode::XOR(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
                    vm.ca = AoType::AoBool(*l ^ *r);
                } else {
                    return AoStatus::InvalidOperation(format!("{} ^ {}", left, right));
                }
            }
            AoOpCode::NOT => {
                if let AoType::AoBool(b) = vm.ca {
                    vm.ca = AoType::AoBool(!b);
                } else {
                    return AoStatus::InvalidOperation(format!("!{}", vm.ca));
                }
            }

            AoOpCode::BAND(src) => {
                let res = vm.ca.clone() & src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::BOR(src) => {
                let res = vm.ca.clone() | src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::BXOR(src) => {
                let res = vm.ca.clone() ^ src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::BNOT => {
                if let AoType::AoInt(i) = vm.ca {
                    vm.ca = AoType::AoInt(!i);
                } else {
                    return AoStatus::InvalidOperation(format!("{}", vm.ca));
                }
            }
            AoOpCode::SHL(src) => {
                let res = vm.ca.clone() << src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::SHR(src) => {
                let res = vm.ca.clone() >> src.get_value(vm);
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }

            AoOpCode::EQU(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (left, right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left == right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left == right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left == right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left == right);
                    }
                    _ => vm.ca = AoType::AoBool(false),
                }
            }
            AoOpCode::NEQ(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (left, right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left != right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left != right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left != right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left != right);
                    }
                    _ => vm.ca = AoType::AoBool(true),
                }
            }
            AoOpCode::GT(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (&left, &right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left > right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left > right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left > right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left > right);
                    }
                    _ => return AoStatus::InvalidOperation(format!("{} > {}", left, right)),
                }
            }
            AoOpCode::LT(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (&left, &right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left < right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left < right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left < right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left < right);
                    }
                    _ => return AoStatus::InvalidOperation(format!("{} < {}", left, right)),
                }
            }
            AoOpCode::GE(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (&left, &right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left >= right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left >= right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left >= right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left >= right);
                    }
                    _ => return AoStatus::InvalidOperation(format!("{} >= {}", left, right)),
                }
            }
            AoOpCode::LE(src) => {
                let left = vm.ca.clone();
                let right = src.get_value(vm);

                match (&left, &right) {
                    (AoType::AoBool(left), AoType::AoBool(right)) => {
                        vm.ca = AoType::AoBool(left <= right);
                    }
                    (AoType::AoInt(left), AoType::AoInt(right)) => {
                        vm.ca = AoType::AoBool(left <= right);
                    }
                    (AoType::AoFloat(left), AoType::AoFloat(right)) => {
                        vm.ca = AoType::AoBool(left <= right);
                    }
                    (AoType::AoString(left), AoType::AoString(right)) => {
                        vm.ca = AoType::AoBool(left <= right);
                    }
                    _ => return AoStatus::InvalidOperation(format!("{} <= {}", left, right)),
                }
            }

            AoOpCode::CSI => match &vm.ca {
                AoType::AoBool(b) => vm.ca = AoType::AoInt(if *b { 1 } else { 0 }),
                AoType::AoInt(_) => (),
                AoType::AoFloat(f) => vm.ca = AoType::AoInt(*f as i32),
                AoType::AoPtr(p) => vm.ca = AoType::AoInt(*p as i32),
                AoType::AoString(s) => vm.ca = AoType::AoInt(s.parse::<i32>().unwrap_or(0)),
            },
            AoOpCode::CSF => match &vm.ca {
                AoType::AoBool(b) => vm.ca = AoType::AoFloat(if *b { 1.0 } else { 0.0 }),
                AoType::AoInt(i) => vm.ca = AoType::AoFloat(*i as f32),
                AoType::AoFloat(_) => (),
                AoType::AoPtr(p) => vm.ca = AoType::AoFloat(*p as f32),
                AoType::AoString(s) => vm.ca = AoType::AoFloat(s.parse::<f32>().unwrap_or(0.0)),
            },
            AoOpCode::CSP => match &vm.ca {
                AoType::AoBool(b) => vm.ca = AoType::AoPtr(if *b { 1 } else { 0 }),
                AoType::AoInt(i) => vm.ca = AoType::AoPtr(*i as u32),
                AoType::AoFloat(f) => vm.ca = AoType::AoPtr(*f as u32),
                AoType::AoPtr(_) => (),
                AoType::AoString(s) => vm.ca = AoType::AoPtr(s.parse::<u32>().unwrap_or(0)),
            },
            AoOpCode::CSS => match &vm.ca {
                AoType::AoBool(b) => {
                    vm.ca = AoType::AoString(if *b {
                        String::from("true")
                    } else {
                        String::from("false")
                    })
                }
                AoType::AoInt(i) => vm.ca = AoType::AoString(i.to_string()),
                AoType::AoFloat(f) => vm.ca = AoType::AoString(f.to_string()),
                AoType::AoPtr(p) => vm.ca = AoType::AoString(p.to_string()),
                AoType::AoString(_) => (),
            },

            AoOpCode::ISB => {
                vm.ca = if let AoType::AoBool(_) = &vm.ca {
                    AoType::AoBool(true)
                } else {
                    AoType::AoBool(false)
                };
            }
            AoOpCode::ISI => {
                vm.ca = if let AoType::AoInt(_) = &vm.ca {
                    AoType::AoBool(true)
                } else {
                    AoType::AoBool(false)
                };
            }
            AoOpCode::ISF => {
                vm.ca = if let AoType::AoFloat(_) = &vm.ca {
                    AoType::AoBool(true)
                } else {
                    AoType::AoBool(false)
                };
            }
            AoOpCode::ISP => {
                vm.ca = if let AoType::AoPtr(_) = &vm.ca {
                    AoType::AoBool(true)
                } else {
                    AoType::AoBool(false)
                };
            }
            AoOpCode::ISS => {
                vm.ca = if let AoType::AoString(_) = &vm.ca {
                    AoType::AoBool(true)
                } else {
                    AoType::AoBool(false)
                };
            }

            AoOpCode::ARG(offset) => {
                vm.dp = vm.dsb + offset;
            }
            AoOpCode::CNF(argc) => {
                vm.dsb = vm.ds.len() as u32 - argc;
            }
        };

        AoStatus::Ok
    }
}

impl Display for AoOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoOpCode::NOP => write!(f, "nop"),

            AoOpCode::CALL(addr) => write!(f, "call {}", addr),
            AoOpCode::RET => write!(f, "ret"),
            AoOpCode::JMP(addr) => write!(f, "jmp {}", addr),
            AoOpCode::JT(addr) => write!(f, "jt {}", addr),
            AoOpCode::JF(addr) => write!(f, "jf {}", addr),

            AoOpCode::MOV(dst, src) => write!(f, "mov {},{}", dst, src),
            AoOpCode::INT(id) => write!(f, "int {}", id),

            AoOpCode::PUSH(src) => write!(f, "push {}", src),
            AoOpCode::POP(save) => {
                if *save {
                    write!(f, "pop ca")
                } else {
                    write!(f, "pop")
                }
            }

            AoOpCode::ADD(src) => write!(f, "add {}", src),
            AoOpCode::SUB(src) => write!(f, "sub {}", src),
            AoOpCode::MUL(src) => write!(f, "mul {}", src),
            AoOpCode::DIV(src) => write!(f, "div {}", src),
            AoOpCode::REM(src) => write!(f, "rem {}", src),
            AoOpCode::INC => write!(f, "inc"),
            AoOpCode::DEC => write!(f, "dec"),

            AoOpCode::AND(src) => write!(f, "and {}", src),
            AoOpCode::OR(src) => write!(f, "or {}", src),
            AoOpCode::XOR(src) => write!(f, "xor {}", src),
            AoOpCode::NOT => write!(f, "not"),

            AoOpCode::BAND(src) => write!(f, "band {}", src),
            AoOpCode::BOR(src) => write!(f, "bor {}", src),
            AoOpCode::BXOR(src) => write!(f, "bxor {}", src),
            AoOpCode::BNOT => write!(f, "bnot"),

            AoOpCode::SHL(src) => write!(f, "shl {}", src),
            AoOpCode::SHR(src) => write!(f, "shr {}", src),

            AoOpCode::EQU(src) => write!(f, "equ {}", src),
            AoOpCode::NEQ(src) => write!(f, "neq {}", src),
            AoOpCode::GT(src) => write!(f, "gt {}", src),
            AoOpCode::LT(src) => write!(f, "lt {}", src),
            AoOpCode::GE(src) => write!(f, "ge {}", src),
            AoOpCode::LE(src) => write!(f, "le {}", src),

            AoOpCode::CSI => write!(f, "csi"),
            AoOpCode::CSF => write!(f, "csf"),
            AoOpCode::CSP => write!(f, "csp"),
            AoOpCode::CSS => write!(f, "css"),

            AoOpCode::ISB => write!(f, "isb"),
            AoOpCode::ISI => write!(f, "isi"),
            AoOpCode::ISF => write!(f, "isf"),
            AoOpCode::ISP => write!(f, "isp"),
            AoOpCode::ISS => write!(f, "iss"),

            AoOpCode::ARG(offset) => write!(f, "arg {}", offset),
            AoOpCode::CNF(argc) => write!(f, "cnf {}", argc),
        }
    }
}

#[macro_export]
macro_rules! aoasm {
    ( nop ) => {
        AoOpCode::NOP
    };

    ( call $addr:expr ) => {
        AoOpCode::CALL($addr as u32)
    };
    ( ret ) => {
        AoOpCode::RET
    };
    ( jmp $addr:expr ) => {
        AoOpCode::JMP($addr as u32)
    };
    ( jt $addr:expr ) => {
        AoOpCode::JT($addr as u32)
    };
    ( jf $addr:expr ) => {
        AoOpCode::JF($addr as u32)
    };

    ( mov $dst:ident,$src:ident ) => {
        AoOpCode::MOV(
            AoArgLowerCase::$dst.to_aoarg(),
            AoArgLowerCase::$src.to_aoarg(),
        )
    };
    ( mov $dst:ident,$val:expr) => {
        AoOpCode::MOV(AoArgLowerCase::$dst.to_aoarg(), AoArg::from($val))
    };
    ( int $id:expr ) => {
        AoOpCode::INT($id as u8)
    };

    ( push $src:ident ) => {
        AoOpCode::PUSH(AoArgLowerCase::$src.to_aoarg())
    };
    ( push $val:expr ) => {
        AoOpCode::PUSH(AoArg::from($val))
    };
    ( pop ) => {
        AoOpCode::POP(false)
    };
    ( pop ca ) => {
        AoOpCode::POP(true)
    };

    ( add $src:ident ) => {
        AoOpCode::ADD(AoArgLowerCase::$src.to_aoarg())
    };
    ( add $val:expr ) => {
        AoOpCode::ADD(AoArg::from($val))
    };
    ( sub $src:ident ) => {
        AoOpCode::SUB(AoArgLowerCase::$src.to_aoarg())
    };
    ( sub $val:expr ) => {
        AoOpCode::SUB(AoArg::from($val))
    };
    ( mul $src:ident ) => {
        AoOpCode::MUL(AoArgLowerCase::$src.to_aoarg())
    };
    ( mul $val:expr ) => {
        AoOpCode::MUL(AoArg::from($val))
    };
    ( div $src:ident ) => {
        AoOpCode::DIV(AoArgLowerCase::$src.to_aoarg())
    };
    ( div $val:expr ) => {
        AoOpCode::DIV(AoArg::from($val))
    };
    ( rem $src:ident ) => {
        AoOpCode::REM(AoArgLowerCase::$src.to_aoarg())
    };
    ( rem $val:expr ) => {
        AoOpCode::REM(AoArg::from($val))
    };
    ( inc ) => {
        AoOpCode::INC
    };
    ( dec ) => {
        AoOpCode::DEC
    };

    ( and $src:ident ) => {
        AoOpCode::AND(AoArgLowerCase::$src.to_aoarg())
    };
    ( and $val:expr ) => {
        AoOpCode::AND(AoArg::from($val))
    };
    ( or $src:ident ) => {
        AoOpCode::OR(AoArgLowerCase::$src.to_aoarg())
    };
    ( or $val:expr ) => {
        AoOpCode::OR(AoArg::from($val))
    };
    ( xor $src:ident ) => {
        AoOpCode::XOR(AoArgLowerCase::$src.to_aoarg())
    };
    ( xor $val:expr ) => {
        AoOpCode::XOR(AoArg::from($val))
    };
    ( not ) => {
        AoOpCode::NOT
    };

    ( band $src:ident ) => {
        AoOpCode::BAND(AoArgLowerCase::$src.to_aoarg())
    };
    ( band $val:expr ) => {
        AoOpCode::BAND(AoArg::from($val))
    };
    ( bor $src:ident ) => {
        AoOpCode::BOR(AoArgLowerCase::$src.to_aoarg())
    };
    ( bor $val:expr ) => {
        AoOpCode::BOR(AoArg::from($val))
    };
    ( bxor $src:ident ) => {
        AoOpCode::BXOR(AoArgLowerCase::$src.to_aoarg())
    };
    ( bxor $val:expr ) => {
        AoOpCode::BXOR(AoArg::from($val))
    };
    ( bnot ) => {
        AoOpCode::BNOT
    };

    ( shl $src:ident ) => {
        AoOpCode::SHL(AoArgLowerCase::$src.to_aoarg())
    };
    ( shl $val:expr ) => {
        AoOpCode::SHL(AoArg::from($val))
    };
    ( shr $src:ident ) => {
        AoOpCode::SHR(AoArgLowerCase::$src.to_aoarg())
    };
    ( shr $val:expr ) => {
        AoOpCode::SHR(AoArg::from($val))
    };

    ( equ $src:ident ) => {
        AoOpCode::EQU(AoArgLowerCase::$src.to_aoarg())
    };
    ( equ $val:expr ) => {
        AoOpCode::EQU(AoArg::from($val))
    };
    ( neq $src:ident ) => {
        AoOpCode::NEQ(AoArgLowerCase::$src.to_aoarg())
    };
    ( neq $val:expr ) => {
        AoOpCode::NEQ(AoArg::from($val))
    };
    ( gt $src:ident ) => {
        AoOpCode::GT(AoArgLowerCase::$src.to_aoarg())
    };
    ( gt $val:expr ) => {
        AoOpCode::GT(AoArg::from($val))
    };
    ( lt $src:ident ) => {
        AoOpCode::LT(AoArgLowerCase::$src.to_aoarg())
    };
    ( lt $val:expr ) => {
        AoOpCode::LT(AoArg::from($val))
    };
    ( ge $src:ident ) => {
        AoOpCode::GE(AoArgLowerCase::$src.to_aoarg())
    };
    ( ge $val:expr ) => {
        AoOpCode::GE(AoArg::from($val))
    };
    ( le $src:ident ) => {
        AoOpCode::LE(AoArgLowerCase::$src.to_aoarg())
    };
    ( le $val:expr ) => {
        AoOpCode::LE(AoArg::from($val))
    };

    ( csi ) => {
        AoOpCode::CSI
    };
    ( csf ) => {
        AoOpCode::CSF
    };
    ( csp ) => {
        AoOpCode::CSP
    };
    ( css ) => {
        AoOpCode::CSS
    };

    ( isb ) => {
        AoOpCode::ISB
    };
    ( isi ) => {
        AoOpCode::ISI
    };
    ( isf ) => {
        AoOpCode::ISF
    };
    ( isp ) => {
        AoOpCode::ISP
    };
    ( iss ) => {
        AoOpCode::ISS
    };

    ( arg $offset:expr ) => {
        AoOpCode::ARG($offset as u32)
    };
    ( cnf $argc:expr ) => {
        AoOpCode::CNF($argc as u32)
    };
}
