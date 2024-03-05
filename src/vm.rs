use crate::*;

pub struct VM {
    memory: Vec<u8>,
    cpu: CPU,
}
impl VM {
    pub fn new(mem: Vec<u8>) -> Self {
        Self {
            memory: mem,
            cpu: CPU::new(),
        }
    }
    pub fn run(&mut self) -> ! {
        loop {
            self.cpu.fetch(&self.memory);
        }
    }
}