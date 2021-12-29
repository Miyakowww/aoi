use super::opcode::AoOpCode;
use super::status::AoStatus;
use super::types::AoType;

pub struct AoVM {
    pub dsb: u32,
    pub pc: u32,
    pub dp: u32,
    pub ca: AoType,

    pub ds: Vec<AoType>,
    pub gvs: Vec<AoType>,
    pub cs: Vec<u32>,

    pub interrupt: fn(u8, Vec<AoType>) -> Option<AoType>,
}

impl AoVM {
    pub fn new(int: fn(u8, Vec<AoType>) -> Option<AoType>, gv_count: i32) -> AoVM {
        let mut vm = AoVM {
            dsb: 0,
            pc: 0,
            dp: 0,
            ca: AoType::default(),

            ds: Vec::new(),
            gvs: Vec::new(),
            cs: Vec::new(),

            interrupt: int,
        };
        vm.gvs.resize_with(gv_count as usize, || AoType::default());
        vm
    }

    pub fn push(&mut self, value: AoType) -> bool {
        if self.ds.len() > 1000000 {
            return false;
        }

        self.ds.push(value);
        true
    }

    pub fn pop(&mut self) -> Option<AoType> {
        self.ds.pop()
    }

    pub fn peek(&self) -> Option<&AoType> {
        self.ds.last()
    }

    pub fn run(&mut self, program: &Vec<AoOpCode>) -> AoStatus {
        while self.pc < program.len() as u32 {
            let current = self.pc as usize;
            self.pc += 1;
            let status = program[current].execute(self);
            match status {
                AoStatus::Ok => (),
                _ => return status,
            }
        }

        AoStatus::Exit
    }

    pub fn reset(&mut self) {
        self.dsb = 0;
        self.pc = 0;
        self.dp = 0;
        self.ca = AoType::default();

        self.ds.clear();
        self.gvs.clear();
        self.cs.clear();
    }
}
