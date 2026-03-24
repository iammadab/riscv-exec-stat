use crate::decode::insn_formats::R;
use crate::decode::util::{funct3, rd, rs1, rs2};
use crate::decode::Instruction;

pub(crate) fn decode_op(insn: u32) -> Instruction {
    let operands = R {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Mul(operands),
        0x1 => Instruction::Mulh(operands),
        0x2 => Instruction::Mulhsu(operands),
        0x3 => Instruction::Mulhu(operands),
        0x4 => Instruction::Div(operands),
        0x5 => Instruction::Divu(operands),
        0x6 => Instruction::Rem(operands),
        0x7 => Instruction::Remu(operands),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_op_32(insn: u32) -> Instruction {
    let operands = R {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Mulw(operands),
        0x4 => Instruction::Divw(operands),
        0x5 => Instruction::Divuw(operands),
        0x6 => Instruction::Remw(operands),
        0x7 => Instruction::Remuw(operands),
        _ => Instruction::Illegal(insn),
    }
}
