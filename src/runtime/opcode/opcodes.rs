use std::fmt::Display;

use super::AoArg;
use crate::AoStatus;
use crate::AoType;
use crate::AoVM;

#[allow(non_camel_case_types)]
pub enum OpcodeArgType {
    NoArg,
    u8(u8),
    i32(i32),
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

macro_rules! impl_disp {
    ( $t:ty, $d:expr ) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $d)
            }
        }
    };
    ( $t:ty, $d:expr, $($f:ident),* ) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $d, $(self.$f),*)
            }
        }
    };
    ( $t:ty, $f:ident, $mt:expr, $mf:expr) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.$f {
                    write!(f, $mt)
                } else {
                    write!(f, $mf)
                }
            }
        }
    };
}

macro_rules! impl_ao_opcode {
    ( $t:tt, (&$s:ident, $v:ident) $e:block ) => {
        #[allow(unused_variables)]
        impl AoOpcode for $t {
            fn execute(&$s, $v: &mut AoVM) -> AoStatus { $e AoStatus::Ok }
        }
    };
}

macro_rules! opcode {
    ( $t:tt, $c:expr, $d:expr, (&$s:ident, $v:ident) $e:block ) => {
        #[derive(Clone)]
        pub struct $t;
        impl_disp!($t, $d);
        impl_ao_opcode!( $t, (&$s, $v) { $e });
        impl Serializable for $t {
            fn get_id(&self) -> u8 {
                $c
            }
            fn get_args(&self) -> OpcodeArgType {
                OpcodeArgType::NoArg
            }
            fn set_args(&mut self, _: OpcodeArgType) {
            }
        }
    };
    ( $t:tt, $c:expr, $d:expr, $f:ident, (&$s:ident, $v:ident) $e:block ) => {
        opcode!($t, $c, $d, AoArg $f, (&$s, $v) $e);
    };
    ( $t:tt, $c:expr, $d:expr, $ft:tt $f:ident, (&$s:ident, $v:ident) $e:block ) => {
        #[derive(Clone)]
        pub struct $t {
            pub $f: $ft,
        }
        impl_disp!($t, $d, $f);
        impl_ao_opcode!( $t, (&$s, $v) { $e });
        impl Serializable for $t {
            fn get_id(&self) -> u8 {
                $c
            }
            fn get_args(&self) -> OpcodeArgType {
                OpcodeArgType::$ft(self.$f.clone())
            }
            fn set_args(&mut self, args: OpcodeArgType) {
                if let OpcodeArgType::$ft($f) = args {
                    self.$f = $f;
                }
            }
        }
    };
    ( $t:tt, $c:expr, $mt:expr, $mf:expr, bool $f:ident, (&$s:ident, $v:ident) $e:block ) => {
        #[derive(Clone)]
        pub struct $t {
            pub $f: bool,
        }
        impl_disp!($t, $f, $mt, $mf);
        impl_ao_opcode!( $t, (&$s, $v) { $e });
        impl Serializable for $t {
            fn get_id(&self) -> u8 {
                $c
            }
            fn get_args(&self) -> OpcodeArgType {
                OpcodeArgType::bool(self.$f.clone())
            }
            fn set_args(&mut self, args: OpcodeArgType){
                if let OpcodeArgType::bool($f) = args {
                    self.$f = $f;
                }
            }
        }
    };
    ( $t:tt, $c:expr, $d:expr, $f1:ident, $f2:ident, (&$s:ident, $v:ident) $e:block ) => {
        #[derive(Clone)]
        pub struct $t {
            pub $f1: AoArg,
            pub $f2: AoArg,
        }
        impl_disp!($t, $d, $f1, $f2);
        impl_ao_opcode!( $t, (&$s, $v) { $e });
        impl Serializable for $t {
            fn get_id(&self) -> u8 {
                $c
            }
            fn get_args(&self) -> OpcodeArgType {
                OpcodeArgType::AoArg2(self.$f1.clone(), self.$f2.clone())
            }
            fn set_args(&mut self, args: OpcodeArgType){
                if let OpcodeArgType::AoArg2($f1, $f2) = args {
                    self.$f1 = $f1;
                    self.$f2 = $f2;
                }
            }
        }
    };
}

opcode!(Nop, 0x00, "nop", (&self, _vm) {});

opcode!(Call, 0x10, "call {}", u32 addr, (&self, vm) {
    if vm.cs.len() >= 100000 {
        return AoStatus::CallStackOverflow;
    }
    vm.cs.push(vm.pc);
    vm.pc = self.addr;
});

opcode!(Ret, 0x11, "ret", (&self, vm) {
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
});

opcode!(Jmp, 0x12, "jmp {}", i32 addr, (&self, vm) {
    vm.pc = (vm.pc as i32 + self.addr - 1) as u32;
});

opcode!(Jmpa, 0x13, "jmpa {}", u32 addr, (&self, vm) {
    vm.pc = self.addr;
});

opcode!(Jt, 0x14, "jt {}", i32 addr, (&self, vm) {
    if match vm.ca {
        AoType::AoBool(b) => b,
        AoType::AoInt(i) => i != 0,
        AoType::AoFloat(f) => f != 0.0,
        _ => false,
    } {
        vm.pc = (vm.pc as i32 + self.addr - 1) as u32;
    }
});

opcode!(Jta, 0x15, "jta {}", u32 addr, (&self, vm) {
    if match vm.ca {
        AoType::AoBool(b) => b,
        AoType::AoInt(i) => i != 0,
        AoType::AoFloat(f) => f != 0.0,
        _ => false,
    } {
        vm.pc = self.addr;
    }
});

opcode!(Jf, 0x16, "jf {}", i32 addr, (&self, vm) {
    if !match vm.ca {
        AoType::AoBool(b) => b,
        AoType::AoInt(i) => i != 0,
        AoType::AoFloat(f) => f != 0.0,
        _ => false,
    } {
        vm.pc = (vm.pc as i32 + self.addr - 1) as u32;
    }
});

opcode!(Jfa, 0x17, "jfa {}", u32 addr, (&self, vm) {
    if !match vm.ca {
        AoType::AoBool(b) => b,
        AoType::AoInt(i) => i != 0,
        AoType::AoFloat(f) => f != 0.0,
        _ => false,
    } {
        vm.pc = self.addr;
    }
});

opcode!(Mov, 0x20, "mov {},{}", dst, src, (&self, vm) {
    match self.dst.set_value(vm, self.src.get_value(vm)) {
        AoStatus::Ok => (),
        err => return err,
    }
});

opcode!(Int, 0x21, "int {}", u8 id, (&self, vm) {
    if self.id == 0 {
        return AoStatus::Exit;
    }

    let mut args: Vec<AoType> = Vec::new();
    for arg in vm.ds[vm.dsb as usize..].iter() {
        args.push(arg.clone());
    }

    if let Some(value) = (vm.interrupt)(self.id, args) {
        vm.ca = value;
    };

    let dsb = vm.dsb - 1;
    if let AoType::AoPtr(ptr) = vm.ds[dsb as usize] {
        vm.dsb = ptr;
        vm.ds.resize_with(dsb as usize, AoType::default);
    } else {
        return AoStatus::BadDataStack;
    }
});

opcode!(Push, 0x22, "push {}", src, (&self, vm) {
    if !vm.push(self.src.get_value(vm)) {
        return AoStatus::DataStackOverflow;
    }
});

opcode!(Pop, 0x23, "pop ca", "pop", bool to_ca, (&self, vm) {
    let value = vm.pop();
    if let Some(value) = value {
        if self.to_ca {
            vm.ca = value;
        }
    } else {
        return AoStatus::DataStackUnderflow;
    }
});

opcode!(Add, 0x30, "add {}", src, (&self, vm) {
    let res = vm.ca.clone() + self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Sub, 0x31, "sub {}", src, (&self, vm) {
    let res = vm.ca.clone() - self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Mul, 0x32, "mul {}", src, (&self, vm) {
    let res = vm.ca.clone() * self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Div, 0x33, "div {}", src, (&self, vm) {
    let res = vm.ca.clone() / self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Rem, 0x34, "rem {}", src, (&self, vm) {
    let res = vm.ca.clone() % self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Inc, 0x35, "inc", (&self, vm) {
    match vm.ca {
        AoType::AoInt(i) => {
            vm.ca = AoType::AoInt(i + 1);
        }
        AoType::AoFloat(f) => {
            vm.ca = AoType::AoFloat(f + 1.0);
        }
        _ => return AoStatus::InvalidOperation(format!("inc {}", vm.ca)),
    }
});

opcode!(Dec, 0x36, "dec", (&self, vm) {
    match vm.ca {
        AoType::AoInt(i) => {
            vm.ca = AoType::AoInt(i - 1);
        }
        AoType::AoFloat(f) => {
            vm.ca = AoType::AoFloat(f - 1.0);
        }
        _ => return AoStatus::InvalidOperation(format!("dec {}", vm.ca)),
    }
});

opcode!(Shl, 0x37, "shl {}", src, (&self, vm) {
    let res = vm.ca.clone() << self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Shr, 0x38, "shr {}", src, (&self, vm) {
    let res = vm.ca.clone() >> self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(And, 0x40, "and {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

    if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
        vm.ca = AoType::AoBool(*l && *r);
    } else {
        return AoStatus::InvalidOperation(format!("{} && {}", left, right));
    }
});

opcode!(Or, 0x41, "or {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

    if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
        vm.ca = AoType::AoBool(*l || *r);
    } else {
        return AoStatus::InvalidOperation(format!("{} || {}", left, right));
    }
});

opcode!(Xor, 0x42, "xor {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

    if let (AoType::AoBool(l), AoType::AoBool(r)) = (&left, &right) {
        vm.ca = AoType::AoBool(*l ^ *r);
    } else {
        return AoStatus::InvalidOperation(format!("{} ^ {}", left, right));
    }
});

opcode!(Not, 0x43, "not", (&self, vm) {
    if let AoType::AoBool(b) = vm.ca {
        vm.ca = AoType::AoBool(!b);
    } else {
        return AoStatus::InvalidOperation(format!("!{}", vm.ca));
    }
});

opcode!(Band, 0x44, "band {}", src, (&self, vm) {
    let res = vm.ca.clone() & self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Bor, 0x45, "bor {}", src, (&self, vm) {
    let res = vm.ca.clone() | self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Bxor, 0x46, "bxor {}", src, (&self, vm) {
    let res = vm.ca.clone() ^ self.src.get_value(vm);
    if let AoStatus::Return(value) = res {
        vm.ca = value;
    } else {
        return res;
    }
});

opcode!(Bnot, 0x47, "bnot", (&self, vm) {
    if let AoType::AoInt(i) = vm.ca {
        vm.ca = AoType::AoInt(!i);
    } else {
        return AoStatus::InvalidOperation(format!("~{}", vm.ca));
    }
});

opcode!(Equ, 0x50, "equ {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Neq, 0x51, "neq {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Gt, 0x52, "gt {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Lt, 0x53, "lt {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Ge, 0x54, "ge {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Le, 0x55, "le {}", src, (&self, vm) {
    let left = vm.ca.clone();
    let right = self.src.get_value(vm);

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
});

opcode!(Csi, 0x61, "csi", (&self, vm) {
    match &vm.ca {
        AoType::AoBool(b) => vm.ca = AoType::AoInt(if *b { 1 } else { 0 }),
        AoType::AoInt(_) => (),
        AoType::AoFloat(f) => vm.ca = AoType::AoInt(*f as i32),
        AoType::AoPtr(p) => vm.ca = AoType::AoInt(*p as i32),
        AoType::AoString(s) => vm.ca = AoType::AoInt(s.parse::<i32>().unwrap_or(0)),
    }
});

opcode!(Csf, 0x62, "csf", (&self, vm) {
    match &vm.ca {
        AoType::AoBool(b) => vm.ca = AoType::AoFloat(if *b { 1.0 } else { 0.0 }),
        AoType::AoInt(i) => vm.ca = AoType::AoFloat(*i as f32),
        AoType::AoFloat(_) => (),
        AoType::AoPtr(p) => vm.ca = AoType::AoFloat(*p as f32),
        AoType::AoString(s) => vm.ca = AoType::AoFloat(s.parse::<f32>().unwrap_or(0.0)),
    }
});

opcode!(Csp, 0x63, "csp", (&self, vm) {
    match &vm.ca {
        AoType::AoBool(b) => vm.ca = AoType::AoPtr(if *b { 1 } else { 0 }),
        AoType::AoInt(i) => vm.ca = AoType::AoPtr(*i as u32),
        AoType::AoFloat(f) => vm.ca = AoType::AoPtr(*f as u32),
        AoType::AoPtr(_) => (),
        AoType::AoString(s) => vm.ca = AoType::AoPtr(s.parse::<u32>().unwrap_or(0)),
    }
});

opcode!(Css, 0x64, "css", (&self, vm) {
    match &vm.ca {
        AoType::AoBool(b) => vm.ca = AoType::AoString(if *b {
            "true".to_string()
        } else {
            "false".to_string()
        }),
        AoType::AoInt(i) => vm.ca = AoType::AoString(i.to_string()),
        AoType::AoFloat(f) => vm.ca = AoType::AoString(f.to_string()),
        AoType::AoPtr(p) => vm.ca = AoType::AoString(p.to_string()),
        AoType::AoString(_) => (),
    }
});

opcode!(Isb, 0x68, "isb", (&self, vm) {
    vm.ca = if let AoType::AoBool(_) = &vm.ca {
        AoType::AoBool(true)
    } else {
        AoType::AoBool(false)
    };
});

opcode!(Isi, 0x69, "isi", (&self, vm) {
    vm.ca = if let AoType::AoInt(_) = &vm.ca {
        AoType::AoBool(true)
    } else {
        AoType::AoBool(false)
    };
});

opcode!(Isf, 0x6A, "isf", (&self, vm) {
    vm.ca = if let AoType::AoFloat(_) = &vm.ca {
        AoType::AoBool(true)
    } else {
        AoType::AoBool(false)
    };
});

opcode!(Isp, 0x6B, "isp", (&self, vm) {
    vm.ca = if let AoType::AoPtr(_) = &vm.ca {
        AoType::AoBool(true)
    } else {
        AoType::AoBool(false)
    };
});

opcode!(Iss, 0x6C, "iss", (&self, vm) {
    vm.ca = if let AoType::AoString(_) = &vm.ca {
        AoType::AoBool(true)
    } else {
        AoType::AoBool(false)
    };
});

opcode!(Arg, 0x70, "arg {}", u32 offset, (&self, vm) {
    vm.dp = vm.dsb + self.offset;
});

opcode!(Cnf, 0x71, "cnf {}", u32 argc, (&self, vm) {
    vm.dsb = vm.ds.len() as u32 - self.argc;
});

pub fn create_opcode_by_id(id: u8) -> Option<Box<dyn AoOpcode>> {
    match id {
        0x00 => Some(Box::new(Nop)),

        0x10 => Some(Box::new(Call { addr: 0 })),
        0x11 => Some(Box::new(Ret)),
        0x12 => Some(Box::new(Jmp { addr: 0 })),
        0x13 => Some(Box::new(Jmpa { addr: 0 })),
        0x14 => Some(Box::new(Jt { addr: 0 })),
        0x15 => Some(Box::new(Jta { addr: 0 })),
        0x16 => Some(Box::new(Jf { addr: 0 })),
        0x17 => Some(Box::new(Jfa { addr: 0 })),

        0x20 => Some(Box::new(Mov {
            src: AoArg::CA,
            dst: AoArg::CA,
        })),
        0x21 => Some(Box::new(Int { id: 0 })),
        0x22 => Some(Box::new(Push { src: AoArg::CA })),
        0x23 => Some(Box::new(Pop { to_ca: false })),

        0x30 => Some(Box::new(Add { src: AoArg::CA })),
        0x31 => Some(Box::new(Sub { src: AoArg::CA })),
        0x32 => Some(Box::new(Mul { src: AoArg::CA })),
        0x33 => Some(Box::new(Div { src: AoArg::CA })),
        0x34 => Some(Box::new(Rem { src: AoArg::CA })),
        0x35 => Some(Box::new(Inc)),
        0x36 => Some(Box::new(Dec)),
        0x37 => Some(Box::new(Shl { src: AoArg::CA })),
        0x38 => Some(Box::new(Shr { src: AoArg::CA })),

        0x40 => Some(Box::new(And { src: AoArg::CA })),
        0x41 => Some(Box::new(Or { src: AoArg::CA })),
        0x42 => Some(Box::new(Xor { src: AoArg::CA })),
        0x43 => Some(Box::new(Not)),
        0x44 => Some(Box::new(Band { src: AoArg::CA })),
        0x45 => Some(Box::new(Bor { src: AoArg::CA })),
        0x46 => Some(Box::new(Bxor { src: AoArg::CA })),
        0x47 => Some(Box::new(Bnot)),

        0x50 => Some(Box::new(Equ { src: AoArg::CA })),
        0x51 => Some(Box::new(Neq { src: AoArg::CA })),
        0x52 => Some(Box::new(Gt { src: AoArg::CA })),
        0x53 => Some(Box::new(Lt { src: AoArg::CA })),
        0x54 => Some(Box::new(Ge { src: AoArg::CA })),
        0x55 => Some(Box::new(Le { src: AoArg::CA })),

        0x61 => Some(Box::new(Csi)),
        0x62 => Some(Box::new(Csf)),
        0x63 => Some(Box::new(Csp)),
        0x64 => Some(Box::new(Css)),
        0x68 => Some(Box::new(Isb)),
        0x69 => Some(Box::new(Isi)),
        0x6A => Some(Box::new(Isf)),
        0x6B => Some(Box::new(Isp)),
        0x6C => Some(Box::new(Iss)),

        0x70 => Some(Box::new(Arg { offset: 0 })),
        0x71 => Some(Box::new(Cnf { argc: 0 })),

        _ => None,
    }
}
