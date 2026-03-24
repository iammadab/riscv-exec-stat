use serde::{Deserialize, Serialize};

use crate::decode::{I, Instruction};

/// Memory operation type for RV64IMAC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MemOp {
    /// No memory operation this cycle.
    #[default]
    None,
    /// Load byte (LB/LBU).
    LoadByte { addr: u64, value: u8, signed: bool },
    /// Load halfword (LH/LHU).
    LoadHalf { addr: u64, value: u16, signed: bool },
    /// Load word (LW/LWU).
    LoadWord { addr: u64, value: u32, signed: bool },
    /// Load doubleword (LD).
    LoadDouble { addr: u64, value: u64 },
    /// Store byte (SB).
    StoreByte { addr: u64, value: u8 },
    /// Store halfword (SH).
    StoreHalf { addr: u64, value: u16 },
    /// Store word (SW).
    StoreWord { addr: u64, value: u32 },
    /// Store doubleword (SD).
    StoreDouble { addr: u64, value: u64 },
    /// Load-reserved word (LR.W).
    LoadReservedWord { addr: u64, value: u32 },
    /// Load-reserved doubleword (LR.D).
    LoadReservedDouble { addr: u64, value: u64 },
    /// Store-conditional word (SC.W).
    StoreConditionalWord {
        addr: u64,
        value: u32,
        success: bool,
    },
    /// Store-conditional doubleword (SC.D).
    StoreConditionalDouble {
        addr: u64,
        value: u64,
        success: bool,
    },
    /// Atomic memory operation word (AMO*.W).
    AtomicWord {
        addr: u64,
        read_value: u32,
        write_value: u32,
    },
    /// Atomic memory operation doubleword (AMO*.D).
    AtomicDouble {
        addr: u64,
        read_value: u64,
        write_value: u64,
    },
}

/// Flags indicating instruction class for AIR constraint selection.
///
/// @dev it is easy to go this way rather than using a separate enum for each instruction.
/// this would blot the table with is directly propostional to the proof size and proof time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstrFlags {
    // Basic RV64I flags
    /// ALU operation (ADD, SUB, AND, OR, XOR, SLT, etc.)
    pub is_alu: bool,
    /// ALU immediate operation (ADDI, ANDI, etc.)
    pub is_alu_imm: bool,
    /// Word-sized ALU operation (ADDW, SUBW, etc.)
    pub is_alu_word: bool,
    /// Word-sized ALU immediate operation (ADDIW, etc.)
    pub is_alu_imm_word: bool,
    /// Load instruction.
    pub is_load: bool,
    /// Store instruction.
    pub is_store: bool,
    /// Branch instruction.
    pub is_branch: bool,
    /// JAL instruction.
    pub is_jal: bool,
    /// JALR instruction.
    pub is_jalr: bool,
    /// LUI instruction.
    pub is_lui: bool,
    /// AUIPC instruction.
    pub is_auipc: bool,

    // M-extension flags
    /// M-extension multiply (MUL, MULH, MULHU, MULHSU).
    pub is_mul: bool,
    /// M-extension multiply word (MULW).
    pub is_mul_word: bool,
    /// M-extension divide (DIV, DIVU).
    pub is_div: bool,
    /// M-extension divide word (DIVW, DIVUW).
    pub is_div_word: bool,
    /// M-extension remainder (REM, REMU).
    pub is_rem: bool,
    /// M-extension remainder word (REMW, REMUW).
    pub is_rem_word: bool,

    // A-extension flags
    /// Load-reserved instruction (LR.W, LR.D).
    pub is_lr: bool,
    /// Store-conditional instruction (SC.W, SC.D).
    pub is_sc: bool,
    /// Atomic memory operation (AMO*).
    pub is_amo: bool,

    // Zicsr-extension flags(for Control Status Register)
    /// CSR operations (eg: CSRRW, CSRRS, CSRRC)
    pub is_csr: bool,
    /// CSR Immediate operations (eg: CSRRWI, CSRRSI, CSRRCI )
    pub is_csr_imm: bool,

    // F-extension
    /// FMA operations(eg: FmaddD, FmsubD, FnmaddD, FnmsubD)
    pub is_fma: bool,

    /// FMA word operations(eg: FmaddS, FmsubS, FnmsubS, FnmaddS )
    pub is_fma_word: bool,

    /// FPU instructions(eg: FaddD, FsubD, FminD, etc)
    pub is_fpu: bool,

    /// FPU word instructions(eg: FaddS, FsubS, FminS, etc)
    pub is_fpu_word: bool,

    /// F-extension square root instruction(eg: FsqrtD)
    pub is_fp_sqrt: bool,

    /// F-extension square root word instruction(eg: FsqrtS)
    pub is_fp_sqrt_word: bool,

    /// F-extension sign instructions(eg: FsgnjD, FsgnjnD, FsgnjxD etc)
    pub is_fp_sgn: bool,

    /// F-extension sign word instruction(eg:  FsgnjS, FsgnjnS, FsgnjxS)
    pub is_fp_sgn_word: bool,

    /// F-extension CVT interger word to single precision instructions(eg: FcvtSW, FcvtSWu)
    pub is_fp_cvt_iw_sp: bool,

    /// F-extension CVT single precision to integer word instructions(eg: FcvtWS, FcvtWuS)
    pub is_fp_cvt_sp_iw: bool,

    /// F-extension CVT double precision to single precision instruction(eg: FcvtSD)
    pub is_fp_cvt_dp_sp: bool,

    /// F-extension CVT single precision to double precision instruction(eg: FcvtDS)
    pub is_fp_cvt_sp_dp: bool,

    /// F-extension CVT double precision to integer word instruction(eg: FcvtWD, FcvtWuD)
    pub is_fp_cvt_dp_iw: bool,

    /// F-extension CVT integer word to double precision instruction(eg: FcvtDW, FcvtDWu)
    pub is_fp_cvt_iw_dp: bool,

    /// F-extension MV single precision to integer instruction(eg: FmvXW)
    pub is_fp_mv_sp_i: bool,

    /// F-extension MV integer to single precision instruction(eg: FmvWX)
    pub is_fp_mv_i_sp: bool,

    /// F-extension Classify instruction(eg: FclassD)
    pub is_fp_class: bool,

    /// F-extension Classify word instruction(eg: FclassS)
    pub is_fp_class_word: bool,

    // F-extension MV from double precision to ineteger(eg: FmvXD)
    pub is_fp_mv_dp_i: bool,

    // F-extension MV from integer to double precision(eg: FmvDX)
    pub is_fp_mv_i_dp: bool,

    // F-extension CVT from single precision to integer(eg: FcvtLS, FcvtLuS)
    pub is_fp_cvt_sp_i: bool,

    // F-extension CVT from integer to single precision(eg: FcvtSL, FcvtSLu)
    pub is_fp_cvt_i_sp: bool,

    // F-extension CVT from double precision to integer(eg: FcvtLD, FcvtLuD)
    pub is_fp_cvt_dp_i: bool,

    // F-extension CVT from integer to double precision(eg: FcvtDL, FcvtDLu)
    pub is_fp_cvt_i_dp: bool,

    /// F-extension load(eg: Flw, Fld)
    pub is_fp_load: bool,

    /// F-extension store(eg: Fsw, Fsd)
    pub is_fp_store: bool,

    // System flags
    /// ECALL instruction.
    pub is_ecall: bool,
    /// EBREAK instruction.
    pub is_ebreak: bool,
    /// FENCE instruction. (Might hav eto to take this out because, VM has no intension to run in a multi-core fashion)
    pub is_fence: bool,
}

impl InstrFlags {
    /// Create flags from an opcode.
    pub fn from_opcode(opcode: &Instruction) -> Self {
        let mut flags = Self::default();

        match opcode {
            // Basic ALU R-type
            Instruction::Add(_)
            | Instruction::Sub(_)
            | Instruction::Xor(_)
            | Instruction::Or(_)
            | Instruction::And(_)
            | Instruction::Sll(_)
            | Instruction::Srl(_)
            | Instruction::Sra(_)
            | Instruction::Slt(_)
            | Instruction::Sltu(_) => {
                flags.is_alu = true;
            }

            // ALU R-type word
            Instruction::Addw(_)
            | Instruction::Subw(_)
            | Instruction::Sllw(_)
            | Instruction::Srlw(_)
            | Instruction::Sraw(_) => {
                flags.is_alu_word = true;
            }

            // ALU I-type
            Instruction::Addi(_)
            | Instruction::Xori(_)
            | Instruction::Ori(_)
            | Instruction::Andi(_)
            | Instruction::Slli(_)
            | Instruction::Srli(_)
            | Instruction::Srai(_)
            | Instruction::Slti(_)
            | Instruction::Sltiu(_) => {
                flags.is_alu_imm = true;
            }

            // ALU I-type word
            Instruction::Addiw(_)
            | Instruction::Slliw(_)
            | Instruction::Srliw(_)
            | Instruction::Sraiw(_) => {
                flags.is_alu_imm_word = true;
            }

            // Loads
            Instruction::Lb(_)
            | Instruction::Lh(_)
            | Instruction::Lw(_)
            | Instruction::Ld(_)
            | Instruction::Lbu(_)
            | Instruction::Lhu(_)
            | Instruction::Lwu(_) => {
                flags.is_load = true;
            }

            // Stores
            Instruction::Sb(_) | Instruction::Sh(_) | Instruction::Sw(_) | Instruction::Sd(_) => {
                flags.is_store = true;
            }

            // Branches
            Instruction::Beq(_)
            | Instruction::Bne(_)
            | Instruction::Blt(_)
            | Instruction::Bge(_)
            | Instruction::Bltu(_)
            | Instruction::Bgeu(_) => {
                flags.is_branch = true;
            }

            // Jumps
            Instruction::Jal(_) => flags.is_jal = true,
            Instruction::Jalr(_) => flags.is_jalr = true,

            // Upper immediates
            Instruction::Lui(_) => flags.is_lui = true,
            Instruction::Auipc(_) => flags.is_auipc = true,

            // M-extension multiply
            Instruction::Mul(_)
            | Instruction::Mulh(_)
            | Instruction::Mulhsu(_)
            | Instruction::Mulhu(_) => {
                flags.is_mul = true;
            }
            Instruction::Mulw(_) => flags.is_mul_word = true,

            // M-extension divide
            Instruction::Div(_) | Instruction::Divu(_) => flags.is_div = true,
            Instruction::Divw(_) | Instruction::Divuw(_) => flags.is_div_word = true,

            // M-extension remainder
            Instruction::Rem(_) | Instruction::Remu(_) => flags.is_rem = true,
            Instruction::Remw(_) | Instruction::Remuw(_) => flags.is_rem_word = true,

            // A-extension load-reserved
            Instruction::LrW(_) | Instruction::LrD(_) => flags.is_lr = true,

            // A-extension store-conditional
            Instruction::ScW(_) | Instruction::ScD(_) => flags.is_sc = true,

            // A-extension atomic operations
            Instruction::AmoSwapW(_)
            | Instruction::AmoAddW(_)
            | Instruction::AmoXorW(_)
            | Instruction::AmoAndW(_)
            | Instruction::AmoOrW(_)
            | Instruction::AmoMinW(_)
            | Instruction::AmoMaxW(_)
            | Instruction::AmoMinuW(_)
            | Instruction::AmoMaxuW(_)
            | Instruction::AmoSwapD(_)
            | Instruction::AmoAddD(_)
            | Instruction::AmoXorD(_)
            | Instruction::AmoAndD(_)
            | Instruction::AmoOrD(_)
            | Instruction::AmoMinD(_)
            | Instruction::AmoMaxD(_)
            | Instruction::AmoMinuD(_)
            | Instruction::AmoMaxuD(_) => {
                flags.is_amo = true;
            }

            // CSR operations
            Instruction::Csrrw(_) | Instruction::Csrrs(_) | Instruction::Csrrc(_) => {
                flags.is_csr = true;
            }

            // CSR Immediate operations
            Instruction::Csrrwi(_) | Instruction::Csrrsi(_) | Instruction::Csrrci(_) => {
                flags.is_csr_imm = true;
            }

            // F-extension
            // FMA instructions
            Instruction::FmaddD(_)
            | Instruction::FmsubD(_)
            | Instruction::FnmaddD(_)
            | Instruction::FnmsubD(_) => {
                flags.is_fma = true;
            }

            // FMA word instructions
            Instruction::FmaddS(_)
            | Instruction::FmsubS(_)
            | Instruction::FnmaddS(_)
            | Instruction::FnmsubS(_) => {
                flags.is_fma_word = true;
            }

            // FP ALU word instructions
            Instruction::FaddS(_)
            | Instruction::FsubS(_)
            | Instruction::FmulS(_)
            | Instruction::FdivS(_)
            | Instruction::FminS(_)
            | Instruction::FmaxS(_)
            | Instruction::FeqS(_)
            | Instruction::FltS(_)
            | Instruction::FleS(_) => {
                flags.is_fpu_word = true;
            }

            // FP ALU instruction
            Instruction::FaddD(_)
            | Instruction::FsubD(_)
            | Instruction::FmulD(_)
            | Instruction::FdivD(_)
            | Instruction::FminD(_)
            | Instruction::FmaxD(_)
            | Instruction::FeqD(_)
            | Instruction::FltD(_)
            | Instruction::FleD(_) => {
                flags.is_fpu = true;
            }

            // FP sqrt word instruction
            Instruction::FsqrtS(_) => {
                flags.is_fp_sqrt_word = true;
            }

            // FP sqrt instruction
            Instruction::FsqrtD(_) => {
                flags.is_fp_sqrt = true;
            }

            // FP sign word instruction
            Instruction::FsgnjS(_) | Instruction::FsgnjnS(_) | Instruction::FsgnjxS(_) => {
                flags.is_fp_sgn_word = true;
            }

            // FP sign instruction
            Instruction::FsgnjD(_) | Instruction::FsgnjnD(_) | Instruction::FsgnjxD(_) => {
                flags.is_fp_sgn = true;
            }

            // FP cvt from single precision to integer word instruction
            Instruction::FcvtWS(_) | Instruction::FcvtWuS(_) => {
                flags.is_fp_cvt_sp_iw = true;
            }

            // FP cvt from integer word to single precision instruction
            Instruction::FcvtSW(_) | Instruction::FcvtSWu(_) => {
                flags.is_fp_cvt_iw_sp = true;
            }

            // FP cvt double precision to single precision instruction
            Instruction::FcvtSD(_) => {
                flags.is_fp_cvt_dp_sp = true;
            }

            // FP cvt single precision to double precision instruction
            Instruction::FcvtDS(_) => {
                flags.is_fp_cvt_sp_dp = true;
            }

            // FP cvt from double precision to integer word instruction
            Instruction::FcvtWD(_) | Instruction::FcvtWuD(_) => {
                flags.is_fp_cvt_dp_iw = true;
            }

            // FP cvt from integer word to double precision instruction
            Instruction::FcvtDW(_) | Instruction::FcvtDWu(_) => {
                flags.is_fp_cvt_iw_dp = true;
            }

            // FP mv from single precision to integer instruction
            Instruction::FmvXW(_) => {
                flags.is_fp_mv_sp_i = true;
            }

            // FP mv from integer to single precision instruction
            Instruction::FmvWX(_) => {
                flags.is_fp_mv_i_sp = true;
            }

            // FP mv from double precision to ineteger
            Instruction::FmvXD(_) => {
                flags.is_fp_mv_dp_i = true;
            }

            // FP mv from integer to double precision
            Instruction::FmvDX(_) => {
                flags.is_fp_mv_i_dp = true;
            }

            // FP classify word
            Instruction::FclassS(_) => {
                flags.is_fp_class_word = true;
            }

            // FP classify
            Instruction::FclassD(_) => {
                flags.is_fp_class = true;
            }

            // FP load
            Instruction::Flw(_) | Instruction::Fld(_) => {
                flags.is_fp_load = true;
            }

            // FP store
            Instruction::Fsw(_) | Instruction::Fsd(_) => {
                flags.is_fp_store = true;
            }

            // FP cvt from single precision to integer
            Instruction::FcvtLS(_) | Instruction::FcvtLuS(_) => {
                flags.is_fp_cvt_sp_i = true;
            }

            // FP cvt from integer to single precision
            Instruction::FcvtSL(_) | Instruction::FcvtSLu(_) => {
                flags.is_fp_cvt_i_sp = true;
            }

            // FP cvt from double precision to integer
            Instruction::FcvtLD(_) | Instruction::FcvtLuD(_) => {
                flags.is_fp_cvt_dp_i = true;
            }

            // FP cvt from integer to double precision
            Instruction::FcvtDL(_) | Instruction::FcvtDLu(_) => {
                flags.is_fp_cvt_i_dp = true;
            }

            // System
            Instruction::Ecall => flags.is_ecall = true,
            Instruction::Ebreak => flags.is_ebreak = true,
            Instruction::Eother => {}

            // Remove when implemented
            _ => {
                println!("Instruction not implemented: {:?}", opcode);
                unimplemented!()
            }
        }

        flags
    }
}

/// A single row of the execution trace.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct TraceRow {
    /// Clock cycle / step number.
    pub clk: u64,
    /// Program counter before this instruction.
    pub pc: u64,
    /// Next program counter (after this instruction).
    pub next_pc: u64,
    /// Raw 32-bit instruction encoding.
    pub raw_instr: u32,
    /// Decoded opcode.
    pub opcode: Instruction,
    /// Instruction classification flags.
    pub flags: InstrFlags,
    /// Integer Register values BEFORE this instruction (x0..x31).
    pub regs: [u64; 32],
    /// Floating point Register values before this instruction
    pub f_regs: [u64; 32],
    /// Source register 1 index.
    pub rs1: u8,
    /// Source register 2 index.
    pub rs2: u8,
    /// Source register 3 index.
    pub rs3: u8,
    /// Destination register index (0 if no write).
    pub rd: u8,
    /// Immediate value.
    pub imm: u64,
    /// Rounding mode
    pub rm: u8,
    /// Value of rs1.
    pub rs1_val: u64,
    /// Value of rs2.
    pub rs2_val: u64,
    /// Value of rs3.
    pub rs3_val: u64,
    /// Value written to rd (if any).
    pub rd_val: u64,
    /// Memory operation (if any).
    pub mem_op: MemOp,
    /// For M-extension: low 64 bits of 128-bit intermediate (for MUL verification).
    pub mul_lo: u64,
    /// For M-extension: high 64 bits of 128-bit intermediate.
    pub mul_hi: u64,
    /// For A-extension: reservation set address (for LR/SC verification).
    pub reservation_addr: u64,
    /// For F-extension: control status register
    pub csr_reg: u32,
    /// Whether the instruction caused a halt.
    pub halted: bool,
}

impl TraceRow {
    /// Create a new trace row with default values.
    pub fn new(clk: u64, pc: u64, regs: [u64; 32], f_regs: [u64; 32]) -> Self {
        Self {
            clk,
            pc,
            next_pc: pc + 4,
            raw_instr: 0x00000013, // NOP (addi x0, x0, 0)
            opcode: Instruction::Addi(I {
                rd: 0,
                rs1: 0,
                imm: 0,
            }),
            flags: InstrFlags::default(),
            regs,
            f_regs,
            rs1: 0,
            rs2: 0,
            rs3: 0,
            rd: 0,
            imm: 0,
            rm: 0,
            rs1_val: 0,
            rs2_val: 0,
            rs3_val: 0,
            rd_val: 0,
            mem_op: MemOp::None,
            mul_lo: 0,
            mul_hi: 0,
            reservation_addr: 0,
            csr_reg: 0,
            halted: false,
        }
    }

    /// Create a trace row from an instruction.
    pub(crate) fn from_instruction(
        clk: u64,
        pc: u64,
        raw_instr: u32,
        instr: &Instruction,
        regs: [u64; 32],
        f_regs: [u64; 32],
    ) -> Self {
        let flags = InstrFlags::from_opcode(&instr);
        let rs1 = instr.rs1();
        let rs2 = instr.rs2();
        let rs3 = instr.rs3();
        let rd = instr.rd();
        let imm = instr.imm();
        let rm = instr.rm();

        let rs1_val = if rs1 == 0 { 0 } else { regs[rs1 as usize] };
        let rs2_val = if rs2 == 0 { 0 } else { regs[rs2 as usize] };
        let rs3_val = if rs3 == 0 { 0 } else { regs[rs3 as usize] };

        Self {
            clk,
            pc,
            next_pc: pc + 4,
            raw_instr,
            opcode: *instr,
            flags,
            regs,
            f_regs,
            rs1: rs1,
            rs2: rs2,
            rs3: rs3,
            rd: rd,
            imm: imm,
            rm,
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
        }
    }

    /// Set the destination register value.
    pub fn with_rd_val(mut self, val: u64) -> Self {
        self.rd_val = val;
        self
    }

    /// Set the next PC.
    pub fn with_next_pc(mut self, next_pc: u64) -> Self {
        self.next_pc = next_pc;
        self
    }

    /// Set the memory operation.
    pub fn with_mem_op(mut self, mem_op: MemOp) -> Self {
        self.mem_op = mem_op;
        self
    }

    /// Set multiplication intermediate values.
    pub fn with_mul_intermediate(mut self, lo: u64, hi: u64) -> Self {
        self.mul_lo = lo;
        self.mul_hi = hi;
        self
    }

    /// Set reservation address for LR/SC.
    pub fn with_reservation(mut self, addr: u64) -> Self {
        self.reservation_addr = addr;
        self
    }

    /// Mark as halted.
    pub fn with_halt(mut self) -> Self {
        self.halted = true;
        self
    }
}

/// Complete execution trace.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ExecutionTrace {
    /// All trace rows.
    pub rows: Vec<TraceRow>,
    /// Initial integer register state.
    pub initial_x_regs: [u64; 32],
    /// Initial floating point register state
    pub initial_f_regs: [u64; 32],
    /// Initial PC.
    pub initial_pc: u64,
    /// Final integer register state.
    pub final_x_regs: [u64; 32],
    /// Final floating point register state
    pub final_f_regs: [u64; 32],
    /// Final PC.
    pub final_pc: u64,
    /// Total cycles executed.
    pub total_cycles: u64,
    /// Exit code (from a0 register on halt).
    pub exit_code: u64,
    /// Halt reason (if any).
    pub halt_reason: Option<String>,
}

impl ExecutionTrace {
    /// Create a new empty trace with initial state.
    pub fn new(initial_pc: u64, initial_x_regs: [u64; 32], initial_f_regs: [u64; 32]) -> Self {
        Self {
            rows: Vec::new(),
            initial_x_regs,
            initial_f_regs,
            initial_pc,
            final_x_regs: initial_x_regs,
            final_f_regs: initial_f_regs,
            final_pc: initial_pc,
            total_cycles: 0,
            exit_code: 0,
            halt_reason: None,
        }
    }

    /// Add a row to the trace.
    pub fn push(&mut self, row: TraceRow) {
        self.total_cycles = row.clk + 1;
        self.final_pc = row.next_pc;

        // Update final integer register state
        if row.opcode.is_integer_insn() {
            if row.rd != 0 {
                self.final_x_regs[row.rd as usize] = row.rd_val;
            }
        }

        // Update final floating point register state
        // First floating point register is not a zero register
        if row.opcode.is_fp_insn() {
            self.final_f_regs[row.rd as usize] = row.rd_val;
        }

        if row.halted {
            self.exit_code = self.final_x_regs[10]; // a0 register
        }

        self.rows.push(row);
    }

    /// Get the number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get a row by index.
    pub fn get(&self, index: usize) -> Option<&TraceRow> {
        self.rows.get(index)
    }

    /// Get a mutable row by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TraceRow> {
        self.rows.get_mut(index)
    }

    /// Iterate over all rows.
    pub fn iter(&self) -> impl Iterator<Item = &TraceRow> {
        self.rows.iter()
    }

    /// Set the halt reason.
    pub fn set_halt_reason(&mut self, reason: impl Into<String>) {
        self.halt_reason = Some(reason.into());
    }

    /// Get memory operations from the trace.
    pub fn memory_ops(&self) -> impl Iterator<Item = (u64, &MemOp)> {
        self.rows
            .iter()
            .filter(|row| row.mem_op != MemOp::None)
            .map(|row| (row.clk, &row.mem_op))
    }

    /// Get all load operations.
    pub fn loads(&self) -> Vec<(u64, u64, u64)> {
        self.rows
            .iter()
            .filter_map(|row| match &row.mem_op {
                MemOp::LoadByte { addr, value, .. } => Some((row.clk, *addr, *value as u64)),
                MemOp::LoadHalf { addr, value, .. } => Some((row.clk, *addr, *value as u64)),
                MemOp::LoadWord { addr, value, .. } => Some((row.clk, *addr, *value as u64)),
                MemOp::LoadDouble { addr, value } => Some((row.clk, *addr, *value)),
                MemOp::LoadReservedWord { addr, value } => Some((row.clk, *addr, *value as u64)),
                MemOp::LoadReservedDouble { addr, value } => Some((row.clk, *addr, *value)),
                MemOp::AtomicWord {
                    addr, read_value, ..
                } => Some((row.clk, *addr, *read_value as u64)),
                MemOp::AtomicDouble {
                    addr, read_value, ..
                } => Some((row.clk, *addr, *read_value)),
                _ => None,
            })
            .collect()
    }

    /// Get all store operations.
    pub fn stores(&self) -> Vec<(u64, u64, u64)> {
        self.rows
            .iter()
            .filter_map(|row| match &row.mem_op {
                MemOp::StoreByte { addr, value } => Some((row.clk, *addr, *value as u64)),
                MemOp::StoreHalf { addr, value } => Some((row.clk, *addr, *value as u64)),
                MemOp::StoreWord { addr, value } => Some((row.clk, *addr, *value as u64)),
                MemOp::StoreDouble { addr, value } => Some((row.clk, *addr, *value)),
                MemOp::StoreConditionalWord {
                    addr,
                    value,
                    success,
                } if *success => Some((row.clk, *addr, *value as u64)),
                MemOp::StoreConditionalDouble {
                    addr,
                    value,
                    success,
                } if *success => Some((row.clk, *addr, *value)),
                MemOp::AtomicWord {
                    addr, write_value, ..
                } => Some((row.clk, *addr, *write_value as u64)),
                MemOp::AtomicDouble {
                    addr, write_value, ..
                } => Some((row.clk, *addr, *write_value)),
                _ => None,
            })
            .collect()
    }

    /// Get instruction count by category.
    pub fn instruction_stats(&self) -> InstructionStats {
        let mut stats = InstructionStats::default();

        for row in &self.rows {
            let f = &row.flags;
            if f.is_alu || f.is_alu_word {
                stats.alu += 1;
            }
            if f.is_alu_imm || f.is_alu_imm_word {
                stats.alu_imm += 1;
            }
            if f.is_load {
                stats.load += 1;
            }
            if f.is_store {
                stats.store += 1;
            }
            if f.is_branch {
                stats.branch += 1;
            }
            if f.is_jal || f.is_jalr {
                stats.jump += 1;
            }
            if f.is_lui || f.is_auipc {
                stats.upper_imm += 1;
            }
            if f.is_mul || f.is_mul_word {
                stats.mul += 1;
            }
            if f.is_div || f.is_div_word {
                stats.div += 1;
            }
            if f.is_rem || f.is_rem_word {
                stats.rem += 1;
            }
            if f.is_lr || f.is_sc || f.is_amo {
                stats.atomic += 1;
            }
            if f.is_csr || f.is_csr_imm {
                stats.csr += 1;
            }
            if f.is_fpu || f.is_fpu_word {
                stats.fpu += 1;
            }
            if f.is_fma || f.is_fma_word {
                stats.fma += 1;
            }
            if f.is_fp_sqrt || f.is_fp_sqrt_word {
                stats.sqrt += 1;
            }
            if f.is_fp_sgn || f.is_fp_sgn_word {
                stats.sgn += 1;
            }
            if f.is_fp_cvt_iw_sp
                || f.is_fp_cvt_sp_iw
                || f.is_fp_cvt_dp_sp
                || f.is_fp_cvt_sp_dp
                || f.is_fp_cvt_dp_iw
                || f.is_fp_cvt_iw_dp
                || f.is_fp_cvt_sp_i
                || f.is_fp_cvt_i_sp
                || f.is_fp_cvt_dp_i
                || f.is_fp_cvt_i_dp
            {
                stats.cvt += 1;
            }
            if f.is_fp_mv_sp_i || f.is_fp_mv_i_sp || f.is_fp_mv_dp_i || f.is_fp_mv_i_dp {
                stats.mv += 1;
            }
            if f.is_fp_class || f.is_fp_class_word {
                stats.class += 1;
            }
            if f.is_fp_load {
                stats.fp_load += 1;
            }
            if f.is_fp_store {
                stats.fp_store += 1;
            }
            if f.is_ecall || f.is_ebreak {
                stats.system += 1;
            }
        }

        stats
    }
}

/// Statistics about instruction types in a trace.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InstructionStats {
    pub alu: u64,
    pub alu_imm: u64,
    pub load: u64,
    pub store: u64,
    pub branch: u64,
    pub jump: u64,
    pub upper_imm: u64,
    pub mul: u64,
    pub div: u64,
    pub rem: u64,
    pub atomic: u64,
    pub csr: u64,
    pub fpu: u64,
    pub fma: u64,
    pub sqrt: u64,
    pub sgn: u64,
    pub cvt: u64,
    pub mv: u64,
    pub class: u64,
    pub fp_load: u64,
    pub fp_store: u64,
    pub system: u64,
}

impl InstructionStats {
    /// Get total instruction count.
    pub fn total(&self) -> u64 {
        self.alu
            + self.alu_imm
            + self.load
            + self.store
            + self.branch
            + self.jump
            + self.upper_imm
            + self.mul
            + self.div
            + self.rem
            + self.atomic
            + self.system
            + self.csr
            + self.fpu
            + self.fma
            + self.sqrt
            + self.sgn
            + self.cvt
            + self.mv
            + self.class
            + self.fp_load
            + self.fp_store
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::R;

    use super::*;

    #[test]
    fn test_trace_row_creation() {
        let regs = [0u64; 32];
        let f_reg = [0u64; 32];
        let row = TraceRow::new(0, 0x1000, regs, f_reg);
        assert_eq!(row.clk, 0);
        assert_eq!(row.pc, 0x1000);
        assert_eq!(row.next_pc, 0x1004);
    }

    #[test]
    fn test_execution_trace() {
        let mut trace = ExecutionTrace::new(0x1000, [0u64; 32], [0u64; 32]);
        assert!(trace.is_empty());

        let row = TraceRow::new(0, 0x1000, [0u64; 32], [0u64; 32]);
        trace.push(row);

        assert_eq!(trace.len(), 1);
        assert_eq!(trace.total_cycles, 1);
    }

    #[test]
    fn test_instr_flags_alu() {
        let flags = InstrFlags::from_opcode(&Instruction::Add(R {
            rd: 0,
            rs1: 0,
            rs2: 0,
        }));
        assert!(flags.is_alu);
        assert!(!flags.is_load);
        assert!(!flags.is_store);
    }

    #[test]
    fn test_instr_flags_load() {
        let flags = InstrFlags::from_opcode(&Instruction::Ld(I {
            rd: 0,
            rs1: 0,
            imm: 0,
        }));
        assert!(flags.is_load);
        assert!(!flags.is_alu);
    }

    #[test]
    fn test_instr_flags_atomic() {
        let flags = InstrFlags::from_opcode(&Instruction::AmoAddD(R {
            rd: 0,
            rs1: 0,
            rs2: 0,
        }));
        assert!(flags.is_amo);
        assert!(!flags.is_lr);
        assert!(!flags.is_sc);
    }

    #[test]
    fn test_mem_op_default() {
        let mem_op = MemOp::default();
        assert_eq!(mem_op, MemOp::None);
    }
}
