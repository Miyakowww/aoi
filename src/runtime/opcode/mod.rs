pub mod opcodes;

use std::fmt::Display;

use super::status::AoStatus;
use super::types::*;
use super::vm::AoVM;

/// The argument of the opcode.
#[derive(Debug, PartialEq, Clone)]
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

#[allow(non_camel_case_types)]
pub enum OpcodeArgType {
    NoArg,
    u8(u8),
    u32(u32),
    bool(bool),
    AoArg(AoArg),
    AoArg2(AoArg, AoArg),
}

pub trait Serializable {
    fn get_id(&self) -> u8;
    fn get_args(&self) -> OpcodeArgType;
    fn set_args(&mut self, args: OpcodeArgType);
}

pub trait AoOpcode: Display + Serializable {
    fn execute(&self, vm: &mut AoVM) -> AoStatus;
}

#[macro_export]
macro_rules! ao_asm {
    ( nop ) => {
        Box::new(opcodes::Nop)
    };

    ( call $addr:expr ) => {
        Box::new(opcodes::Call { addr: $addr })
    };
    ( ret ) => {
        Box::new(opcodes::Ret)
    };
    ( jmp $addr:expr ) => {
        Box::new(opcodes::Jmp { addr: $addr })
    };
    ( jt $addr:expr ) => {
        Box::new(opcodes::Jt { addr: $addr })
    };
    ( jf $addr:expr ) => {
        Box::new(opcodes::Jf { addr: $addr })
    };

    ( mov $dst:ident,$src:ident ) => {
        Box::new(opcodes::Mov {
            dst: AoArgLowerCase::$dst.to_aoarg(),
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( mov $dst:ident,$val:expr) => {
        Box::new(opcodes::Mov {
            dst: AoArgLowerCase::$dst.to_aoarg(),
            src: AoArg::from($val),
        })
    };
    ( int $id:expr ) => {
        Box::new(opcodes::Int { id: $id as u8 })
    };
    ( push $src:ident ) => {
        Box::new(opcodes::Push {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( push $val:expr ) => {
        Box::new(opcodes::Push {
            src: AoArg::from($val),
        })
    };
    ( pop ) => {
        Box::new(opcodes::Pop { to_ca: false })
    };
    ( pop ca ) => {
        Box::new(opcodes::Pop { to_ca: true })
    };

    ( add $src:ident ) => {
        Box::new(opcodes::Add {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( add $val:expr ) => {
        Box::new(opcodes::Add {
            src: AoArg::from($val),
        })
    };
    ( sub $src:ident ) => {
        Box::new(opcodes::Sub {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( sub $val:expr ) => {
        Box::new(opcodes::Sub {
            src: AoArg::from($val),
        })
    };
    ( mul $src:ident ) => {
        Box::new(opcodes::Mul {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( mul $val:expr ) => {
        Box::new(opcodes::Mul {
            src: AoArg::from($val),
        })
    };
    ( div $src:ident ) => {
        Box::new(opcodes::Div {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( div $val:expr ) => {
        Box::new(opcodes::Div {
            src: AoArg::from($val),
        })
    };
    ( rem $src:ident ) => {
        Box::new(opcodes::Rem {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( rem $val:expr ) => {
        Box::new(opcodes::Rem {
            src: AoArg::from($val),
        })
    };
    ( inc ) => {
        Box::new(opcodes::Inc)
    };
    ( dec ) => {
        Box::new(opcodes::Dec)
    };
    ( shl $src:ident ) => {
        Box::new(opcodes::Shl {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( shl $val:expr ) => {
        Box::new(opcodes::Shl {
            src: AoArg::from($val),
        })
    };
    ( shr $src:ident ) => {
        Box::new(opcodes::Shr {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( shr $val:expr ) => {
        Box::new(opcodes::Shr {
            src: AoArg::from($val),
        })
    };

    ( and $src:ident ) => {
        Box::new(opcodes::And {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( and $val:expr ) => {
        Box::new(opcodes::And {
            src: AoArg::from($val),
        })
    };
    ( or $src:ident ) => {
        Box::new(opcodes::Or {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( or $val:expr ) => {
        Box::new(opcodes::Or {
            src: AoArg::from($val),
        })
    };
    ( xor $src:ident ) => {
        Box::new(opcodes::Xor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( xor $val:expr ) => {
        Box::new(opcodes::Xor {
            src: AoArg::from($val),
        })
    };
    ( not ) => {
        Box::new(opcodes::Not)
    };
    ( band $src:ident ) => {
        Box::new(opcodes::Band {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( band $val:expr ) => {
        Box::new(opcodes::Band {
            src: AoArg::from($val),
        })
    };
    ( bor $src:ident ) => {
        Box::new(opcodes::Bor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( bor $val:expr ) => {
        Box::new(opcodes::Bor {
            src: AoArg::from($val),
        })
    };
    ( bxor $src:ident ) => {
        Box::new(opcodes::Bxor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( bxor $val:expr ) => {
        Box::new(opcodes::Bxor {
            src: AoArg::from($val),
        })
    };
    ( bnot ) => {
        Box::new(opcodes::Bnot)
    };

    ( equ $src:ident ) => {
        Box::new(opcodes::Equ {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( equ $val:expr ) => {
        Box::new(opcodes::Equ {
            src: AoArg::from($val),
        })
    };
    ( neq $src:ident ) => {
        Box::new(opcodes::Neq {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( neq $val:expr ) => {
        Box::new(opcodes::Neq {
            src: AoArg::from($val),
        })
    };
    ( gt $src:ident ) => {
        Box::new(opcodes::Gt {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( gt $val:expr ) => {
        Box::new(opcodes::Gt {
            src: AoArg::from($val),
        })
    };
    ( lt $src:ident ) => {
        Box::new(opcodes::Lt {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( lt $val:expr ) => {
        Box::new(opcodes::Lt {
            src: AoArg::from($val),
        })
    };
    ( ge $src:ident ) => {
        Box::new(opcodes::Ge {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( ge $val:expr ) => {
        Box::new(opcodes::Ge {
            src: AoArg::from($val),
        })
    };
    ( le $src:ident ) => {
        Box::new(opcodes::Le {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( le $val:expr ) => {
        Box::new(opcodes::Le {
            src: AoArg::from($val),
        })
    };

    ( csi ) => {
        Box::new(opcodes::Csi)
    };
    ( csf ) => {
        Box::new(opcodes::Csf)
    };
    ( csp ) => {
        Box::new(opcodes::Csp)
    };
    ( css ) => {
        Box::new(opcodes::Css)
    };
    ( isb ) => {
        Box::new(opcodes::Isb)
    };
    ( isi ) => {
        Box::new(opcodes::Isi)
    };
    ( isf ) => {
        Box::new(opcodes::Isf)
    };
    ( isp ) => {
        Box::new(opcodes::Isp)
    };
    ( iss ) => {
        Box::new(opcodes::Iss)
    };

    ( arg $offset:expr ) => {
        Box::new(opcodes::Arg { offset: $offset })
    };
    ( cnf $argc:expr ) => {
        Box::new(opcodes::Cnf { argc: $argc })
    };
}

#[macro_export]
macro_rules! prog_muncher {
    ( $v:ident, ) => {};
    ( $v:ident, $asm:ident; $($tail:tt)* ) => {
        $v.push(ao_asm!($asm));
        prog_muncher!($v, $($tail)*)
    };
    ( $v:ident, $asm:ident $arg:ident; $($tail:tt)* ) => {
        $v.push(ao_asm!($asm $arg));
        prog_muncher!($v, $($tail)*)
    };
    ( $v:ident, $asm:ident $arg:expr; $($tail:tt)* ) => {
        $v.push(ao_asm!($asm $arg));
        prog_muncher!($v, $($tail)*)
    };
    ( $v:ident, $asm:ident $arg1:ident,$arg2:ident; $($tail:tt)* ) => {
        $v.push(ao_asm!($asm $arg1,$arg2));
        prog_muncher!($v, $($tail)*)
    };
    ( $v:ident, $asm:ident $arg1:ident,$arg2:expr; $($tail:tt)* ) => {
        $v.push(ao_asm!($asm $arg1,$arg2));
        prog_muncher!($v, $($tail)*)
    };
}

#[macro_export]
macro_rules! ao_program {
    ( $( $rest:tt )* ) => {{
        let mut program: aoi::AoProgram = vec![];
        prog_muncher!(program, $($rest)*);
        program
    }};
}
