use crate::decode::insn_formats::R;
use crate::decode::util::{funct3, funct7, rd, rs1, rs2};
use crate::decode::Instruction;

pub(crate) fn decode_atomics(insn: u32) -> Instruction {
    let funct5 = funct7(insn) >> 2;

    let rd = rd(insn);
    let rs1 = rs1(insn);
    let rs2 = rs2(insn);

    let operand = R { rd, rs1, rs2 };

    match (funct3(insn), funct5) {
        (0x2, 0x02) if rs2 == 0 => Instruction::LrW(operand),
        (0x2, 0x03) => Instruction::ScW(operand),
        (0x2, 0x01) => Instruction::AmoSwapW(operand),
        (0x2, 0x00) => Instruction::AmoAddW(operand),
        (0x2, 0x04) => Instruction::AmoXorW(operand),
        (0x2, 0x0c) => Instruction::AmoAndW(operand),
        (0x2, 0x08) => Instruction::AmoOrW(operand),
        (0x2, 0x10) => Instruction::AmoMinW(operand),
        (0x2, 0x14) => Instruction::AmoMaxW(operand),
        (0x2, 0x18) => Instruction::AmoMinuW(operand),
        (0x2, 0x1c) => Instruction::AmoMaxuW(operand),

        (0x3, 0x02) if rs2 == 0 => Instruction::LrD(operand),
        (0x3, 0x03) => Instruction::ScD(operand),
        (0x3, 0x01) => Instruction::AmoSwapD(operand),
        (0x3, 0x00) => Instruction::AmoAddD(operand),
        (0x3, 0x04) => Instruction::AmoXorD(operand),
        (0x3, 0x0C) => Instruction::AmoAndD(operand),
        (0x3, 0x08) => Instruction::AmoOrD(operand),
        (0x3, 0x10) => Instruction::AmoMinD(operand),
        (0x3, 0x14) => Instruction::AmoMaxD(operand),
        (0x3, 0x18) => Instruction::AmoMinuD(operand),
        (0x3, 0x1c) => Instruction::AmoMaxuD(operand),

        _ => Instruction::Illegal(insn),
    }
}
