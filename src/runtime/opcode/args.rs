use std::fmt::Display;

use crate::AoStatus;
use crate::AoType;
use crate::AoVM;

/// The argument of the opcode.
#[derive(Debug, PartialEq, Clone)]
pub enum AoArg {
    /// Program counter.
    PC,
    /// Pointer for accessing data.
    DP,
    /// Pointer for accessing memory.
    MP,
    /// Pointer to the bottom of the stack frame.
    DSB,
    /// Pointer to the top of the stack.
    DST,
    /// Registers for calculation.
    CA,
    CB,
    /// Stack.
    DS,
    /// Memory.
    MEM,
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
    /// let mut vm = AoVM::default();
    /// assert_eq!(AoArg::CA.get_value(&mut vm), AoType::default());
    /// ```
    pub fn get_value(&self, vm: &AoVM) -> AoType {
        match self {
            AoArg::PC => AoType::AoPtr(vm.pc),
            AoArg::DP => AoType::AoPtr(vm.dp),
            AoArg::MP => AoType::AoPtr(vm.mp),
            AoArg::DSB => AoType::AoPtr(vm.dsb as u32),
            AoArg::DST => AoType::AoPtr(vm.ds.len() as u32),
            AoArg::CA => vm.ca.clone(),
            AoArg::CB => vm.cb.clone(),
            AoArg::DS => vm.ds[vm.dp as usize].clone(),
            AoArg::MEM => vm.mem.get(vm.mp),
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
    /// let mut vm = AoVM::default();
    /// AoArg::CA.set_value(&mut vm, AoType::AoPtr(0x12345678));
    /// assert_eq!(vm.ca, AoType::AoPtr(0x12345678));
    /// ```
    pub fn set_value(&self, vm: &mut AoVM, value: AoType) -> AoStatus {
        match self {
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
            AoArg::MP => match value {
                AoType::AoPtr(p) => {
                    vm.mp = p;
                    AoStatus::Ok
                }
                _ => {
                    AoStatus::SetValueInvalidType("cannot set MP to non-pointer value".to_string())
                }
            },
            AoArg::DSB => match value {
                AoType::AoPtr(p) => {
                    vm.dsb = p;
                    AoStatus::Ok
                }
                _ => AoStatus::SetValueInvalidType(format!("cannot set DSB to {}", value)),
            },
            AoArg::DST => AoStatus::SetValueInvalidTarget("cannot set DST".to_string()),
            AoArg::CA => {
                vm.ca = value;
                AoStatus::Ok
            }
            AoArg::CB => {
                vm.cb = value;
                AoStatus::Ok
            }
            AoArg::DS => {
                vm.ds[vm.dp as usize] = value;
                AoStatus::Ok
            }
            AoArg::MEM => {
                vm.mem.set(vm.mp, value);
                AoStatus::Ok
            }
            AoArg::Imm(_) => {
                AoStatus::SetValueInvalidTarget("cannot set immediate value".to_string())
            }
        }
    }
}

impl Display for AoArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoArg::PC => write!(f, "pc"),
            AoArg::DP => write!(f, "dp"),
            AoArg::MP => write!(f, "mp"),
            AoArg::DSB => write!(f, "dsb"),
            AoArg::DST => write!(f, "dst"),
            AoArg::CA => write!(f, "ca"),
            AoArg::CB => write!(f, "cb"),
            AoArg::DS => write!(f, "ds"),
            AoArg::MEM => write!(f, "mem"),
            AoArg::Imm(v) => write!(f, "{}", v),
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

#[allow(non_camel_case_types)]
pub enum AoArgLowerCase {
    pc,
    dp,
    mp,
    dsb,
    dst,
    ca,
    cb,
    ds,
    mem,
    imm(AoType),
}

impl AoArgLowerCase {
    pub fn to_aoarg(&self) -> AoArg {
        match self {
            AoArgLowerCase::pc => AoArg::PC,
            AoArgLowerCase::dp => AoArg::DP,
            AoArgLowerCase::mp => AoArg::MP,
            AoArgLowerCase::dsb => AoArg::DSB,
            AoArgLowerCase::dst => AoArg::DST,
            AoArgLowerCase::ca => AoArg::CA,
            AoArgLowerCase::cb => AoArg::CB,
            AoArgLowerCase::ds => AoArg::DS,
            AoArgLowerCase::mem => AoArg::MEM,
            AoArgLowerCase::imm(v) => AoArg::Imm(v.clone()),
        }
    }
}
