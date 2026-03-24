/// D-extension
use crate::VM;
use crate::decode::{I, R4, RF, S};
use crate::trace::{MemOp, Tracer};
use crate::util::{classify64, is_snan_f32, is_snan_f64, mask};

#[inline(always)]
pub(crate) fn execute_fmadd_d<T: Tracer>(vm: &mut VM<T>, insn: &R4) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let c = vm.read_f64(insn.rs3);
    let res = a.mul_add(b, c);
    vm.write_f64(insn.rd, res);
    vm.raise_fflags_fma_f64(a, b, c, res);
}

#[inline(always)]
pub(crate) fn execute_fmsub_d<T: Tracer>(vm: &mut VM<T>, insn: &R4) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let c = vm.read_f64(insn.rs3);
    let res = a.mul_add(b, -c);
    vm.write_f64(insn.rd, res);
    vm.raise_fflags_fma_f64(a, b, -c, res);
}

#[inline(always)]
pub(crate) fn execute_fnmsub_d<T: Tracer>(vm: &mut VM<T>, insn: &R4) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let c = vm.read_f64(insn.rs3);
    let res = (-a).mul_add(b, c);
    vm.write_f64(insn.rd, res);
    vm.raise_fflags_fma_f64(-a, b, c, res);
}

#[inline(always)]
pub(crate) fn execute_fnmadd_d<T: Tracer>(vm: &mut VM<T>, insn: &R4) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let c = vm.read_f64(insn.rs3);
    let res = (-a).mul_add(b, -c);
    vm.write_f64(insn.rd, res);
    vm.raise_fflags_fma_f64(-a, b, -c, res);
}

#[inline(always)]
pub(crate) fn execute_fadd_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let mut res = a + b;

    if res.is_nan() && !a.is_nan() && !b.is_nan() {
        res = f64::from_bits(0x7FF8000000000000); // Canonical positive qNaN
    }

    vm.write_f64(insn.rd, res);
    vm.raise_fflags_f64(a, b, res, '+');
}

#[inline(always)]
pub(crate) fn execute_fsub_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let mut res = a - b;

    if res.is_nan() && !a.is_nan() && !b.is_nan() {
        res = f64::from_bits(0x7FF8000000000000);
    }

    vm.write_f64(insn.rd, res);
    vm.raise_fflags_f64(a, b, res, '-');
}

#[inline(always)]
pub(crate) fn execute_fmul_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let mut res = a * b;

    if res.is_nan() && !a.is_nan() && !b.is_nan() {
        res = f64::from_bits(0x7FF8000000000000);
    }

    vm.write_f64(insn.rd, res);
    vm.raise_fflags_f64(a, b, res, '*');
}

#[inline(always)]
pub(crate) fn execute_fdiv_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);
    let mut res = a / b;

    if res.is_nan() && !a.is_nan() && !b.is_nan() {
        res = f64::from_bits(0x7FF8000000000000);
    }

    vm.write_f64(insn.rd, res);
    vm.raise_fflags_f64(a, b, res, '/');
}

#[inline(always)]
pub(crate) fn execute_fsqrt_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);

    if is_snan_f64(a) || (a < 0.0 && !a.is_nan()) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    let mut res = a.sqrt();

    if res.is_nan() && !a.is_nan() {
        res = f64::from_bits(0x7FF8000000000000);
    }

    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fsgnj_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let sign = vm.read_f64(insn.rs2).to_bits() & (1 << 63);
    let val = vm.read_f64(insn.rs1).to_bits() & mask(63);
    let res = f64::from_bits(sign | val);
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fsgnjn_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let sign = (vm.read_f64(insn.rs2).to_bits() ^ (1 << 63)) & (1 << 63);
    let val = vm.read_f64(insn.rs1).to_bits() & mask(63);
    let res = f64::from_bits(sign | val);
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fsgnjx_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let sign = (vm.read_f64(insn.rs1).to_bits() & (1 << 63))
        ^ (vm.read_f64(insn.rs2).to_bits() & (1 << 63));
    let val = vm.read_f64(insn.rs1).to_bits() & mask(63);
    let res = f64::from_bits(sign | val);
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fmin_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);

    if is_snan_f64(a) || is_snan_f64(b) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    let res = if a.is_nan() && b.is_nan() {
        f64::from_bits(0x7FF8000000000000) // Canonical NaN
    } else if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else if a == 0.0 && b == 0.0 {
        if a.to_bits() & 0x8000000000000000 != 0 {
            a
        } else {
            b
        }
    } else {
        a.min(b)
    };
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fmax_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);

    if is_snan_f64(a) || is_snan_f64(b) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    let res = if a.is_nan() && b.is_nan() {
        f64::from_bits(0x7FF8000000000000)
    } else if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else if a == 0.0 && b == 0.0 {
        if a.to_bits() & 0x8000000000000000 == 0 {
            a
        } else {
            b
        }
    } else {
        a.max(b)
    };
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fcvt_sd<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let res = a as f32;

    // Set NX if precision was lost
    if !a.is_nan() && !a.is_infinite() && (res as f64) != a {
        vm.fcsr_reg |= 0b00001;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    // Set NV for sNaN
    if is_snan_f64(a) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    vm.write_f32(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_fcvt_ds<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f32(insn.rs1);

    // Set NV for sNaN
    if is_snan_f32(a) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    let res = a as f64;
    vm.write_f64(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_feq_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);

    if is_snan_f64(a) || is_snan_f64(b) {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
    }

    let res = if a.is_nan() || b.is_nan() {
        0
    } else {
        (a == b) as u64
    };

    vm.reg_mut(insn.rd, res);
}

#[inline(always)]
pub(crate) fn execute_flt_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);

    if a.is_nan() || b.is_nan() {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
        vm.reg_mut(insn.rd, 0);
    } else {
        vm.reg_mut(insn.rd, (a < b) as u64);
    }
}

#[inline(always)]
pub(crate) fn execute_fle_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = vm.read_f64(insn.rs1);
    let b = vm.read_f64(insn.rs2);

    if a.is_nan() || b.is_nan() {
        vm.fcsr_reg |= 0b10000;
        vm.tracer.record_csr_reg(vm.fcsr_reg);
        vm.reg_mut(insn.rd, 0);
    } else {
        vm.reg_mut(insn.rd, (a <= b) as u64);
    }
}

#[inline(always)]
pub(crate) fn execute_fclass_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = classify64(vm.read_f64(insn.rs1).to_bits());
    vm.reg_mut(insn.rd, val);
}

#[inline(always)]
pub(crate) fn execute_fcvt_wd<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.read_f64(insn.rs1);

    let (result, flags): (i32, u32) = if val.is_nan() {
        (i32::MAX, 0b10000)
    } else if val >= (i32::MAX as f64) + 1.0 {
        (i32::MAX, 0b10000)
    } else if val < (i32::MIN as f64) {
        (i32::MIN, 0b10000)
    } else {
        let truncated = val.trunc();
        let int_val = val as i32;
        let inexact = if val != truncated { 0b00001 } else { 0 };
        (int_val, inexact)
    };

    vm.fcsr_reg |= flags;
    vm.reg_mut(insn.rd, result as i64 as u64);
    vm.tracer.record_csr_reg(vm.fcsr_reg);
}

#[inline(always)]
pub(crate) fn execute_fcvt_wu_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.read_f64(insn.rs1);

    let (result, flags): (u32, u32) = if val.is_nan() {
        (u32::MAX, 0b10000)
    } else if val <= -1.0 {
        (0_u32, 0b10000) // NV - changed from < to <=
    } else if val < 0.0 {
        (0_u32, 0b00001) // NX only
    } else if val >= (u32::MAX as f64) + 1.0 {
        (u32::MAX, 0b10000)
    } else {
        let truncated = val.trunc();
        let int_val = truncated as u32;
        let inexact = if val != truncated { 0b00001 } else { 0 };
        (int_val, inexact)
    };

    vm.fcsr_reg |= flags;
    vm.reg_mut(insn.rd, result as i32 as i64 as u64);
    vm.tracer.record_csr_reg(vm.fcsr_reg);
}

#[inline(always)]
pub(crate) fn execute_fcvt_dw<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = (vm.reg(insn.rs1) as i32) as f64;
    vm.write_f64(insn.rd, a);
}

#[inline(always)]
pub(crate) fn execute_fcvt_dwu<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let a = (vm.reg(insn.rs1) as u32) as f64;
    vm.write_f64(insn.rd, a);
}

#[inline(always)]
pub(crate) fn execute_fld<T: Tracer>(vm: &mut VM<T>, insn: &I) {
    let addr = (vm.reg(insn.rs1).wrapping_add(insn.imm as u64)) as usize;
    let val = f64::from_bits(vm.load_u64(addr));
    vm.write_f64(insn.rd, val);
}

#[inline(always)]
pub(crate) fn execute_fsd<T: Tracer>(vm: &mut VM<T>, insn: &S) {
    let data = vm.read_f64(insn.rs2).to_le_bytes();
    let addr = (vm.reg(insn.rs1).wrapping_add(insn.imm as u64)) as usize;
    vm.store_u64(addr, u64::from_le_bytes(data));
    vm.tracer.record_mem_op(MemOp::StoreDouble {
        addr: addr as u64,
        value: u64::from_le_bytes(data),
    });
}

#[inline(always)]
pub(crate) fn execute_fcvt_ld<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.read_f64(insn.rs1);

    let (result, flags): (i64, u32) = if val.is_nan() {
        (i64::MAX, 0b10000)
    } else if val >= (i64::MAX as f64) {
        (i64::MAX, 0b10000)
    } else if val < (i64::MIN as f64) {
        (i64::MIN, 0b10000)
    } else {
        let truncated = val.trunc();
        let int_val = val as i64;
        let inexact = if val != truncated { 0b00001 } else { 0 };
        (int_val, inexact)
    };

    vm.fcsr_reg |= flags;
    vm.reg_mut(insn.rd, result as u64);
    vm.tracer.record_csr_reg(vm.fcsr_reg);
}

#[inline(always)]
pub(crate) fn execute_fcvt_lu_d<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.read_f64(insn.rs1);

    let (result, flags): (u64, u32) = if val.is_nan() {
        (u64::MAX, 0b10000)
    } else if val <= -1.0 {
        (0_u64, 0b10000) // NV - changed from < to <=
    } else if val < 0.0 {
        (0_u64, 0b00001) // NX only
    } else if val >= (u64::MAX as f64) {
        (u64::MAX, 0b10000)
    } else {
        let truncated = val.trunc();
        let int_val = truncated as u64;
        let inexact = if val != truncated { 0b00001 } else { 0 };
        (int_val, inexact)
    };

    vm.fcsr_reg |= flags;
    vm.reg_mut(insn.rd, result);
    vm.tracer.record_csr_reg(vm.fcsr_reg);
}

#[inline(always)]
pub(crate) fn execute_fmv_xd<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.read_f64(insn.rs1).to_bits();
    vm.reg_mut(insn.rd, val);
}

#[inline(always)]
pub(crate) fn execute_fcvt_dl<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = (vm.reg(insn.rs1) as i64) as f64;
    vm.write_f64(insn.rd, val);
}

#[inline(always)]
pub(crate) fn execute_fcvt_dlu<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = vm.reg(insn.rs1) as f64;
    vm.write_f64(insn.rd, val);
}

#[inline(always)]
pub(crate) fn execute_fmv_dx<T: Tracer>(vm: &mut VM<T>, insn: &RF) {
    let val = f64::from_bits(vm.reg(insn.rs1));
    vm.write_f64(insn.rd, val);
}
