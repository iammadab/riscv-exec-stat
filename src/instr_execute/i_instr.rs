// Register Opcodes

use crate::VM;
use crate::decode::{B, I, J, R, S, Sh, U};
use crate::trace::{MemOp, Tracer};
use crate::util::{mask, sext};

#[inline(always)]
pub(crate) fn execute_add<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1).wrapping_add(vm.reg(insn.rs2));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sub<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1).wrapping_sub(vm.reg(insn.rs2));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_xor<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1) ^ vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_or<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1) | vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_and<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1) & vm.reg(insn.rs2);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sll<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1) << (vm.reg(insn.rs2) & mask(6));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srl<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = vm.reg(insn.rs1) >> (vm.reg(insn.rs2) & mask(6));
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sra<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let val = vm.reg(insn.rs1) as i64;
    let result = (val >> (vm.reg(insn.rs2) & mask(6))) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slt<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = if (vm.reg(insn.rs1) as i64) < (vm.reg(insn.rs2) as i64) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sltu<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = if vm.reg(insn.rs1) < vm.reg(insn.rs2) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

// Immediate Opcodes
#[inline(always)]
pub(crate) fn execute_addi<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_xori<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = vm.reg(insn.rs1) ^ insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_ori<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = vm.reg(insn.rs1) | insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_andi<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = vm.reg(insn.rs1) & insn.imm as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slli<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let result = vm.reg(insn.rs1) << insn.shamt;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srli<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let result = vm.reg(insn.rs1) >> insn.shamt;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srai<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let shift = insn.shamt;
    let val = vm.reg(insn.rs1) as i64;
    let result = (val >> shift) as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slti<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = if (vm.reg(insn.rs1) as i64) < (insn.imm as i64) {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sltiu<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let result = if vm.reg(insn.rs1) < insn.imm as u64 {
        1
    } else {
        0
    };
    vm.reg_mut(insn.rd, result);
}

// Load Opcodes
#[inline(always)]
pub(crate) fn execute_lb<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u8(addr as usize) as u64;
    let result = sext(raw_value, 8);
    vm.tracer.record_mem_op(MemOp::LoadByte {
        addr,
        value: raw_value as u8,
        signed: true,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lbu<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u8(addr as usize) as u64;
    vm.tracer.record_mem_op(MemOp::LoadByte {
        addr,
        value: result as u8,
        signed: false,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lh<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u16(addr as usize) as u64;
    let result = sext(raw_value, 16);
    vm.tracer.record_mem_op(MemOp::LoadHalf {
        addr,
        value: raw_value as u16,
        signed: true,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lhu<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u16(addr as usize) as u64;
    vm.tracer.record_mem_op(MemOp::LoadHalf {
        addr,
        value: result as u16,
        signed: false,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lw<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let raw_value = vm.load_u32(addr as usize) as u64;
    let result = sext(raw_value, 32);
    vm.tracer.record_mem_op(MemOp::LoadWord {
        addr,
        value: raw_value as u32,
        signed: true,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lwu<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u32(addr as usize) as u64;
    vm.tracer.record_mem_op(MemOp::LoadWord {
        addr,
        value: result as u32,
        signed: false,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_ld<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let result = vm.load_u64(addr as usize);
    vm.tracer.record_mem_op(MemOp::LoadDouble {
        addr,
        value: result,
    });
    vm.reg_mut(insn.rd, result);
}

// Store Opcodes
#[inline(always)]
pub(crate) fn execute_sb<T: Tracer>(vm: &mut VM<T>, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u8(addr as usize, value as u8);
    vm.tracer.record_mem_op(MemOp::StoreByte {
        addr,
        value: value as u8,
    });
}

#[inline(always)]
pub(crate) fn execute_sh<T: Tracer>(vm: &mut VM<T>, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u16(addr as usize, value as u16);
    vm.tracer.record_mem_op(MemOp::StoreHalf {
        addr,
        value: value as u16,
    });
}

#[inline(always)]
pub(crate) fn execute_sw<T: Tracer>(vm: &mut VM<T>, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u32(addr as usize, value as u32);
    vm.tracer.record_mem_op(MemOp::StoreWord {
        addr,
        value: value as u32,
    });
}

#[inline(always)]
pub(crate) fn execute_sd<T: Tracer>(vm: &mut VM<T>, insn: &S) {
    let addr = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    let value = vm.reg(insn.rs2);
    vm.store_u64(addr as usize, value);
    vm.tracer.record_mem_op(MemOp::StoreDouble { addr, value });
}

// Branch Opcodes
#[inline(always)]
pub(crate) fn execute_beq<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) == vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bne<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) != vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_blt<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if (vm.reg(insn.rs1) as i64) < (vm.reg(insn.rs2) as i64) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bltu<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) < vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bge<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if (vm.reg(insn.rs1) as i64) >= (vm.reg(insn.rs2) as i64) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

#[inline(always)]
pub(crate) fn execute_bgeu<T: Tracer>(vm: &mut VM<T>, insn: &B, current_pc: u64) {
    if vm.reg(insn.rs1) >= vm.reg(insn.rs2) {
        vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
    }
}

// Jump opcodes
#[inline(always)]
pub(crate) fn execute_jal<T: Tracer>(
    vm: &mut VM<T>,
    insn: &J,
    current_pc: u64,
    is_compressed: bool,
) {
    vm.reg_mut(insn.rd, vm.pc());
    vm.set_pc(current_pc.wrapping_add(insn.imm as u64));
}

#[inline(always)]
pub(crate) fn execute_jalr<T: Tracer>(
    vm: &mut VM<T>,
    insn: &I,
    current_pc: u64,
    is_compressed: bool,
) {
    let target = vm.reg(insn.rs1).wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, vm.pc());
    vm.set_pc(target);
}

// Lui and Auipc
#[inline(always)]
pub(crate) fn execute_lui<T: Tracer>(vm: &mut VM<T>, insn: &U) {
    vm.reg_mut(insn.rd, insn.imm as u64);
}

#[inline(always)]
pub(crate) fn execute_auipc<T: Tracer>(vm: &mut VM<T>, insn: &U, current_pc: u64) {
    let result = current_pc.wrapping_add(insn.imm as u64);
    vm.reg_mut(insn.rd, result);
}

// RV64I Rs
#[inline(always)]
pub(crate) fn execute_addiw<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let res = vm.reg(insn.rs1).wrapping_add(insn.imm as u64) & mask(32);
    let result = sext(res, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_slliw<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let val = vm.reg(insn.rs1) << insn.shamt;
    let result = sext(val & mask(32), 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srliw<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let result = sext((vm.reg(insn.rs1) & mask(32)) >> insn.shamt, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sraiw<T: Tracer>(vm: &mut VM<T>, insn: &Sh) {
    let shift = insn.shamt;
    let a = (vm.reg(insn.rs1) & mask(32)) as i32;
    let result = (a >> shift) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_addw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let result = sext(
        vm.reg(insn.rs1).wrapping_add(vm.reg(insn.rs2)) & mask(32),
        32,
    );
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_subw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1) as i32;
    let b = vm.reg(insn.rs2) as i32;
    let result = a.wrapping_sub(b) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sllw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1);
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = sext((a << shift) & mask(32), 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_srlw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1) & mask(32);
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = sext(a >> shift, 32);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sraw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = (vm.reg(insn.rs1) & mask(32)) as i32;
    let shift = vm.reg(insn.rs2) & mask(5);
    let result = (a >> shift) as i64 as u64;
    vm.reg_mut(insn.rd, result);
}
