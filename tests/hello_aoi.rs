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
fn hello_aoi() {
    let mut vm = AoVM::new(interrupt, 0);
    let program = vec![
        aoasm!(push dsb),
        aoasm!(push "Hello Aoi!"),
        aoasm!(cnf 1),
        aoasm!(int 1),
    ];

    // run
    let status = vm.run(&program);
    if let AoStatus::Exit = status {
        println!("\nProcess finished.");
    } else {
        eprintln!("\nProcess terminated: {}.", status);
    }
    println!("");
}
