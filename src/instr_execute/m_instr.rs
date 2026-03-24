// M Extension - Multiplication

use crate::VM;
use crate::decode::R;
use crate::trace::Tracer;
use crate::util::{mask, sext};

#[inline(always)]
pub(crate) fn execute_mul<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1) as i64;
    let b = vm.reg(insn.rs2) as i64;
    let full = (a as i128).wrapping_mul(b as i128);
    let result = a.wrapping_mul(b) as u64;
    vm.tracer.record_mul(result, (full >> 64) as u64);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_mulh<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = (vm.reg(insn.rs1) as i64) as i128;
    let b = (vm.reg(insn.rs2) as i64) as i128;
    let full = a.wrapping_mul(b);
    let lo = full as u64;
    let hi = (full >> 64) as u64;
    vm.tracer.record_mul(lo, hi);
    vm.reg_mut(insn.rd, hi);
}

#[inline(always)]
pub(crate) fn execute_mulhsu<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = (vm.reg(insn.rs1) as i64) as i128;
    let b = (vm.reg(insn.rs2) as u128) as i128;
    let full = a.wrapping_mul(b);
    let lo = full as u64;
    let hi = (full >> 64) as u64;
    vm.tracer.record_mul(lo, hi);
    vm.reg_mut(insn.rd, hi);
}

#[inline(always)]
pub(crate) fn execute_mulhu<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1) as u128;
    let b = vm.reg(insn.rs2) as u128;
    let full = a.wrapping_mul(b);
    let lo = full as u64;
    let hi = (full >> 64) as u64;
    vm.tracer.record_mul(lo, hi);
    vm.reg_mut(insn.rd, hi);
}

#[inline(always)]
pub(crate) fn execute_mulw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let a = vm.reg(insn.rs1);
    let b = vm.reg(insn.rs2);
    let product = a.wrapping_mul(b);
    let result = (((product & mask(32)) as i32) as i64) as u64;
    vm.tracer.record_mul(product & mask(32), 0);
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_div<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = vm.reg(insn.rs1) as i64;
    let divisor = vm.reg(insn.rs2) as i64;
    let result = if divisor == 0 {
        u64::MAX
    } else if dividend == i64::MIN && divisor == -1 {
        dividend as u64
    } else {
        dividend.wrapping_div(divisor) as u64
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_divu<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = vm.reg(insn.rs1);
    let divisor = vm.reg(insn.rs2);
    let result = if divisor == 0 {
        u64::MAX
    } else {
        dividend.wrapping_div(divisor)
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_rem<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = vm.reg(insn.rs1) as i64;
    let divisor = vm.reg(insn.rs2) as i64;
    let result = if divisor == 0 {
        dividend as u64
    } else if dividend == i64::MIN && divisor == -1 {
        0
    } else {
        dividend.wrapping_rem(divisor) as u64
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_remu<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = vm.reg(insn.rs1);
    let divisor = vm.reg(insn.rs2);
    let result = if divisor == 0 {
        dividend
    } else {
        dividend.wrapping_rem(divisor)
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_divw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = (vm.reg(insn.rs1) & mask(32)) as i32;
    let divisor = (vm.reg(insn.rs2) & mask(32)) as i32;
    let result = if divisor == 0 {
        u64::MAX
    } else if dividend == i32::MIN && divisor == -1 {
        (dividend as i64) as u64
    } else {
        (dividend.wrapping_div(divisor) as i64) as u64
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_divuw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = (vm.reg(insn.rs1) & mask(32)) as u32;
    let divisor = (vm.reg(insn.rs2) & mask(32)) as u32;
    let result = if divisor == 0 {
        u64::MAX
    } else {
        sext(dividend.wrapping_div(divisor) as u64, 32)
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_remw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = (vm.reg(insn.rs1) & mask(32)) as i32;
    let divisor = (vm.reg(insn.rs2) & mask(32)) as i32;
    let result = if divisor == 0 {
        (dividend as i64) as u64
    } else if dividend == i32::MIN && divisor == -1 {
        0
    } else {
        (dividend.wrapping_rem(divisor) as i64) as u64
    };
    vm.reg_mut(insn.rd, result);
}

#[inline(always)]
pub(crate) fn execute_remuw<T: Tracer>(vm: &mut VM<T>, insn: &R) {
    let dividend = (vm.reg(insn.rs1) & mask(32)) as u32;
    let divisor = (vm.reg(insn.rs2) & mask(32)) as u32;
    let result = if divisor == 0 {
        sext(dividend as u64, 32)
    } else {
        sext(dividend.wrapping_rem(divisor) as u64, 32)
    };
    vm.reg_mut(insn.rd, result);
}
