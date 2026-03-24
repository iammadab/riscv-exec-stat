// Register Opcodes

use crate::decode::{Sh, B, I, J, R, S, U};
use crate::util::{mask, sext};
use crate::VM;

#[inline(always)]
pub(crate) fn execute_add(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1).wrapping_add(vm.reg(insn.rs2));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sub(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1).wrapping_sub(vm.reg(insn.rs2));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_xor(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1) ^ vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_or(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1) | vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_and(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1) & vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sll(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1) << (vm.reg(insn.rs2) & mask(6));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srl(vm: &mut VM, insn: &R) {
    let result = vm.reg(insn.rs1) >> (vm.reg(insn.rs2) & mask(6));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sra(vm: &mut VM, insn: &R) {
    let val = vm.reg(insn.rs1) as i64;
    let result = (val >> (vm.reg(insn.rs2) & mask(6))) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slt(vm: &mut VM, insn: &R) {
    let result = if (vm.reg(insn.rs1) as i64) < (vm.reg(insn.rs2) as i64) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sltu(vm: &mut VM, insn: &R) {
    let result = if vm.reg(insn.rs1) < vm.reg(insn.rs2) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

// Immediate Opcodes
#[inline(always)]
pub(crate) fn execute_addi(vm: &mut VM, insn: &I) {
    let result = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_xori(vm: &mut VM, insn: &I) {
    let result = vm.reg(insn.rs1) ^ insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_ori(vm: &mut VM, insn: &I) {
    let result = vm.reg(insn.rs1) | insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_andi(vm: &mut VM, insn: &I) {
    let result = vm.reg(insn.rs1) & insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slli(vm: &mut VM, insn: &Sh) {
    let result = vm.reg(insn.rs1) << insn.shamt;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srli(vm: &mut VM, insn: &Sh) {
    let result = vm.reg(insn.rs1) >> insn.shamt;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srai(vm: &mut VM, insn: &Sh) {
    let shift = insn.shamt;
    let val = vm.reg(insn.rs1) as i64;
    let result = (val >> shift) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slti(vm: &mut VM, insn: &I) {
    let result = if (vm.reg(insn.rs1) as i64) < (insn.imm as i64) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sltiu(vm: &mut VM, insn: &I) {
    let result = if vm.reg(insn.rs1) < insn.imm as u64 {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

// Load Opcodes
#[inline(always)]
pub(crate) fn execute_lb(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u8(addr as usize) as u64;
    let result = sext(raw_value, 8);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lbu(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u8(addr as usize) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lh(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u16(addr as usize) as u64;
    let result = sext(raw_value, 16);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lhu(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u16(addr as usize) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lw(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u32(addr as usize) as u64;
    let result = sext(raw_value, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lwu(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u32(addr as usize) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_ld(vm: &mut VM, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u64(addr as usize);
    vm.reg_mut(insn.rd, result);
}

// Store Opcodes
#[inline(always)]
pub(crate) fn execute_sb(vm: &mut VM, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u8(addr as usize, value as u8);
}

#[inline(always)]
pub(crate) fn execute_sh(vm: &mut VM, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u16(addr as usize, value as u16);
}

#[inline(always)]
pub(crate) fn execute_sw(vm: &mut VM, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u32(addr as usize, value as u32);
}

#[inline(always)]
pub(crate) fn execute_sd(vm: &mut VM, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u64(addr as usize, value);
}

// Branch Opcodes
#[inline(always)]
pub(crate) fn execute_beq(vm: &mut VM, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) == vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bne(vm: &mut VM, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) != vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_blt(vm: &mut VM, insn: &B, current_pc: u64) {
    if (vm.reg(insn.rs1) as i64) < (vm.reg(insn.rs2) as i64) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bltu(vm: &mut VM, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) < vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bge(vm: &mut VM, insn: &B, current_pc: u64) {
    if (vm.reg(insn.rs1) as i64) >= (vm.reg(insn.rs2) as i64) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bgeu(vm: &mut VM, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) >= vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

// Jump opcodes
#[inline(always)]
pub(crate) fn execute_jal(vm: &mut VM, insn: &J, current_pc: u64, is_compressed: bool) {
    vm.reg_mut(insn.rd, vm.pc());
    vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
}

#[inline(always)]
pub(crate) fn execute_jalr(vm: &mut VM, insn: &I, current_pc: u64, is_compressed: bool) {
    let target = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, vm.pc());
    vm.set_pc(target);
}

// Lui and Auipc
#[inline(always)]
pub(crate) fn execute_lui(vm: &mut VM, insn: &U) {
    vm.reg_mut(insn.rd, insn.imm as u64);
}

#[inline(always)]
pub(crate) fn execute_auipc(vm: &mut VM, insn: &U, current_pc: u64) {
    let result = current_pc.wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, result);
}

// RV64I Rs
#[inline(always)]
pub(crate) fn execute_addiw(vm: &mut VM, insn: &I) {
    let res = vm.reg(insn.rs1).wrapping_add(insn.imm as u64) & mask(32);
    let result = sext(res, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slliw(vm: &mut VM, insn: &Sh) {
    let val = vm.reg(insn.rs1) << insn.shamt;
    let result = sext(val & mask(32), 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srliw(vm: &mut VM, insn: &Sh) {
    let result = sext((vm.reg(insn.rs1) & mask(32)) >> insn.shamt, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sraiw(vm: &mut VM, insn: &Sh) {
    let shift = insn.shamt;
    let a = (vm.reg(insn.rs1) & mask(32)) as i32;
    let result = (a >> shift) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_addw(vm: &mut VM, insn: &R) {
    let result = sext(
        vm.reg(insn.rs1).wrapping_add(vm.reg(insn.rs2)) & mask(32),
        32,
    );
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_subw(vm: &mut VM, insn: &R) {
    let a = vm.reg(insn.rs1) as i32;
    let b = vm.reg(insn.rs2) as i32;
    let result = a.wrapping_sub(b) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sllw(vm: &mut VM, insn: &R) {
    let a = vm.reg(insn.rs1);
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = sext((a << shift) & mask(32), 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srlw(vm: &mut VM, insn: &R) {
    let a = vm.reg(insn.rs1) & mask(32);
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = sext(a >> shift, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sraw(vm: &mut VM, insn: &R) {
    let a = (vm.reg(insn.rs1) & mask(32)) as i32;
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = (a >> shift) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}
