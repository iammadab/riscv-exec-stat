use crate::memory::MemoryDefault;
use std::mem::offset_of;

/// RISC-V Virtual Machine with configurable tracing.
///
/// The VM is generic over a `Tracer` type, enabling zero-cost abstraction:
/// - `NoopTracer`: All tracing calls are optimized away (zero overhead)
/// - `FullTracer`: Complete execution trace is captured
#[repr(C)]
pub struct VM {
    pub(crate) registers: [u64; 32],
    reg_reads: [u64; 32],
    reg_writes: [u64; 32],
    pc: u64,
    pub(crate) f_reg: [u64; 32],
    pub(crate) fcsr_reg: u32,
    pub(crate) reservation_set: u64,
    pub halted: bool,
    pub exit_code: u64,
    memory: MemoryDefault,
}

pub(crate) const VM_REGS_OFFSET: usize = offset_of!(VM, registers);
pub(crate) const VM_PC_OFFSET: usize = offset_of!(VM, pc);
pub(crate) const VM_FREGS_OFFSET: usize = offset_of!(VM, f_reg);
pub(crate) const VM_FCSR_OFFSET: usize = offset_of!(VM, fcsr_reg);
pub(crate) const VM_RESERVATION_OFFSET: usize = offset_of!(VM, reservation_set);
pub(crate) const VM_HALTED_OFFSET: usize = offset_of!(VM, halted);
pub(crate) const VM_EXIT_CODE_OFFSET: usize = offset_of!(VM, exit_code);

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0u64; 32],
            reg_reads: [0u64; 32],
            reg_writes: [0u64; 32],
            memory: MemoryDefault::default(),
            reservation_set: 0,
            pc: 0,
            halted: false,
            exit_code: 0,
            f_reg: [0u64; 32],
            fcsr_reg: 0,
        }
    }
}

impl VM {
    /// Returns a VM with empty state
    pub fn init() -> Self {
        Self::default()
    }

    pub(crate) fn from_parts(registers: [u64; 32], memory: MemoryDefault, pc: u64) -> Self {
        Self {
            registers,
            memory,
            pc,
            ..Default::default()
        }
    }

    /// Get the current PC
    pub fn pc(&self) -> u64 {
        self.pc
    }

    pub(crate) fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    /// Check if the VM has halted
    pub fn halted(&self) -> bool {
        self.halted
    }

    /// Get the exit code
    pub fn exit_code(&self) -> u64 {
        self.exit_code
    }

    /// Returns the current value at the idx register
    pub(crate) fn reg(&mut self, idx: u8) -> u64 {
        self.reg_reads[idx as usize] = self.reg_reads[idx as usize].wrapping_add(1);
        if idx == 0 {
            0
        } else {
            self.registers[idx as usize]
        }
    }

    /// Returns a mutable reference to the idx register
    pub(crate) fn reg_mut(&mut self, idx: u8, value: u64) {
        self.reg_writes[idx as usize] = self.reg_writes[idx as usize].wrapping_add(1);
        if idx == 0 {
            self.registers[idx as usize] = 0;
        } else {
            self.registers[idx as usize] = value;
        }
    }

    pub fn reg_reads(&self) -> &[u64; 32] {
        &self.reg_reads
    }

    pub fn reg_writes(&self) -> &[u64; 32] {
        &self.reg_writes
    }

    pub fn reg_accesses(&self) -> [u64; 32] {
        let mut totals = [0u64; 32];
        for (i, item) in totals.iter_mut().enumerate() {
            *item = self.reg_reads[i].wrapping_add(self.reg_writes[i]);
        }
        totals
    }

    pub fn reg_read_probabilities(&self) -> [f64; 32] {
        Self::normalize_counts(&self.reg_reads)
    }

    pub fn reg_write_probabilities(&self) -> [f64; 32] {
        Self::normalize_counts(&self.reg_writes)
    }

    pub fn reg_access_probabilities(&self) -> [f64; 32] {
        let counts = self.reg_accesses();
        Self::normalize_counts(&counts)
    }

    fn normalize_counts(counts: &[u64; 32]) -> [f64; 32] {
        let total: u64 = counts.iter().sum();
        if total == 0 {
            return [0.0; 32];
        }

        let denom = total as f64;
        let mut probs = [0.0; 32];
        for (i, item) in probs.iter_mut().enumerate() {
            *item = counts[i] as f64 / denom;
        }
        probs
    }

    /// Returns the current value at the idx floating point register
    pub(crate) fn read_f64(&self, idx: u8) -> f64 {
        f64::from_bits(self.f_reg[idx as usize])
    }

    /// Updates idx floating point register to value
    pub(crate) fn write_f64(&mut self, idx: u8, value: f64) {
        let res = value.to_bits();
        self.f_reg[idx as usize] = res;
    }

    // Read f32
    pub(crate) fn read_f32(&self, idx: u8) -> f32 {
        let val = self.f_reg[idx as usize];
        if val >> 32 != 0xffff_ffff {
            // signal quiet
            return f32::from_bits(0x7FC0_0000);
        }
        f32::from_bits(val as u32)
    }

    // Write f32
    pub(crate) fn write_f32(&mut self, idx: u8, val: f32) {
        let res = 0xffff_ffff_0000_0000 | (val.to_bits() as u64);
        self.f_reg[idx as usize] = res;
    }

    /// Load 8 bytes from memory at the given addr
    /// assumes value at memory address is the LSB
    pub(crate) fn load_u64(&mut self, addr: usize) -> u64 {
        self.memory.read_u64(addr as u64)
    }

    /// Load 4 bytes from memory at the given addr
    /// assumes value at memory address is the LSB
    pub(crate) fn load_u32(&mut self, addr: usize) -> u32 {
        self.memory.read_u32(addr as u64)
    }

    /// Load 2 bytes from memory at the given addr
    /// assumes value at memory address is the LSB
    pub(crate) fn load_u16(&mut self, addr: usize) -> u16 {
        self.memory.read_u16(addr as u64)
    }

    /// Load 1 byte from memory at the given addr
    pub(crate) fn load_u8(&mut self, addr: usize) -> u8 {
        self.memory.read_u8(addr as u64)
    }

    /// Write 8 butes to memory at the given addr
    pub(crate) fn store_u64(&mut self, addr: usize, value: u64) {
        self.memory.write_u64(addr as u64, value);
    }

    /// Write 4 bytes to memory at the given addr
    pub(crate) fn store_u32(&mut self, addr: usize, value: u32) {
        self.memory.write_u32(addr as u64, value);
    }

    /// Write 2 bytes to memory at the given addr
    pub(crate) fn store_u16(&mut self, addr: usize, value: u16) {
        self.memory.write_u16(addr as u64, value);
    }

    /// Write 1 byte to memory at the given addr
    pub(crate) fn store_u8(&mut self, addr: usize, value: u8) {
        self.memory.write_u8(addr as u64, value);
    }

    /// Write multiple bytes from a given address
    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) {
        self.memory.write_n_bytes(addr as u64, data);
    }

    /// Read multiple bytes from a given address
    pub(crate) fn read_bytes(&mut self, addr: usize, len: usize) -> Vec<u8> {
        self.memory.read_n_bytes(addr as u64, len)
    }

    pub(crate) fn read_csr(&self, csr: u32) -> u32 {
        match csr {
            // Read fflags
            0x1 => self.fcsr_reg & 0x1f,
            // Read frm
            0x2 => (self.fcsr_reg >> 5) & 0x7,
            // Read csr
            0x3 => self.fcsr_reg & 0xff,
            _ => 0,
        }
    }

    pub(crate) fn set_csr(&mut self, csr: u32, val: u32) {
        match csr {
            // Set fflags
            0x1 => {
                self.fcsr_reg &= !0x1f;
                self.fcsr_reg |= val & 0x1f;
            }
            // Set Frm
            0x2 => {
                self.fcsr_reg &= !(0x7 << 5);
                self.fcsr_reg |= (val & 0x7) << 5;
            }
            // Set Csr
            0x3 => {
                self.fcsr_reg &= !0xff;
                self.fcsr_reg |= val & 0xff;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn tracks_register_reads_writes_including_x0() {
        let mut vm = VM::init();

        vm.reg_mut(0, 123);
        vm.reg_mut(5, 42);
        let _ = vm.reg(0);
        let _ = vm.reg(5);

        assert_eq!(vm.reg(0), 0);
        assert_eq!(vm.reg(5), 42);

        assert_eq!(vm.reg_writes()[0], 1);
        assert_eq!(vm.reg_writes()[5], 1);
        assert_eq!(vm.reg_reads()[0], 2);
        assert_eq!(vm.reg_reads()[5], 2);
    }

    #[test]
    fn computes_access_and_probability_vectors() {
        let mut vm = VM::init();

        let _ = vm.reg(1);
        let _ = vm.reg(1);
        vm.reg_mut(1, 7);
        let _ = vm.reg(2);
        vm.reg_mut(2, 9);

        let accesses = vm.reg_accesses();
        assert_eq!(accesses[1], 3);
        assert_eq!(accesses[2], 2);

        let read_probs = vm.reg_read_probabilities();
        assert!((read_probs[1] - (2.0 / 3.0)).abs() < 1e-12);
        assert!((read_probs[2] - (1.0 / 3.0)).abs() < 1e-12);

        let write_probs = vm.reg_write_probabilities();
        assert!((write_probs[1] - 0.5).abs() < 1e-12);
        assert!((write_probs[2] - 0.5).abs() < 1e-12);

        let access_probs = vm.reg_access_probabilities();
        let sum: f64 = access_probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-12);
        assert!((access_probs[1] - 0.6).abs() < 1e-12);
        assert!((access_probs[2] - 0.4).abs() < 1e-12);
    }

    #[test]
    fn probability_vectors_are_zero_when_no_accesses() {
        let vm = VM::init();
        assert!(vm.reg_read_probabilities().iter().all(|v| *v == 0.0));
        assert!(vm.reg_write_probabilities().iter().all(|v| *v == 0.0));
        assert!(vm.reg_access_probabilities().iter().all(|v| *v == 0.0));
    }
}
