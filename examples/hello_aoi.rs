use aoi::*;

fn main() {
    let mut vm = AoVM::default();
    let program = ao_program![
        push dsb
        push "Hello Aoi!"
        cnf 1
        int 1
    ];

    // run
    let status = vm.run(&program);
    if let AoStatus::Exit = status {
        println!("\nProcess finished.");
    } else {
        eprintln!("\nProcess terminated: {}.", status);
    }
    println!();
}
