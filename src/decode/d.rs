use crate::decode::fp_util::{fp_funct2, fp_funct7, fp_r4, fp_rf};
use crate::decode::imm::{imm_i, imm_s};
use crate::decode::insn_formats::{I, S};
use crate::decode::util::{funct3, opcode, rd, rs1, rs2};
use crate::decode::Instruction;

pub(crate) fn decode_fp_load(insn: u32) -> Instruction {
    let operand = I {
        rd: rd(insn),
        rs1: rs1(insn),
        imm: imm_i(insn),
    };

    match funct3(insn) {
        0x3 => Instruction::Fld(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_fp_store(insn: u32) -> Instruction {
    let operand = S {
        rs1: rs1(insn),
        rs2: rs2(insn),
        imm: imm_s(insn),
    };

    match funct3(insn) {
        0x3 => Instruction::Fsd(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_fp_fma(insn: u32) -> Instruction {
    if fp_funct2(insn) != 0x1 {
        return Instruction::Illegal(insn);
    }

    let operand = fp_r4(insn);

    match opcode(insn) {
        0b1000011 => Instruction::FmaddD(operand),
        0b1000111 => Instruction::FmsubD(operand),
        0b1001011 => Instruction::FnmsubD(operand),
        0b1001111 => Instruction::FnmaddD(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_fp_op(insn: u32) -> Instruction {
    let rs2 = rs2(insn);
    let rm = funct3(insn);
    let operand = fp_rf(insn);
    let funct7 = fp_funct7(insn);

    match (funct7, rm) {
        (0x01, _) => Instruction::FaddD(operand),
        (0x05, _) => Instruction::FsubD(operand),
        (0x09, _) => Instruction::FmulD(operand),
        (0x0d, _) => Instruction::FdivD(operand),

        (0x2d, _) if rs2 == 0 => Instruction::FsqrtD(operand),

        (0x11, 0x0) => Instruction::FsgnjD(operand),
        (0x11, 0x1) => Instruction::FsgnjnD(operand),
        (0x11, 0x2) => Instruction::FsgnjxD(operand),

        (0x15, 0x0) => Instruction::FminD(operand),
        (0x15, 0x1) => Instruction::FmaxD(operand),

        (0x51, 0x0) => Instruction::FleD(operand),
        (0x51, 0x1) => Instruction::FltD(operand),
        (0x51, 0x2) => Instruction::FeqD(operand),

        (0x61, _) => match rs2 {
            0x00 => Instruction::FcvtWD(operand),
            0x01 => Instruction::FcvtWuD(operand),
            0x02 => Instruction::FcvtLD(operand),
            0x03 => Instruction::FcvtLuD(operand),
            _ => Instruction::Illegal(insn),
        },

        (0x69, _) => match rs2 {
            0x00 => Instruction::FcvtDW(operand),
            0x01 => Instruction::FcvtDWu(operand),
            0x02 => Instruction::FcvtDL(operand),
            0x03 => Instruction::FcvtDLu(operand),
            _ => Instruction::Illegal(insn),
        },

        (0x21, _) if rs2 == 0x00 => Instruction::FcvtDS(operand),

        (0x71, 0x0) if rs2 == 0 => Instruction::FmvXD(operand),
        (0x71, 0x1) if rs2 == 0 => Instruction::FclassD(operand),
        (0x79, 0x0) if rs2 == 0 => Instruction::FmvDX(operand),

        _ => Instruction::Illegal(insn),
    }
}
