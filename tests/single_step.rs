use aoi::*;

fn interrupt(id: u8, args: Vec<AoType>) -> Option<AoType> {
    match id {
        1 => {
            match &args[0] {
                AoType::AoBool(v) => print!("{}", v),
                AoType::AoInt(v) => print!("{}", v),
                AoType::AoFloat(v) => print!("{}", v),
                AoType::AoString(v) => print!("{}", v),
                _ => (),
            }
            None
        }
        2 => {
            match &args[0] {
                AoType::AoBool(v) => print!("{}\n", v),
                AoType::AoInt(v) => print!("{}\n", v),
                AoType::AoFloat(v) => print!("{}\n", v),
                AoType::AoString(v) => print!("{}\n", v),
                _ => (),
            }
            None
        }
        _ => None,
    }
}

#[test]
fn single_step() {
    let mut vm = AoVM::new(interrupt, 0);

    // calculate 1 + 2 + ... + 10
    let program = vec![
        AoOpCode::PUSH(AoArg::from(1)),      // 0:  push 1    ; a = 1
        AoOpCode::PUSH(AoArg::from(0)),      // 1:  push 0    ; b = 0
        AoOpCode::ARG(0),                    // 2:  arg 0     ; while a <= 10 {
        AoOpCode::MOV(AoArg::CA, AoArg::DS), // 3:  mov ca,ds ;
        AoOpCode::LE(AoArg::from(10)),       // 4:  le 10     ;
        AoOpCode::JF(15),                    // 5:  jf 15     ;
        AoOpCode::MOV(AoArg::CA, AoArg::DS), // 6:  mov ca,ds ;     b = b + a
        AoOpCode::ARG(1),                    // 7:  arg 1     ;
        AoOpCode::ADD(AoArg::DS),            // 8:  add ds    ;
        AoOpCode::MOV(AoArg::DS, AoArg::CA), // 9:  mov ds,ca ;
        AoOpCode::ARG(0),                    // 10: arg 0     ;     a = a + 1
        AoOpCode::MOV(AoArg::CA, AoArg::DS), // 11: mov ca,ds ;
        AoOpCode::INC,                       // 12: inc       ;
        AoOpCode::MOV(AoArg::DS, AoArg::CA), // 13: mov ds,ca ;
        AoOpCode::JMP(2),                    // 14: jmp 2     ; }
        AoOpCode::PUSH(AoArg::DSB),          // 15: push dsb  ; println b
        AoOpCode::ARG(1),                    // 16: arg 1     ;
        AoOpCode::PUSH(AoArg::DS),           // 17: push ds   ;
        AoOpCode::CNF(1),                    // 18: cnf 1     ;
        AoOpCode::INT(2),                    // 19: int 2     ;
    ];

    // run
    let mut vm_bak = clone_vm_status(&vm);
    loop {
        if vm.pc as usize >= program.len() {
            println!("\nProcess finished.");
            break;
        }
        print!("\x1b[2J"); // clear screen
        print!("\x1b[H"); // move cursor to top-left
        println!("{}: {}", vm.pc, program[vm.pc as usize]);

        let status = vm.step(&program);
        match status {
            AoStatus::Ok => (),
            AoStatus::Exit => {
                println!("\nProcess finished.");
                break;
            }
            _ => {
                eprintln!("\nProcess terminated: {}.", status);
                break;
            }
        }
        display_vm_status(&vm, &vm_bak);
        vm_bak = clone_vm_status(&vm);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}

macro_rules! compare_disp {
    ( $name:ident, $now:ident, $bak:ident, $( $fi:tt )+ ) => {
        print!(stringify!($name));
        if $now.$($fi)+ != $bak.$($fi)+ {
            print!(": \x1b[38;5;208m{}\x1b[0m", $now.$($fi)+);
        } else {
            print!(": {}", $now.$($fi)+);
        }
    };
}

fn display_vm_status(vm: &AoVM, vm_bak: &AoVM) {
    println!("\n[VM Status]");

    compare_disp!(PC, vm, vm_bak, pc);
    print!("\n");
    compare_disp!(CA, vm, vm_bak, ca);
    print!("\n");

    compare_disp!(DP, vm, vm_bak, dp);
    print!(", ");
    compare_disp!(DSB, vm, vm_bak, dsb);
    print!(", ");
    compare_disp!(DST, vm, vm_bak, ds.len());
    print!("\n");

    print!("DS: ");
    for i in 0..vm.ds.len() {
        if i == vm.dsb as usize {
            if vm.dsb != vm_bak.dsb {
                print!("\x1b[38;5;208m{{\x1b[0m");
            } else {
                print!("{{");
            }
        }
        if i >= vm_bak.ds.len() || vm.ds[i] != vm_bak.ds[i] {
            print!("\x1b[38;5;208m[{}]\x1b[0m", vm.ds[i]);
        } else {
            print!("[{}]", vm.ds[i]);
        }
    }
    println!("}}");
}

fn clone_vm_status(vm: &AoVM) -> AoVM {
    let mut new_vm = AoVM::new(|_, _| None, 0);
    new_vm.pc = vm.pc;
    new_vm.ca = vm.ca.clone();
    new_vm.dp = vm.dp;
    new_vm.dsb = vm.dsb;
    new_vm.ds = vm.ds.clone();
    new_vm
}
