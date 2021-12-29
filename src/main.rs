use aoi::runtime::opcode::*;
use aoi::runtime::status::AoStatus;
use aoi::runtime::types::AoType;
use aoi::runtime::vm::AoVM;
use aoi::serialization::AoAsmSerializer;

fn main() {
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
        AoOpCode::PUSH(AoArg::DSB),                // push dsb          <- push DSB
        AoOpCode::PUSH(AoArg::from("Hello Aoi!")), // push "Hello Aoi!" <- push args
        AoOpCode::MOV(AoArg::CA, AoArg::DST),      // mov ca,dst        <- set DSB to (DST - argc)
        AoOpCode::SUB(AoArg::from(1u32)),          // sub 1p             *
        AoOpCode::MOV(AoArg::DSB, AoArg::CA),      // mov dsb,ca         *
        AoOpCode::INT(1),                          // int 1             <- interrupt 1
    ];

    // run
    {
        let status = vm.run(&program);
        if let AoStatus::Exit = status {
            println!("\nProcess finished.");
        } else {
            eprintln!("\nProcess terminated: {}.", status);
        }
        println!("");
    }

    // test serialization and deserialization
    {
        let bin = AoAsmSerializer::serialize(&program);
        let des_prog = AoAsmSerializer::deserialize(&bin).unwrap();

        // print asm
        for op in &des_prog {
            println!("{}", op);
        }
        println!("");

        // re-run
        vm.reset();
        let status = vm.run(&des_prog);
        if let AoStatus::Exit = status {
            println!("\nProcess finished.");
        } else {
            eprintln!("\nProcess terminated: {}.", status);
        }
    }
}
