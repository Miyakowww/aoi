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
macro_rules! asm_muncher {
    ( $v:ident, ) => {};
    ( $v:ident, nop $($tail:tt)* ) => {
        $v.push(ao_asm!(nop));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, call $addr:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(call $addr));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, ret $($tail:tt)* ) => {
        $v.push(ao_asm!(ret));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, jmp $addr:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(jmp $addr));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, jt $addr:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(jt $addr));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, jf $addr:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(jf $addr));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, mov $dst:tt,$src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(mov $dst,$src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, int $id:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(int $id));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, push $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(push $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, pop ca $($tail:tt)* ) => {
        $v.push(ao_asm!(pop ca));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, pop $($tail:tt)* ) => {
        $v.push(ao_asm!(pop));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, add $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(add $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, sub $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(sub $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, mul $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(mul $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, div $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(div $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, rem $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(rem $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, inc $($tail:tt)* ) => {
        $v.push(ao_asm!(inc));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, dec $($tail:tt)* ) => {
        $v.push(ao_asm!(dec));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, shl $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(shl $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, shr $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(shr $src));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, and $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(and $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, or $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(or $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, xor $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(xor $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, not $($tail:tt)* ) => {
        $v.push(ao_asm!(not));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, band $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(band $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, bor $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(bor $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, bxor $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(bxor $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, bnot $($tail:tt)* ) => {
        $v.push(ao_asm!(bnot));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, equ $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(equ $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, neq $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(neq $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, gt $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(gt $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, lt $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(lt $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, ge $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(ge $src));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, le $src:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(le $src));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, csi $($tail:tt)* ) => {
        $v.push(ao_asm!(csi));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, csf $($tail:tt)* ) => {
        $v.push(ao_asm!(csf));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, csp $($tail:tt)* ) => {
        $v.push(ao_asm!(csp));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, css $($tail:tt)* ) => {
        $v.push(ao_asm!(css));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, isb $($tail:tt)* ) => {
        $v.push(ao_asm!(isb));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, isi $($tail:tt)* ) => {
        $v.push(ao_asm!(isi));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, isf $($tail:tt)* ) => {
        $v.push(ao_asm!(isf));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, isp $($tail:tt)* ) => {
        $v.push(ao_asm!(isp));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, iss $($tail:tt)* ) => {
        $v.push(ao_asm!(iss));
        asm_muncher!($v, $($tail)*)
    };

    ( $v:ident, arg $offset:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(arg $offset));
        asm_muncher!($v, $($tail)*)
    };
    ( $v:ident, cnf $argc:tt $($tail:tt)* ) => {
        $v.push(ao_asm!(cnf $argc));
        asm_muncher!($v, $($tail)*)
    };
}

#[macro_export]
macro_rules! ao_program {
    ( $( $rest:tt )* ) => {{
        let mut program: aoi::AoProgram = vec![];
        asm_muncher!(program, $($rest)*);
        program
    }};
}
