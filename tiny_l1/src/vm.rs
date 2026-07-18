use crate::prelude::*;
use crate::state::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Push(u64),
    Pop,
    Dup(usize),
    Load(u64),
    Store(u64),
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
            eprintln!(
                "Executing opcode: {:?}, PC: {}, Stack: {:?}, Memory: {:?}",
                self.code[self.pc], self.pc, self.stack, self.memory
            );
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
                Opcode::Mul => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a.wrapping_mul(b));
                }
                Opcode::Store(key) => {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    self.memory.insert(*key, value);
                }
                Opcode::Load(key) => {
                    let value = self.memory.get(&*key).copied().ok_or("Memory read error")?;
                    self.stack.push(value);
                }
                Opcode::Jump(target) => {
                    self.pc = *target as usize;
                    continue;
                }
                Opcode::JumpIfZero(target) => {
                    let condition = self.stack.pop().ok_or("Stack underflow")?;
                    if condition == 0 {
                        self.pc = *target as usize;
                        continue;
                    }
                }
                Opcode::Dup(offset) => {
                    let idx = self.stack.len().checked_sub(*offset + 1).ok_or("Duplication index out of bounds")?;
                    let value = self.stack[idx];
                    self.stack.push(value);
                }
                Opcode::Div => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a.wrapping_div(b));
                }
                Opcode::Call(target) => {
                    self.stack.push(self.pc as u64);
                    self.pc = *target as usize;
                    continue;
                }
                Opcode::Return => {
                    self.pc = self.stack.pop().ok_or("Stack underflow")? as usize;
                    continue;
                }
                Opcode::Halt => break,

            }
            self.pc += 1;
        }
        Ok(())
    }
}