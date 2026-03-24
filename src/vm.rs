use crate::memory::MemoryDefault;
use crate::util::{is_snan_f32, is_snan_f64, is_subnormal_f32, is_subnormal_f64};
use std::mem::offset_of;

/// RISC-V Virtual Machine with configurable tracing.
///
/// The VM is generic over a `Tracer` type, enabling zero-cost abstraction:
/// - `NoopTracer`: All tracing calls are optimized away (zero overhead)
/// - `FullTracer`: Complete execution trace is captured
#[repr(C)]
pub struct VM {
    pub(crate) registers: [u64; 32],
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

    pub(crate) fn from_parts(
        registers: [u64; 32],
        memory: MemoryDefault,
        pc: u64,
    ) -> Self {
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
    pub(crate) fn reg(&self, idx: u8) -> u64 {
        if idx == 0 {
            0
        } else {
            self.registers[idx as usize]
        }
    }

    /// Returns a mutable reference to the idx register
    pub(crate) fn reg_mut(&mut self, idx: u8, value: u64) {
        if idx == 0 {
            self.registers[idx as usize] = 0;
        } else {
            self.registers[idx as usize] = value;
        }
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

    pub(crate) fn raise_fflags_f32(&mut self, a: f32, b: f32, res: f32, op: char) {
        let mut flags = 0u32;

        // NV: Invalid operation
        // 1. Result is NaN but neither input was NaN
        if res.is_nan() && !a.is_nan() && !b.is_nan() {
            flags |= 0b10000;
        }

        // 2. Check for signaling NaN inputs (always invalid)
        if is_snan_f32(a) || is_snan_f32(b) {
            flags |= 0b10000;
        }

        // 3. Invalid subtraction: inf - inf or (-inf) - (-inf)
        if op == '-' && a.is_infinite() && b.is_infinite() && a.signum() == b.signum() {
            flags |= 0b10000;
        }

        // 4. Invalid addition: inf + (-inf) or (-inf) + inf
        if op == '+' && a.is_infinite() && b.is_infinite() && a.signum() != b.signum() {
            flags |= 0b10000;
        }

        // 5. Invalid multiplication: 0 * inf or inf * 0
        if op == '*' && ((a == 0.0 && b.is_infinite()) || (a.is_infinite() && b == 0.0)) {
            flags |= 0b10000;
        }

        // 6. Invalid division: 0/0 or inf/inf
        if op == '/' && ((a == 0.0 && b == 0.0) || (a.is_infinite() && b.is_infinite())) {
            flags |= 0b10000;
        }

        // DZ: Divide by zero (finite / 0)
        if op == '/' && b == 0.0 && !a.is_nan() && !a.is_infinite() && a != 0.0 {
            flags |= 0b01000;
        }

        // OF: Overflow (result is infinite but inputs weren't)
        if res.is_infinite() && !a.is_infinite() && !b.is_infinite() {
            flags |= 0b00100;
            flags |= 0b00001; // Overflow also sets inexact
        }

        // UF: Underflow (result is subnormal)
        if is_subnormal_f32(res) {
            flags |= 0b00010;
            flags |= 0b00001; // Underflow also sets inexact (usually)
        }

        // NX: Inexact
        if !res.is_nan() && !res.is_infinite() && !a.is_nan() && !b.is_nan() {
            let a64 = a as f64;
            let b64 = b as f64;
            let exact = match op {
                '+' => a64 + b64,
                '-' => a64 - b64,
                '*' => a64 * b64,
                '/' => {
                    if b64 != 0.0 {
                        a64 / b64
                    } else {
                        res as f64
                    }
                }
                _ => res as f64,
            };
            if exact != (res as f64) {
                flags |= 0b00001;
            }
        }

        self.fcsr_reg |= flags;
    }

    pub(crate) fn raise_fflags_f64(&mut self, a: f64, b: f64, res: f64, op: char) {
        let mut flags = 0u32;

        // NV: Invalid operation - result is NaN but neither input was NaN
        if res.is_nan() && !a.is_nan() && !b.is_nan() {
            flags |= 0b10000;
        }

        // Check for signaling NaN inputs
        if is_snan_f64(a) || is_snan_f64(b) {
            flags |= 0b10000;
        }

        // DZ: Divide by zero
        if op == '/' && b == 0.0 && !a.is_nan() && !b.is_nan() && !a.is_infinite() {
            flags |= 0b01000;
        }

        // OF: Overflow (result is infinite but inputs weren't)
        if res.is_infinite() && !a.is_infinite() && !b.is_infinite() {
            flags |= 0b00100;
            flags |= 0b00001; // Overflow also sets inexact
        }

        // UF: Underflow (result is subnormal)
        if is_subnormal_f64(res) {
            flags |= 0b00010;
        }

        // NX: Inexact - for f64, we can't easily use higher precision
        // Use a heuristic: check if result has full precision bits used
        // This is imperfect but catches many cases
        if !res.is_nan() && !res.is_infinite() {
            // For operations that are commonly inexact
            if op == '/' {
                // Division is often inexact unless result is exact
                let check = res * b;
                if check != a {
                    flags |= 0b00001;
                }
            }
            // For sqrt, add, sub, mul - harder to detect without f128
        }

        self.fcsr_reg |= flags;
    }

    pub(crate) fn raise_fflags_fma_f32(&mut self, a: f32, b: f32, c: f32, res: f32) {
        let mut flags = 0u32;

        // NV: Invalid operation
        // sNaN inputs
        if is_snan_f32(a) || is_snan_f32(b) || is_snan_f32(c) {
            flags |= 0b10000;
        }

        // 0 * inf or inf * 0
        if (a == 0.0 && b.is_infinite()) || (a.is_infinite() && b == 0.0) {
            flags |= 0b10000;
        }

        // inf + (-inf) in the addition part
        let mul_res = a * b;
        if mul_res.is_infinite() && c.is_infinite() && mul_res.signum() != c.signum() {
            flags |= 0b10000;
        }

        // Result is NaN but no input was NaN
        if res.is_nan() && !a.is_nan() && !b.is_nan() && !c.is_nan() {
            flags |= 0b10000;
        }

        // OF: Overflow
        if res.is_infinite() && !a.is_infinite() && !b.is_infinite() && !c.is_infinite() {
            flags |= 0b00100;
            flags |= 0b00001;
        }

        // UF: Underflow
        if is_subnormal_f32(res) {
            flags |= 0b00010;
            flags |= 0b00001;
        }

        // NX: Inexact - use f64 to check
        if !res.is_nan() && !res.is_infinite() && !a.is_nan() && !b.is_nan() && !c.is_nan() {
            let a64 = a as f64;
            let b64 = b as f64;
            let c64 = c as f64;
            let exact = a64.mul_add(b64, c64);
            if exact != (res as f64) {
                flags |= 0b00001;
            }
        }

        self.fcsr_reg |= flags;
    }

    pub(crate) fn raise_fflags_fma_f64(&mut self, a: f64, b: f64, c: f64, res: f64) {
        let mut flags = 0u32;

        // NV: Invalid operation
        if is_snan_f64(a) || is_snan_f64(b) || is_snan_f64(c) {
            flags |= 0b10000;
        }

        if (a == 0.0 && b.is_infinite()) || (a.is_infinite() && b == 0.0) {
            flags |= 0b10000;
        }

        let mul_res = a * b;
        if mul_res.is_infinite() && c.is_infinite() && mul_res.signum() != c.signum() {
            flags |= 0b10000;
        }

        if res.is_nan() && !a.is_nan() && !b.is_nan() && !c.is_nan() {
            flags |= 0b10000;
        }

        // OF: Overflow
        if res.is_infinite() && !a.is_infinite() && !b.is_infinite() && !c.is_infinite() {
            flags |= 0b00100;
            flags |= 0b00001;
        }

        // UF: Underflow
        if is_subnormal_f64(res) {
            flags |= 0b00010;
        }

        self.fcsr_reg |= flags;
    }
}




