/// CSR instructions
use crate::VM;
use crate::decode::I;
use crate::trace::Tracer;

#[inline(always)]
pub(crate) fn execute_csrrw<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF; // Mask to 12 bits
    let old = vm.read_csr(csr_addr) as u64;
    let val = vm.reg(insn.rs1) as u32;

    vm.set_csr(csr_addr, val);
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}

#[inline(always)]
pub(crate) fn execute_csrrs<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF;
    let old = vm.read_csr(csr_addr) as u64;
    if insn.rs1 != 0 {
        let val = vm.reg(insn.rs1) as u32;
        let new_val = old as u32 | val;
        vm.set_csr(csr_addr, new_val);
    }
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}

#[inline(always)]
pub(crate) fn execute_csrrc<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF;
    let old = vm.read_csr(csr_addr) as u64;
    if insn.rs1 != 0 {
        let val = vm.reg(insn.rs1) as u32;
        let new_val = old as u32 & !val;
        vm.set_csr(csr_addr, new_val);
    }
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}

#[inline(always)]
pub(crate) fn execute_csrrwi<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF;
    let old = vm.read_csr(csr_addr) as u64;
    let val = (insn.rs1 as u32) & 0x1F;
    vm.set_csr(csr_addr, val);
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}

#[inline(always)]
pub(crate) fn execute_csrrsi<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF;
    let old = vm.read_csr(csr_addr) as u64;
    let val = (insn.rs1 as u32) & 0x1F;
    if val != 0 {
        let new_val = old as u32 | val;
        vm.set_csr(csr_addr, new_val);
    }
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}

#[inline(always)]
pub(crate) fn execute_csrrci<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let csr_addr = (insn.imm as u32) & 0xFFF;
    let old = vm.read_csr(csr_addr) as u64;
    let val = (insn.rs1 as u32) & 0x1F;
    if val != 0 {
        let new_val = old as u32 & !val;
        vm.set_csr(csr_addr, new_val);
    }
    if insn.rd != 0 {
        vm.reg_mut(insn.rd, old);
    }
}
