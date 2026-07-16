#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Push(u64),
    Pop,
    Load([u8; 20]),
    Store([u8; 20]),
    Transfer([u8; 20], u64),
    Jump(usize),
    JumpIfZero(usize),
    Call(usize),
    Return,
    Halt,
}

pub struct VM {
    pub pc: usize,
    pub memory: HashMap<u64, u64>,
    pub stack: Vec<u64>,
    pub state: State,
    pub code: Vec<Opcode>,
    pub gas_limit: u64,
    pub gas_used: u64,
}

impl VM {
    pub fn new(code: Vec<Opcode>, state: State, gas_limit: u64) -> Self {
        Self {
            pc: 0,
            memory: HashMap::new(),
            stack: Vec::new(),
            state,
            code,
            gas_limit,
            gas_used: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.code.len() {
            self.gas_used += 1;
            if self.gas_used > self.gas_limit {
                return Err("Out of gas".to_string());
            }

            match &self.code[self.pc] {
                Opcode::Push(value) => {
                    self.stack.push(*value);
                }
                Opcode::Pop => {
                    self.stack.pop().ok_or("Stack underflow")?;
                }
                Opcode::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a.wrapping_add(b));
                }
                Opcode::Sub => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a.wrapping_sub(b));
                }
                
            let opcode = &self.code[self.pc];
            self.execute(opcode)?;
            self.pc += 1;
        }
        Ok(())
    }
}