use bit_field::BitField;
use color_eyre::Result;
use std::collections::HashMap;

use self::regs::RegisterId;

pub mod regs;

/// Fasterthanlime's reader in https://fasterthanli.me/series/reading-files-the-hard-way/part-3#reading-an-inode-in-ext4
pub struct CPU {
    regs: Vec<u64>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: vec![0; std::mem::variant_count::<RegisterId>()],
        }
    }
    pub fn get_reg(&self, reg_offset: RegisterId) -> u64 {
        self.regs[reg_offset as usize]
    }
    pub fn get_regl(&self, reg_offset: RegisterId) -> u8 {self.get_reg(reg_offset).get_bits(0..7) as _}
    pub fn get_regh(&self, reg_offset: RegisterId) -> u8 {self.get_reg(reg_offset).get_bits(8..15) as _}
    pub fn get_regx(&self, reg_offset: RegisterId) -> u32 {self.get_reg(reg_offset).get_bits(0..31) as _}
    
    pub fn set_regl(&mut self, reg_offset: RegisterId, val: u8) {self.regs[reg_offset as usize].set_bits(0..7, val.into());}
    pub fn set_regh(&mut self, reg_offset: RegisterId, val: u8) {self.regs[reg_offset as usize].set_bits(8..15, val.into());}
    pub fn set_regx(&mut self, reg_offset: RegisterId, val: u32) {self.regs[reg_offset as usize].set_bits(0..31, val.into());}
    pub fn fetch(&mut self, memory: &[u8]) -> CpuInstruction {
        if self.get_reg(RegisterId::EIP) as usize >= memory.len() {
            tracing::info!("Done executing program !");
            tracing::debug!("Debugging registers:");
            for (i, &reg) in self.regs.iter().enumerate() {
                let reg_name = RegisterId::try_from(i as u64).unwrap();
                println!("- {:?}: {}", reg_name, reg);
            }
            panic!();
        }
        let raw_instruction = memory[self.get_reg(RegisterId::EIP) as usize];
        let instruction = CpuInstruction::Mov(RegisterId::RAX, RegisterId::RBX);
        self.regs[RegisterId::EIP as usize] += 8;
        instruction
    }
    pub fn execute(&mut self, instruction: &CpuInstruction) -> Result<()> {
        match instruction {
            CpuInstruction::Mov(reg_out, reg_in) => {}
        }
        Ok(())
    }
}
pub enum CpuInstruction {
    Mov(RegisterId, RegisterId), //TODO Use a movable trait
}
