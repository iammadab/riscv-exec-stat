use crate::decode::insn_formats::{R4, RF};
use crate::decode::util::rs3;
use crate::decode::util::{funct3, funct7, rd, rs1, rs2};

pub(crate) fn fp_r4(insn: u32) -> R4 {
    R4 {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
        rs3: rs3(insn),
        rm: funct3(insn),
    }
}

pub(crate) fn fp_rf(insn: u32) -> RF {
    RF {
        rd: rd(insn),
        rs1: rs1(insn),
        rs2: rs2(insn),
        rm: funct3(insn),
    }
}

pub(crate) fn fp_funct7(insn: u32) -> u8 {
    funct7(insn)
}

pub(crate) fn fp_funct2(insn: u32) -> u8 {
    (funct7(insn) & 0b11) as u8
}
