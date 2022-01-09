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

impl From<bool> for AoArg {
    /// Creates an immediate argument from a boolean value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from(true), AoArg::Imm(AoType::AoBool(true)));
    /// ```
    fn from(t: bool) -> AoArg {
        AoArg::Imm(AoType::AoBool(t))
    }
}

impl From<i32> for AoArg {
    /// Creates an immediate argument from an integer value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from(123), AoArg::Imm(AoType::AoInt(123)));
    /// ```
    fn from(t: i32) -> AoArg {
        AoArg::Imm(AoType::AoInt(t))
    }
}

impl From<f32> for AoArg {
    /// Creates an immediate argument from a float value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from(123.0), AoArg::Imm(AoType::AoFloat(123.0)));
    /// ```
    fn from(t: f32) -> AoArg {
        AoArg::Imm(AoType::AoFloat(t))
    }
}

impl From<u32> for AoArg {
    /// Creates an immediate argument from a pointer value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from(0x12345678_u32), AoArg::Imm(AoType::AoPtr(0x12345678)));
    /// ```
    fn from(t: u32) -> AoArg {
        AoArg::Imm(AoType::AoPtr(t))
    }
}

impl From<String> for AoArg {
    /// Creates an immediate argument from a string value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from("hello".to_string()), AoArg::Imm(AoType::AoString("hello".to_string())));
    /// ```
    fn from(t: String) -> AoArg {
        AoArg::Imm(AoType::AoString(t))
    }
}

impl From<&str> for AoArg {
    /// Creates an immediate argument from a string value.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::opcode::AoArg;
    /// use aoi::runtime::types::AoType;
    ///
    /// assert_eq!(AoArg::from("hello"), AoArg::Imm(AoType::AoString("hello".to_string())));
    /// ```
    fn from(t: &str) -> AoArg {
        AoArg::Imm(AoType::AoString(t.to_string()))
    }
}

impl Display for AoArg {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoArg::DSB => write!(formatter, "dsb"),
            AoArg::DST => write!(formatter, "dst"),
            AoArg::PC => write!(formatter, "pc"),
            AoArg::DP => write!(formatter, "dp"),
            AoArg::CA => write!(formatter, "ca"),
            AoArg::DS => write!(formatter, "ds"),
            AoArg::GVS => write!(formatter, "gvs"),
            AoArg::Imm(value) => write!(formatter, "{}", value),
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
                if vm.cs.len() < 1 {
                    return AoStatus::CallStackUnderflow;
                }

                let dsb = vm.dsb - 1;
                if let AoType::AoPtr(ptr) = vm.ds[dsb as usize] {
                    vm.dsb = ptr;
                    vm.ds.resize_with(dsb as usize, || AoType::default());
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
                    vm.ds.resize_with(dsb as usize, || AoType::default());
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
