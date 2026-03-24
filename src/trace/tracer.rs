//! Zero-cost abstraction for execution tracing.
use super::primitives::{ExecutionTrace, InstrFlags, MemOp, TraceRow};
use crate::decode::{decode, Instruction};

/// Trait for instruction execution tracing.
///
/// This trait enables zero-cost abstraction for tracing:
/// - When using `NoopTracer`, all methods are inlined and optimized away
/// - When using `FullTracer`, a complete execution trace is built
///
/// The trait is designed to be called at specific points during instruction
/// execution, allowing the tracer to capture the complete CPU state.
pub trait Tracer: Default + Sized {
    /// Begin tracing a new instruction.
    ///
    /// Called at the start of instruction execution with the current CPU state.
    fn begin_instruction(
        &mut self,
        clk: u64,
        pc: u64,
        x_regs: &[u64; 32],
        f_regs: &[u64; 32],
        raw_instr: u32,
        instr: &Instruction,
    );

    /// Record destination register write.
    fn record_rd(&mut self, rd: u8, value: u64);

    /// Record next PC value.
    fn record_next_pc(&mut self, next_pc: u64);

    /// Record memory operation.
    fn record_mem_op(&mut self, op: MemOp);

    /// Record multiplication intermediate values for verification.
    fn record_mul(&mut self, lo: u64, hi: u64);

    /// Record reservation address for LR/SC atomic operations.
    fn record_reservation(&mut self, addr: u64);

    /// Record Control Status Register for F-extension
    fn record_csr_reg(&mut self, flag: u32);

    /// Mark instruction as causing halt.
    fn record_halt(&mut self);

    /// Commit current instruction's trace row.
    ///
    /// Called at the end of instruction execution to finalize the trace row.
    fn commit(&mut self);

    /// Finalize and return the execution trace.
    ///
    /// Consumes the tracer and returns the complete execution trace,
    /// or `None` if tracing was not enabled.
    fn finalize(
        self,
        final_x_regs: [u64; 32],
        final_f_regs: [u64; 32],
        final_pc: u64,
    ) -> Option<ExecutionTrace>;

    /// Check if this tracer actually records traces.
    ///
    /// Returns `false` for `NoopTracer`, `true` for `FullTracer`.
    /// Can be used for conditional logic that should only run when tracing.
    fn is_active(&self) -> bool;
}

/// A no-op tracer that does nothing.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopTracer;

impl Tracer for NoopTracer {
    #[inline(always)]
    fn begin_instruction(
        &mut self,
        _clk: u64,
        _pc: u64,
        _regs: &[u64; 32],
        _f_regs: &[u64; 32],
        _raw_instr: u32,
        _instr: &Instruction,
    ) {
    }

    #[inline(always)]
    fn record_rd(&mut self, _rd: u8, _value: u64) {}

    #[inline(always)]
    fn record_next_pc(&mut self, _next_pc: u64) {}

    #[inline(always)]
    fn record_mem_op(&mut self, _op: MemOp) {}

    #[inline(always)]
    fn record_mul(&mut self, _lo: u64, _hi: u64) {}

    #[inline(always)]
    fn record_reservation(&mut self, _addr: u64) {}

    #[inline(always)]
    fn record_csr_reg(&mut self, flag: u32) {}

    #[inline(always)]
    fn record_halt(&mut self) {}

    #[inline(always)]
    fn commit(&mut self) {}

    #[inline(always)]
    fn finalize(
        self,
        _final_x_regs: [u64; 32],
        _final_f_regs: [u64; 32],
        _final_pc: u64,
    ) -> Option<ExecutionTrace> {
        None
    }

    #[inline(always)]
    fn is_active(&self) -> bool {
        false
    }
}

/// A full tracer that builds a complete execution trace.
///
/// This tracer captures all CPU state changes during execution,
/// building a `TraceRow` for each instruction executed.
#[derive(Debug, Default)]
pub struct FullTracer {
    /// The execution trace being built.
    trace: ExecutionTrace,
    /// Current instruction's trace row being built.
    current: Option<TraceRow>,
}

impl FullTracer {
    /// Create a new full tracer with initial state.
    pub fn new(initial_pc: u64, initial_x_regs: [u64; 32], initial_f_regs: [u64; 32]) -> Self {
        Self {
            trace: ExecutionTrace::new(initial_pc, initial_x_regs, initial_f_regs),
            current: None,
        }
    }

    /// Get a reference to the current trace.
    pub fn trace(&self) -> &ExecutionTrace {
        &self.trace
    }

    /// Get the number of rows recorded so far.
    pub fn len(&self) -> usize {
        self.trace.len()
    }

    /// Check if no rows have been recorded.
    pub fn is_empty(&self) -> bool {
        self.trace.is_empty()
    }
}

impl Tracer for FullTracer {
    fn begin_instruction(
        &mut self,
        clk: u64,
        pc: u64,
        regs: &[u64; 32],
        f_regs: &[u64; 32],
        raw_instr: u32,
        instr: &Instruction,
    ) {
        // Check if instruction is integer or floating point instruction
        let rs1_val = if instr.is_integer_insn() {
            if instr.rs1() == 0 {
                0
            } else {
                regs[instr.rs1() as usize]
            }
        } else if instr.is_fp_insn() {
            f_regs[instr.rs1() as usize]
        } else {
            panic!("Instruction not accounted for: {:?}", instr);
        };

        let rs2_val = if instr.is_integer_insn() {
            if instr.rs2() == 0 {
                0
            } else {
                regs[instr.rs2() as usize]
            }
        } else if instr.is_fp_insn() {
            f_regs[instr.rs2() as usize]
        } else {
            panic!("Instruction not accounted for: {:?}", instr);
        };

        let rs3_val = if instr.is_integer_insn() {
            if instr.rs3() == 0 {
                0
            } else {
                regs[instr.rs3() as usize]
            }
        } else if instr.is_fp_insn() {
            f_regs[instr.rs3() as usize]
        } else {
            panic!("Instruction not accounted for: {:?}", instr);
        };

        self.current = Some(TraceRow {
            clk,
            pc,
            next_pc: pc.wrapping_add(4), // Default: sequential execution, use record_next_pc should the instruction be non-sequential liek branch or jump
            regs: *regs,
            f_regs: *f_regs,
            raw_instr,
            opcode: *instr,
            flags: InstrFlags::from_opcode(&instr),
            rs1: instr.rs1(),
            rs2: instr.rs2(),
            rs3: instr.rs3(),
            rd: instr.rd(),
            imm: instr.imm(),
            rm: instr.rm(),
            rs1_val,
            rs2_val,
            rs3_val,
            rd_val: 0,
            mem_op: MemOp::None,
            mul_lo: 0,
            mul_hi: 0,
            reservation_addr: 0,
            csr_reg: 0,
            halted: false,
        });
    }

    fn record_rd(&mut self, rd: u8, value: u64) {
        if let Some(ref mut row) = self.current {
            row.rd = rd;
            row.rd_val = value;
        }
    }

    fn record_next_pc(&mut self, next_pc: u64) {
        if let Some(ref mut row) = self.current {
            row.next_pc = next_pc;
        }
    }

    fn record_mem_op(&mut self, op: MemOp) {
        if let Some(ref mut row) = self.current {
            row.mem_op = op;
        }
    }

    fn record_mul(&mut self, lo: u64, hi: u64) {
        if let Some(ref mut row) = self.current {
            row.mul_lo = lo;
            row.mul_hi = hi;
        }
    }

    fn record_reservation(&mut self, addr: u64) {
        if let Some(ref mut row) = self.current {
            row.reservation_addr = addr;
        }
    }

    fn record_csr_reg(&mut self, flag: u32) {
        if let Some(ref mut row) = self.current {
            row.csr_reg = flag;
        }
    }

    fn record_halt(&mut self) {
        if let Some(ref mut row) = self.current {
            row.halted = true;
        }
    }

    fn commit(&mut self) {
        if let Some(row) = self.current.take() {
            let trace_row = TraceRow {
                clk: row.clk,
                pc: row.pc,
                next_pc: row.next_pc,
                raw_instr: row.raw_instr,
                opcode: row.opcode,
                flags: row.flags,
                regs: row.regs,
                f_regs: row.f_regs,
                rs1: row.rs1,
                rs2: row.rs2,
                rs3: row.rs3,
                rd: row.rd,
                imm: row.imm,
                rm: row.rm,
                rs1_val: row.rs1_val,
                rs2_val: row.rs2_val,
                rs3_val: row.rs3_val,
                rd_val: row.rd_val,
                mem_op: row.mem_op,
                mul_lo: row.mul_lo,
                mul_hi: row.mul_hi,
                reservation_addr: row.reservation_addr,
                csr_reg: row.csr_reg,
                halted: row.halted,
            };
            self.trace.push(trace_row);
        }
    }

    fn finalize(
        mut self,
        final_x_regs: [u64; 32],
        final_f_regs: [u64; 32],
        final_pc: u64,
    ) -> Option<ExecutionTrace> {
        self.trace.final_x_regs = final_x_regs;
        self.trace.final_f_regs = final_f_regs;
        self.trace.final_pc = final_pc;
        Some(self.trace)
    }

    #[inline(always)]
    fn is_active(&self) -> bool {
        true
    }
}

pub type DefaultTracer = NoopTracer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_tracer_is_zero_cost() {
        let mut tracer = NoopTracer;
        let regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x00000033;
        let insn = decode(raw_instr);

        // These should all compile to nothing
        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_rd(3, 42);
        tracer.record_next_pc(0x1004);
        tracer.commit();

        assert!(!tracer.is_active());
        assert!(tracer.finalize([0; 32], [0; 32], 0).is_none());
    }

    #[test]
    fn test_full_tracer_captures_state() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let mut regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x002080b3;
        let insn = decode(raw_instr);
        regs[1] = 10;
        regs[2] = 20;
        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_rd(3, 30);
        tracer.record_next_pc(0x1004);
        tracer.commit();

        assert!(tracer.is_active());
        assert_eq!(tracer.len(), 1);

        let trace = tracer.finalize([0; 32], [0; 32], 0x1004).unwrap();
        assert_eq!(trace.rows.len(), 1);

        let row = &trace.rows[0];
        assert_eq!(row.clk, 0);
        assert_eq!(row.pc, 0x1000);
        assert_eq!(row.next_pc, 0x1004);
        assert_eq!(row.rs1_val, 10);
        assert_eq!(row.rs2_val, 20);
        assert_eq!(row.rd_val, 30);
    }

    #[test]
    fn test_full_tracer_memory_op() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x00000003;
        let insn = decode(raw_instr);

        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_mem_op(MemOp::LoadWord {
            addr: 0x2000,
            value: 0x12345678,
            signed: true,
        });
        tracer.record_rd(2, 0x12345678);
        tracer.commit();

        let trace = tracer.finalize([0; 32], [0; 32], 0x1004).unwrap();
        let row = &trace.rows[0];

        match row.mem_op {
            MemOp::LoadWord {
                addr,
                value,
                signed,
            } => {
                assert_eq!(addr, 0x2000);
                assert_eq!(value, 0x12345678);
                assert!(signed);
            }
            _ => panic!("Expected LoadWord"),
        }
    }

    #[test]
    fn test_full_tracer_halt() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x00000073;
        let insn = decode(raw_instr);

        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_halt();
        tracer.commit();

        let trace = tracer.finalize([0; 32], [0; 32], 0x1000).unwrap();
        assert!(trace.rows[0].halted);
    }

    #[test]
    fn test_full_tracer_mul_intermediate() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x00000033;
        let insn = decode(raw_instr);

        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_mul(0xDEADBEEF, 0xCAFEBABE);
        tracer.commit();

        let trace = tracer.finalize([0; 32], [0; 32], 0x1004).unwrap();
        let row = &trace.rows[0];
        assert_eq!(row.mul_lo, 0xDEADBEEF);
        assert_eq!(row.mul_hi, 0xCAFEBABE);
    }

    #[test]
    fn test_multiple_instructions() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let regs = [0u64; 32];
        let f_regs = [0u64; 32];
        let raw_instr = 0x00000033;
        let insn = decode(raw_instr);

        for i in 0..5 {
            tracer.begin_instruction(i, 0x1000 + (i * 4), &regs, &f_regs, raw_instr, &insn);
            tracer.record_next_pc(0x1000 + ((i + 1) * 4));
            tracer.commit();
        }

        assert_eq!(tracer.len(), 5);

        let trace = tracer.finalize([0; 32], [0; 32], 0x1014).unwrap();
        assert_eq!(trace.total_cycles, 5);
    }

    #[test]
    fn test_full_tracer_for_f_extension() {
        let mut tracer = FullTracer::new(0x1000, [0u64; 32], [0u64; 32]);
        let regs = [0u64; 32];
        let mut f_regs = [0u64; 32];
        let raw_instr = 0x0020f1d3;
        let insn = decode(raw_instr);
        f_regs[1] = u64::from_le_bytes(10_f64.to_le_bytes());
        f_regs[2] = u64::from_le_bytes(20_f64.to_le_bytes());

        tracer.begin_instruction(0, 0x1000, &regs, &f_regs, raw_instr, &insn);
        tracer.record_rd(3, u64::from_le_bytes(30_f64.to_le_bytes()));
        tracer.record_next_pc(0x1004);
        tracer.commit();

        assert!(tracer.is_active());
        assert_eq!(tracer.len(), 1);

        let trace = tracer.finalize([0; 32], [0; 32], 0x1004).unwrap();
        assert_eq!(trace.rows.len(), 1);

        let row = &trace.rows[0];
        assert_eq!(row.clk, 0);
        assert_eq!(row.pc, 0x1000);
        assert_eq!(row.next_pc, 0x1004);
        assert_eq!(f64::from_le_bytes(row.rs1_val.to_le_bytes()), 10_f64);
        assert_eq!(f64::from_le_bytes(row.rs2_val.to_le_bytes()), 20_f64);
        assert_eq!(f64::from_le_bytes(row.rd_val.to_le_bytes()), 30_f64);
    }
}
