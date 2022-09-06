#[macro_export]
macro_rules! ao_asm {
    ( nop ) => {
        Box::new(opcodes::Nop)
    };

    ( call $addr:literal ) => {
        Box::new(opcodes::Call { addr: $addr })
    };
    ( ret ) => {
        Box::new(opcodes::Ret)
    };
    ( jmp $addr:expr ) => {
        Box::new(opcodes::Jmp { addr: $addr })
    };
    ( jmpa $addr:literal ) => {
        Box::new(opcodes::Jmpa { addr: $addr })
    };
    ( jt $addr:expr ) => {
        Box::new(opcodes::Jt { addr: $addr })
    };
    ( jta $addr:literal ) => {
        Box::new(opcodes::Jta { addr: $addr })
    };
    ( jf $addr:expr ) => {
        Box::new(opcodes::Jf { addr: $addr })
    };
    ( jfa $addr:literal ) => {
        Box::new(opcodes::Jfa { addr: $addr })
    };

    ( mov $dst:ident,$src:ident ) => {
        Box::new(opcodes::Mov {
            dst: AoArgLowerCase::$dst.to_aoarg(),
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( mov dp,$val:literal ) => {
        Box::new(opcodes::Mov {
            dst: AoArg::DP,
            src: AoArg::from($val as u32),
        })
    };
    ( mov mp,$val:literal ) => {
        Box::new(opcodes::Mov {
            dst: AoArg::MP,
            src: AoArg::from($val as u32),
        })
    };
    ( mov $dst:ident,$val:literal ) => {
        Box::new(opcodes::Mov {
            dst: AoArgLowerCase::$dst.to_aoarg(),
            src: AoArg::from($val),
        })
    };
    ( int $id:literal ) => {
        Box::new(opcodes::Int { id: $id as u8 })
    };
    ( push $src:ident ) => {
        Box::new(opcodes::Push {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( push $val:literal ) => {
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
    ( add $val:literal ) => {
        Box::new(opcodes::Add {
            src: AoArg::from($val),
        })
    };
    ( sub $src:ident ) => {
        Box::new(opcodes::Sub {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( sub $val:literal ) => {
        Box::new(opcodes::Sub {
            src: AoArg::from($val),
        })
    };
    ( mul $src:ident ) => {
        Box::new(opcodes::Mul {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( mul $val:literal ) => {
        Box::new(opcodes::Mul {
            src: AoArg::from($val),
        })
    };
    ( div $src:ident ) => {
        Box::new(opcodes::Div {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( div $val:literal ) => {
        Box::new(opcodes::Div {
            src: AoArg::from($val),
        })
    };
    ( rem $src:ident ) => {
        Box::new(opcodes::Rem {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( rem $val:literal ) => {
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
    ( shl $val:literal ) => {
        Box::new(opcodes::Shl {
            src: AoArg::from($val),
        })
    };
    ( shr $src:ident ) => {
        Box::new(opcodes::Shr {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( shr $val:literal ) => {
        Box::new(opcodes::Shr {
            src: AoArg::from($val),
        })
    };

    ( and $src:ident ) => {
        Box::new(opcodes::And {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( and $val:literal ) => {
        Box::new(opcodes::And {
            src: AoArg::from($val),
        })
    };
    ( or $src:ident ) => {
        Box::new(opcodes::Or {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( or $val:literal ) => {
        Box::new(opcodes::Or {
            src: AoArg::from($val),
        })
    };
    ( xor $src:ident ) => {
        Box::new(opcodes::Xor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( xor $val:literal ) => {
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
    ( band $val:literal ) => {
        Box::new(opcodes::Band {
            src: AoArg::from($val),
        })
    };
    ( bor $src:ident ) => {
        Box::new(opcodes::Bor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( bor $val:literal ) => {
        Box::new(opcodes::Bor {
            src: AoArg::from($val),
        })
    };
    ( bxor $src:ident ) => {
        Box::new(opcodes::Bxor {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( bxor $val:literal ) => {
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
    ( equ $val:literal ) => {
        Box::new(opcodes::Equ {
            src: AoArg::from($val),
        })
    };
    ( neq $src:ident ) => {
        Box::new(opcodes::Neq {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( neq $val:literal ) => {
        Box::new(opcodes::Neq {
            src: AoArg::from($val),
        })
    };
    ( gt $src:ident ) => {
        Box::new(opcodes::Gt {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( gt $val:literal ) => {
        Box::new(opcodes::Gt {
            src: AoArg::from($val),
        })
    };
    ( lt $src:ident ) => {
        Box::new(opcodes::Lt {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( lt $val:literal ) => {
        Box::new(opcodes::Lt {
            src: AoArg::from($val),
        })
    };
    ( ge $src:ident ) => {
        Box::new(opcodes::Ge {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( ge $val:literal ) => {
        Box::new(opcodes::Ge {
            src: AoArg::from($val),
        })
    };
    ( le $src:ident ) => {
        Box::new(opcodes::Le {
            src: AoArgLowerCase::$src.to_aoarg(),
        })
    };
    ( le $val:literal ) => {
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

    ( arg $offset:literal ) => {
        Box::new(opcodes::Arg { offset: $offset })
    };
    ( cnf $argc:literal ) => {
        Box::new(opcodes::Cnf { argc: $argc })
    };
}

#[macro_export]
macro_rules! ao_program {
    (@muncher $v:ident, ) => {};

    // two args
    (@muncher $v:ident, mov $dst:tt,$src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(mov $dst,$src));
        ao_program!(@muncher $v, $($tail)*)
    };

    // no args
    (@muncher $v:ident, nop $($tail:tt)* ) => {
        $v.push(ao_asm!(nop));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, ret $($tail:tt)* ) => {
        $v.push(ao_asm!(ret));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, pop ca $($tail:tt)* ) => {
        $v.push(ao_asm!(pop ca));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, pop $($tail:tt)* ) => {
        $v.push(ao_asm!(pop));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, inc $($tail:tt)* ) => {
        $v.push(ao_asm!(inc));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, dec $($tail:tt)* ) => {
        $v.push(ao_asm!(dec));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, not $($tail:tt)* ) => {
        $v.push(ao_asm!(not));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, bnot $($tail:tt)* ) => {
        $v.push(ao_asm!(bnot));
        ao_program!(@muncher $v, $($tail)*)
    };

    (@muncher $v:ident, csi $($tail:tt)* ) => {
        $v.push(ao_asm!(csi));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, csf $($tail:tt)* ) => {
        $v.push(ao_asm!(csf));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, csp $($tail:tt)* ) => {
        $v.push(ao_asm!(csp));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, css $($tail:tt)* ) => {
        $v.push(ao_asm!(css));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, isb $($tail:tt)* ) => {
        $v.push(ao_asm!(isb));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, isi $($tail:tt)* ) => {
        $v.push(ao_asm!(isi));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, isf $($tail:tt)* ) => {
        $v.push(ao_asm!(isf));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, isp $($tail:tt)* ) => {
        $v.push(ao_asm!(isp));
        ao_program!(@muncher $v, $($tail)*)
    };
    (@muncher $v:ident, iss $($tail:tt)* ) => {
        $v.push(ao_asm!(iss));
        ao_program!(@muncher $v, $($tail)*)
    };

    // one args
    (@muncher $v:ident, $op:ident $arg:tt $($tail:tt)* ) => {
        $v.push(ao_asm!($op $arg));
        ao_program!(@muncher $v, $($tail)*)
    };

    ( $( $rest:tt )* ) => {{
        let mut program: aoi::AoProgram = vec![];
        ao_program!(@muncher program, $($rest)*);
        program
    }};
}
