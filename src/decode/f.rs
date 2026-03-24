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
        0x2 => Instruction::Flw(operand),
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
        0x2 => Instruction::Fsw(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_fp_fma(insn: u32) -> Instruction {
    if fp_funct2(insn) != 0x0 {
        return Instruction::Illegal(insn);
    }

    let operand = fp_r4(insn);

    match opcode(insn) {
        0b1000011 => Instruction::FmaddS(operand),
        0b1000111 => Instruction::FmsubS(operand),
        0b1001011 => Instruction::FnmsubS(operand),
        0b1001111 => Instruction::FnmaddS(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_fp_op(insn: u32) -> Instruction {
    let rs2 = rs2(insn);
    let rm = funct3(insn);
    let operand = fp_rf(insn);
    let funct7 = fp_funct7(insn);

    match (funct7, rm) {
        (0x00, _) => Instruction::FaddS(operand),
        (0x04, _) => Instruction::FsubS(operand),
        (0x08, _) => Instruction::FmulS(operand),
        (0x0c, _) => Instruction::FdivS(operand),

        (0x2c, _) if rs2 == 0 => Instruction::FsqrtS(operand),

        (0x10, 0x0) => Instruction::FsgnjS(operand),
        (0x10, 0x1) => Instruction::FsgnjnS(operand),
        (0x10, 0x2) => Instruction::FsgnjxS(operand),

        (0x14, 0x0) => Instruction::FminS(operand),
        (0x14, 0x1) => Instruction::FmaxS(operand),

        (0x50, 0x0) => Instruction::FleS(operand),
        (0x50, 0x1) => Instruction::FltS(operand),
        (0x50, 0x2) => Instruction::FeqS(operand),

        (0x60, _) => match rs2 {
            0x00 => Instruction::FcvtWS(operand),
            0x01 => Instruction::FcvtWuS(operand),
            0x02 => Instruction::FcvtLS(operand),
            0x03 => Instruction::FcvtLuS(operand),
            _ => Instruction::Illegal(insn),
        },

        (0x68, _) => match rs2 {
            0x00 => Instruction::FcvtSW(operand),
            0x01 => Instruction::FcvtSWu(operand),
            0x02 => Instruction::FcvtSL(operand),
            0x03 => Instruction::FcvtSLu(operand),
            _ => Instruction::Illegal(insn),
        },

        (0x20, _) if rs2 == 0x01 => Instruction::FcvtSD(operand),

        (0x70, 0x0) if rs2 == 0 => Instruction::FmvXW(operand),
        (0x70, 0x1) if rs2 == 0 => Instruction::FclassS(operand),
        (0x78, 0x0) if rs2 == 0 => Instruction::FmvWX(operand),

        _ => Instruction::Illegal(insn),
    }
}
