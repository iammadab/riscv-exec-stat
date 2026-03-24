use crate::VM;
/// A Extension - Load Reserved / Store Conditional
use crate::decode::R;
use crate::trace::{MemOp, Tracer};
use crate::util::{mask, sext};

#[inline(always)]
pub(crate) fn execute_lr_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let value = vm.load_u32(addr as usize) as u64;
    let result = sext(value, 32);
    vm.reservation_set = addr;
    vm.tracer.record_reservation(addr);
    vm.tracer.record_mem_op(MemOp::LoadReservedWord {
        addr,
        value: value as u32,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_lr_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let value = vm.load_u64(addr as usize);
    vm.reservation_set = addr;
    vm.tracer.record_reservation(addr);
    vm.tracer
        .record_mem_op(MemOp::LoadReservedDouble { addr, value });
    vm.reg_mut(insn.rd, value);
}

#[inline(always)]
pub(crate) fn execute_sc_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let value = vm.reg(insn.rs2) & mask(32);
    let success = addr == vm.reservation_set;
    if success {
        vm.store_u32(addr as usize, value as u32);
    }
    let result = if success { 0 } else { 1 };
    vm.reservation_set = 0;
    vm.tracer.record_mem_op(MemOp::StoreConditionalWord {
        addr,
        value: value as u32,
        success,
    });
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_sc_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let value = vm.reg(insn.rs2);
    let success = addr == vm.reservation_set;
    if success {
        vm.store_u64(addr as usize, value);
    }
    let result = if success { 0 } else { 1 };
    vm.reservation_set = 0;
    vm.tracer.record_mem_op(MemOp::StoreConditionalDouble {
        addr,
        value,
        success,
    });
    vm.reg_mut(insn.rd, result);
}

// A Extension - Atomic Memory Operations (Word)
#[inline(always)]
pub(crate) fn execute_amo_swap_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as u64;
    let write_value = vm.reg(insn.rs2) & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, sext(read_value, 32));
}

#[inline(always)]
pub(crate) fn execute_amo_add_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = (read_value.wrapping_add(rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_xor_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = ((read_value ^ rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_and_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = ((read_value & rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_or_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = ((read_value | rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_min_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = (read_value.min(rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_max_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as i32;
    let rs2_val = (vm.reg(insn.rs2) & mask(32)) as i32;
    let write_value = (read_value.max(rs2_val) as i64) as u64 & mask(32);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, (read_value as i64) as u64);
}

#[inline(always)]
pub(crate) fn execute_amo_minu_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as u64;
    let rs2_val = vm.reg(insn.rs2) & mask(32);
    let write_value = read_value.min(rs2_val);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, sext(read_value, 32));
}

#[inline(always)]
pub(crate) fn execute_amo_maxu_w<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u32(addr as usize) as u64;
    let rs2_val = vm.reg(insn.rs2) & mask(32);
    let write_value = read_value.max(rs2_val);
    vm.store_u32(addr as usize, write_value as u32);
    vm.tracer.record_mem_op(MemOp::AtomicWord {
        addr,
        read_value: read_value as u32,
        write_value: write_value as u32,
    });
    vm.reg_mut(insn.rd, sext(read_value, 32));
}

// A Extension - Atomic Memory Operations (Double)
#[inline(always)]
pub(crate) fn execute_amo_swap_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let write_value = vm.reg(insn.rs2);
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_add_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value.wrapping_add(rs2_val);
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_xor_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value ^ rs2_val;
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_and_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value & rs2_val;
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_or_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value | rs2_val;
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_min_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2) as i64;
    let write_value = (read_value as i64).min(rs2_val) as u64;
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_max_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2) as i64;
    let write_value = (read_value as i64).max(rs2_val) as u64;
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_minu_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value.min(rs2_val);
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}

#[inline(always)]
pub(crate) fn execute_amo_maxu_d<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let addr = vm.reg(insn.rs1);
    let read_value = vm.load_u64(addr as usize);
    let rs2_val = vm.reg(insn.rs2);
    let write_value = read_value.max(rs2_val);
    vm.store_u64(addr as usize, write_value);
    vm.tracer.record_mem_op(MemOp::AtomicDouble {
        addr,
        read_value,
        write_value,
    });
    vm.reg_mut(insn.rd, read_value);
}
