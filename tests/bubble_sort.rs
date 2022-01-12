#![recursion_limit = "107"]
use aoi::*;

#[test]
fn bubble_sort() {
    let mut vm = AoVM::default();
    let program = ao_program![
        //        arr = [ 3, 19, 5, 15, 1, 4, 16, 8 ]
        /*   0 */ mov mp,0
        /*   1 */ mov mem,3
        /*   2 */ mov mp,1
        /*   3 */ mov mem,19
        /*   4 */ mov mp,2
        /*   5 */ mov mem,5
        /*   6 */ mov mp,3
        /*   7 */ mov mem,15
        /*   8 */ mov mp,4
        /*   9 */ mov mem,1
        /*  10 */ mov mp,5
        /*  11 */ mov mem,4
        /*  12 */ mov mp,6
        /*  13 */ mov mem,16
        /*  14 */ mov mp,7
        /*  15 */ mov mem,8
        //        len = 8
        /*  16 */ push 8
        //        for (i = 0; i + 1 < len; i++) {
        /*  17 */ push 0
        /*  18 */ arg 1
        /*  19 */ mov ca,ds
        /*  20 */ add 1
        /*  21 */ arg 0
        /*  22 */ lt ds
        /*  23 */ jf 80
        //            for (j = 0; j + 1 + i < len; j++) {
        /*  24 */ push 0
        /*  25 */ arg 2
        /*  26 */ mov ca,ds
        /*  27 */ add 1
        /*  28 */ arg 1
        /*  29 */ add ds
        /*  30 */ arg 0
        /*  31 */ lt ds
        /*  32 */ jf 74
        //                if (arr[j + 1] < arr[j]) {
        /*  33 */ arg 2
        /*  34 */ mov ca,ds
        /*  35 */ csp
        /*  36 */ mov mp,ca
        /*  37 */ push mem
        /*  38 */ add 1
        /*  39 */ mov mp,ca
        /*  40 */ push mem
        /*  41 */ mov ca,dst
        /*  42 */ sub 2
        /*  43 */ mov dp,ca
        /*  44 */ pop ca
        /*  45 */ lt ds
        /*  46 */ pop
        /*  47 */ jf 69
        //                    tmp = arr[j + 1];
        /*  48 */ arg 2
        /*  49 */ mov ca,ds
        /*  50 */ add 1
        /*  51 */ csp
        /*  52 */ mov mp,ca
        /*  53 */ push mem
        //                    arr[j + 1] = arr[j];
        /*  54 */ arg 2
        /*  55 */ mov ca,ds
        /*  56 */ csp
        /*  57 */ mov mp,ca
        /*  58 */ push mem
        /*  59 */ add 1
        /*  60 */ mov mp,ca
        /*  61 */ pop ca
        /*  62 */ mov mem,ca
        //                    arr[j] = tmp;
        /*  63 */ arg 2
        /*  64 */ mov ca,ds
        /*  65 */ csp
        /*  66 */ mov mp,ca
        /*  67 */ pop ca
        /*  68 */ mov mem,ca
        //                }
        //            }
        /*  69 */ arg 2
        /*  70 */ mov ca,ds
        /*  71 */ add 1
        /*  72 */ mov ds,ca
        /*  73 */ jmp 25
        /*  74 */ pop
        //        }
        /*  75 */ arg 1
        /*  76 */ mov ca,ds
        /*  77 */ add 1
        /*  78 */ mov ds,ca
        /*  79 */ jmp 18
        /*  80 */ pop
        //        for (i = 0; i < len; i++) {
        /*  81 */ push 0
        /*  82 */ arg 1
        /*  83 */ mov ca,ds
        /*  84 */ arg 0
        /*  85 */ lt ds
        /*  86 */ jf 103
        //            print(arr[i] + ", ");
        /*  87 */ push dsb
        /*  88 */ arg 1
        /*  89 */ mov ca,ds
        /*  90 */ csp
        /*  91 */ mov mp,ca
        /*  92 */ mov ca,mem
        /*  93 */ css
        /*  94 */ add ", "
        /*  95 */ push ca
        /*  96 */ cnf 1
        /*  97 */ int 1
        //        }
        /*  98 */ arg 1
        /*  99 */ mov ca,ds
        /* 100 */ add 1
        /* 101 */ mov ds,ca
        /* 102 */ jmp 82
        /* 103 */ pop
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
