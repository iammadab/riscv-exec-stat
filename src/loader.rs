use std::fs;
use std::path::Path;

use crate::VM;
use crate::elf::decode_elf;
use crate::trace::Tracer;

const DEFAULT_STACK_POINTER: u64 = 0x0800_0000;

pub fn init_from_elf<T: Tracer + Default>(path: impl AsRef<Path>) -> VM<T> {
    init_from_elf_with_tracer(path, T::default())
}

pub fn init_from_elf_with_tracer<T: Tracer>(path: impl AsRef<Path>, tracer: T) -> VM<T> {
    let elf_bytes = fs::read(path).unwrap();
    let (memory, pc) = decode_elf(&elf_bytes);
    // Initialize stack pointer (x2/sp) to a valid memory address
    let mut registers = [0u64; 32];
    registers[2] = DEFAULT_STACK_POINTER;
    VM::from_parts(registers, memory, pc, tracer)
}
