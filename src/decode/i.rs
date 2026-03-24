use crate::decode::imm::{imm_b, imm_i, imm_j, imm_s, imm_u, shamt5, shamt6};
use crate::decode::insn_formats::{Sh, B, I, J, R, S, U};
use crate::decode::util::{funct3, funct6, funct7, rd, rs1, rs2};
use crate::decode::Instruction;

pub(crate) fn decode_op(insn: u32) -> Instruction {
    if funct7(insn) == 0x01 {
        return crate::decode::m::decode_op(insn);
    }

    if funct7(insn) == 0x01 {
        return Instruction::Illegal(insn);
    }

    let insn_operands = R {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
    };

    match (funct3(insn), funct7(insn)) {
        (0x0, 0x00) => Instruction::Add(insn_operands),
        (0x0, 0x20) => Instruction::Sub(insn_operands),
        (0x4, 0x00) => Instruction::Xor(insn_operands),
        (0x6, 0x00) => Instruction::Or(insn_operands),
        (0x7, 0x00) => Instruction::And(insn_operands),
        (0x1, 0x00) => Instruction::Sll(insn_operands),
        (0x5, 0x00) => Instruction::Srl(insn_operands),
        (0x5, 0x20) => Instruction::Sra(insn_operands),
        (0x2, 0x00) => Instruction::Slt(insn_operands),
        (0x3, 0x00) => Instruction::Sltu(insn_operands),

        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_op_imm(insn: u32) -> Instruction {
    let rd = rd(insn);
    let rs1 = rs1(insn);
    let imm = imm_i(insn);

    if funct3(insn) == 0x0 && rd == 0 && rs1 == 0 && imm == 0 {
        return Instruction::Nop;
    }

    let i_operands = I { rd, rs1, imm };
    let s_operands = Sh {
        rd,
        rs1,
        shamt: shamt6(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Addi(i_operands),
        0x4 => Instruction::Xori(i_operands),
        0x6 => Instruction::Ori(i_operands),
        0x7 => Instruction::Andi(i_operands),
        0x2 => Instruction::Slti(i_operands),
        0x3 => Instruction::Sltiu(i_operands),
        0x1 | 0x5 => match (funct3(insn), funct7(insn), funct6(insn)) {
            (0x5, _, 0x00) => Instruction::Srli(s_operands),
            (0x1, _, 0x00) => Instruction::Slli(s_operands),
            (0x5, _, 0x10) => Instruction::Srai(s_operands),
            _ => Instruction::Illegal(insn),
        },
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_op_32(insn: u32) -> Instruction {
    if funct7(insn) == 0x01 {
        return crate::decode::m::decode_op_32(insn);
    }

    if funct7(insn) == 0x01 {
        return Instruction::Illegal(insn);
    }

    let operands = R {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
    };

    match (funct3(insn), funct7(insn)) {
        (0x0, 0x00) => Instruction::Addw(operands),
        (0x0, 0x20) => Instruction::Subw(operands),
        (0x1, 0x00) => Instruction::Sllw(operands),
        (0x5, 0x00) => Instruction::Srlw(operands),
        (0x5, 0x20) => Instruction::Sraw(operands),

        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_op_imm_32(insn: u32) -> Instruction {
    let rd = rd(insn);
    let rs1 = rs1(insn);
    let imm = imm_i(insn);

    let i_operands = I { rd, rs1, imm };
    let s_operands = Sh {
        rd,
        rs1,
        shamt: shamt5(insn),
    };

    match (funct3(insn), funct7(insn)) {
        (0x0, _) => Instruction::Addiw(i_operands),
        (0x1, 0x00) => Instruction::Slliw(s_operands),
        (0x5, 0x00) => Instruction::Srliw(s_operands),
        (0x5, 0x20) => Instruction::Sraiw(s_operands),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_load(insn: u32) -> Instruction {
    let operand = I {
        rd: rd(insn),
        rs1: rs1(insn),
        imm: imm_i(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Lb(operand),
        0x1 => Instruction::Lh(operand),
        0x2 => Instruction::Lw(operand),
        0x3 => Instruction::Ld(operand),
        0x4 => Instruction::Lbu(operand),
        0x5 => Instruction::Lhu(operand),
        0x6 => Instruction::Lwu(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_store(insn: u32) -> Instruction {
    let operand = S {
        rs1: rs1(insn),
        rs2: rs2(insn),
        imm: imm_s(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Sb(operand),
        0x1 => Instruction::Sh(operand),
        0x2 => Instruction::Sw(operand),
        0x3 => Instruction::Sd(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_branch(insn: u32) -> Instruction {
    let operand = B {
        rs1: rs1(insn),
        rs2: rs2(insn),
        imm: imm_b(insn),
    };

    match funct3(insn) {
        0x0 => Instruction::Beq(operand),
        0x1 => Instruction::Bne(operand),
        0x4 => Instruction::Blt(operand),
        0x5 => Instruction::Bge(operand),
        0x6 => Instruction::Bltu(operand),
        0x7 => Instruction::Bgeu(operand),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_jal(insn: u32) -> Instruction {
    Instruction::Jal(J {
        rd: rd(insn),
        imm: imm_j(insn),
    })
}

pub(crate) fn decode_jalr(insn: u32) -> Instruction {
    match funct3(insn) {
        0x0 => Instruction::Jalr(I {
            rd: rd(insn),
            rs1: rs1(insn),
            imm: imm_i(insn),
        }),
        _ => Instruction::Illegal(insn),
    }
}

pub(crate) fn decode_lui(insn: u32) -> Instruction {
    Instruction::Lui(U {
        rd: rd(insn),
        imm: imm_u(insn),
    })
}

pub(crate) fn decode_auipc(insn: u32) -> Instruction {
    Instruction::Auipc(U {
        rd: rd(insn),
        imm: imm_u(insn),
    })
}

pub(crate) fn decode_fence(_insn: u32) -> Instruction {
    // system-reminder: was Fence
    Instruction::Nop
}
