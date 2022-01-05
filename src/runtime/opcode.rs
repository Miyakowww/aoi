use super::status::AoStatus;
use super::types::*;
use super::vm::AoVM;
use std::fmt::Display;

pub enum AoArg {
    DSB,
    DST,
    PC,
    DP,
    CA,
    DS,
    GVS,
    Imm(AoType),
}

impl AoArg {
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
    fn from(t: bool) -> AoArg {
        AoArg::Imm(AoType::AoBool(t))
    }
}

impl From<i32> for AoArg {
    fn from(t: i32) -> AoArg {
        AoArg::Imm(AoType::AoInt(t))
    }
}

impl From<f32> for AoArg {
    fn from(t: f32) -> AoArg {
        AoArg::Imm(AoType::AoFloat(t))
    }
}

impl From<u32> for AoArg {
    fn from(t: u32) -> AoArg {
        AoArg::Imm(AoType::AoPtr(t))
    }
}

impl From<String> for AoArg {
    fn from(t: String) -> AoArg {
        AoArg::Imm(AoType::AoString(t))
    }
}

impl From<&str> for AoArg {
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

pub enum AoOpCode {
    NOP,

    CALL(u32),
    RET,
    JMP(u32),
    JT(u32),
    JF(u32),

    MOV(AoArg, AoArg),
    INT(u8),

    PUSH(AoArg),
    POP(bool),

    ADD(AoArg),
    SUB(AoArg),
    MUL(AoArg),
    DIV(AoArg),
    MOD(AoArg),
    INC,
    DEC,

    AND(AoArg),
    OR(AoArg),
    XOR(AoArg),
    NOT,

    BAND(AoArg),
    BOR(AoArg),
    BXOR(AoArg),
    BNOT,

    SHL(AoArg),
    SHR(AoArg),

    EQU(AoArg),
    NEQ(AoArg),
    GT(AoArg),
    LT(AoArg),
    GE(AoArg),
    LE(AoArg),

    CSI,
    CSF,
    CSP,
    CSS,

    ISB,
    ISI,
    ISF,
    ISP,
    ISS,

    ARG(u32),
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
                let res = BIN_OPER_ADD.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::SUB(src) => {
                let res = BIN_OPER_SUB.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::MUL(src) => {
                let res = BIN_OPER_MUL.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::DIV(src) => {
                let res = BIN_OPER_DIV.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::MOD(src) => {
                let res = BIN_OPER_MOD.apply(vm.ca.clone(), src.get_value(vm));
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
                let res = BIN_OPER_BAND.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::BOR(src) => {
                let res = BIN_OPER_BOR.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::BXOR(src) => {
                let res = BIN_OPER_BXOR.apply(vm.ca.clone(), src.get_value(vm));
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
                let res = BIN_OPER_SHL.apply(vm.ca.clone(), src.get_value(vm));
                if let AoStatus::Return(value) = res {
                    vm.ca = value;
                } else {
                    return res;
                }
            }
            AoOpCode::SHR(src) => {
                let res = BIN_OPER_SHR.apply(vm.ca.clone(), src.get_value(vm));
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
        };

        AoStatus::Ok
    }
}

impl Display for AoOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoOpCode::NOP => write!(f, "nop"),

            AoOpCode::CALL(target) => write!(f, "call {}", target),
            AoOpCode::RET => write!(f, "ret"),
            AoOpCode::JMP(target) => write!(f, "jmp {}", target),
            AoOpCode::JT(target) => write!(f, "jt {}", target),
            AoOpCode::JF(target) => write!(f, "jf {}", target),

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
            AoOpCode::MOD(src) => write!(f, "mod {}", src),
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
        }
    }
}
