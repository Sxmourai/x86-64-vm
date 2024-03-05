#![cfg_attr(debug_assertions, allow(dead_code, unused))]
#![feature(variant_count)]

pub use color_eyre::Result;
use hex_slice::AsHex;

pub mod cpu;
pub use cpu::CPU;
pub mod vm;

pub const MEM_SIZE: usize = 512 * 1024;

fn main() {
    // let memory = [0; MEM_SIZE];
    let memory = std::fs::read("user.o").expect("Failed opening user program !");
    println!("{:x}", memory.as_hex());
    tracing::warn!("Assuming 64bit mode for now, because 16bit real mode isn't supported !");
    vm::VM::new(memory).run()
}
