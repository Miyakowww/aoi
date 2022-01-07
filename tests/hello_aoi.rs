use aoi::runtime::opcode::{AoArg, AoOpCode};
use aoi::runtime::status::AoStatus;
use aoi::runtime::types::AoType;
use aoi::runtime::vm::AoVM;

#[test]
fn hello_aoi() {
    let mut vm = AoVM::new(
        |id, para| match id {
            1 => {
                match &para[0] {
                    AoType::AoBool(v) => print!("{}", v),
                    AoType::AoInt(v) => print!("{}", v),
                    AoType::AoFloat(v) => print!("{}", v),
                    AoType::AoString(v) => print!("{}", v),
                    _ => (),
                }
                None
            }
            2 => {
                match &para[0] {
                    AoType::AoBool(v) => print!("{}\n", v),
                    AoType::AoInt(v) => print!("{}\n", v),
                    AoType::AoFloat(v) => print!("{}\n", v),
                    AoType::AoString(v) => print!("{}\n", v),
                    _ => (),
                }
                None
            }
            _ => None,
        },
        0,
    );
    let program = vec![
        AoOpCode::PUSH(AoArg::DSB),                // push dsb
        AoOpCode::PUSH(AoArg::from("Hello Aoi!")), // push "Hello Aoi!"
        AoOpCode::CNF(1),                          // cnf 1
        AoOpCode::INT(1),                          // int 1
    ];

    // print Aoi assembly
    for op in &program {
        println!("{}", op);
    }
    println!("");

    // run
    let status = vm.run(&program);
    if let AoStatus::Exit = status {
        println!("\nProcess finished.");
    } else {
        eprintln!("\nProcess terminated: {}.", status);
    }
    println!("");
}
