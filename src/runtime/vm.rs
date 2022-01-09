use super::opcode::AoOpCode;
use super::status::AoStatus;
use super::types::AoType;

/// Aoi VM.
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
    /// Create a new AoVM.
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
        vm.gvs.resize_with(gv_count as usize, AoType::default);
        vm
    }

    /// Push a value to the data stack.
    ///
    /// # Examples
    /// ```
    /// use aoi::runtime::vm::AoVM;
    /// use aoi::runtime::types::AoType;
    ///
    /// let mut vm = AoVM::new(|_, _| None, 0);
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
    /// let mut vm = AoVM::new(|_, _| None, 0);
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
    /// let mut vm = AoVM::new(|_, _| None, 0);
    /// vm.push(AoType::AoInt(1));
    /// vm.push(AoType::AoInt(2));
    ///
    /// assert_eq!(*vm.peek().unwrap(), AoType::AoInt(2));
    /// ```
    pub fn peek(&self) -> Option<&AoType> {
        self.ds.last()
    }

    /// Use the VM to execute a program.
    pub fn run(&mut self, program: &[AoOpCode]) -> AoStatus {
        loop {
            let status = self.step(program);
            match status {
                AoStatus::Ok => (),
                _ => return status,
            }
        }
    }

    /// Go one step in the program.
    pub fn step(&mut self, program: &[AoOpCode]) -> AoStatus {
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
        self.dsb = 0;
        self.pc = 0;
        self.dp = 0;
        self.ca = AoType::default();

        self.ds.clear();
        self.gvs.clear();
        self.cs.clear();
    }
}
