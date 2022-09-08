pub mod memory;

use super::AoOpcode;
use super::AoStatus;
use super::AoType;
use memory::Memory;

/// Aoi VM.
pub struct AoVM {
    pub pc: u32,
    pub dp: u32,
    pub mp: u32,
    pub cs: Vec<u32>,

    pub dsb: u32,
    pub ca: AoType,
    pub cb: AoType,

    pub ds: Vec<AoType>,
    pub mem: Memory,

    pub interrupt: fn(u8, Vec<AoType>) -> Option<AoType>,
}

impl AoVM {
    fn default_interrupt(id: u8, args: Vec<AoType>) -> Option<AoType> {
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
                    AoType::AoBool(v) => println!("{}", v),
                    AoType::AoInt(v) => println!("{}", v),
                    AoType::AoFloat(v) => println!("{}", v),
                    AoType::AoString(v) => println!("{}", v),
                    _ => (),
                }
                None
            }
            _ => None,
        }
    }

    /// Create a new AoVM.
    pub fn new(int: fn(u8, Vec<AoType>) -> Option<AoType>) -> AoVM {
        AoVM {
            pc: 0,
            dp: 0,
            mp: 0,
            cs: Vec::new(),

            dsb: 0,
            ca: AoType::default(),
            cb: AoType::default(),

            ds: Vec::new(),
            mem: Memory::new(),

            interrupt: int,
        }
    }

    /// Create a new AoVM with default interrupt.
    pub fn default() -> AoVM {
        AoVM::new(AoVM::default_interrupt)
    }

    /// Push a value to the data stack.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::vm::AoVM;
    /// use aoi::runtime::types::AoType;
    ///
    /// let mut vm = AoVM::default();
    /// vm.push(AoType::AoInt(1));
    /// assert_eq!(vm.ds.len(), 1);
    /// assert_eq!(vm.ds[0], AoType::AoInt(1));
    /// ```
    pub fn push(&mut self, value: AoType) -> bool {
        if self.ds.len() > 1000000 {
            return false;
        }

        self.ds.push(value);
        true
    }

    /// Pop a value from the data stack.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::vm::AoVM;
    /// use aoi::runtime::types::AoType;
    ///
    /// let mut vm = AoVM::default();
    /// vm.push(AoType::AoInt(1));
    /// vm.push(AoType::AoInt(2));
    ///
    /// assert_eq!(vm.pop().unwrap(), AoType::AoInt(2));
    /// assert_eq!(vm.pop().unwrap(), AoType::AoInt(1));
    /// assert_eq!(vm.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<AoType> {
        self.ds.pop()
    }

    /// Get the value at the top of the data stack.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::vm::AoVM;
    /// use aoi::runtime::types::AoType;
    ///
    /// let mut vm = AoVM::default();
    /// vm.push(AoType::AoInt(1));
    /// vm.push(AoType::AoInt(2));
    ///
    /// assert_eq!(*vm.peek().unwrap(), AoType::AoInt(2));
    /// ```
    pub fn peek(&self) -> Option<&AoType> {
        self.ds.last()
    }

    /// Use the VM to execute a program.
    pub fn run(&mut self, program: &[Box<dyn AoOpcode>]) -> AoStatus {
        loop {
            let status = self.step(program);
            match status {
                AoStatus::Ok => (),
                _ => return status,
            }
        }
    }

    /// Go one step in the program.
    pub fn step(&mut self, program: &[Box<dyn AoOpcode>]) -> AoStatus {
        if self.pc < program.len() as u32 {
            let current = self.pc as usize;
            self.pc += 1;
            program[current].execute(self)
        } else {
            AoStatus::Exit
        }
    }

    /// Reset the VM.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.dp = 0;
        self.mp = 0;
        self.cs.clear();

        self.dsb = 0;
        self.ca = AoType::default();
        self.cb = AoType::default();

        self.ds.clear();
        self.mem = Memory::new();
    }
}
