use crate::util::{mask16, mask32};

/// Extracts the opcode value from a 32 bit insn
#[inline]
pub(crate) fn opcode(insn: u32) -> u8 {
    (insn & mask32(7)) as u8
}

#[inline]
pub(crate) fn funct3(insn: u32) -> u8 {
    ((insn >> 12) & mask32(3)) as u8
}

#[inline]
pub(crate) fn funct6(insn: u32) -> u8 {
    ((insn >> 26) & mask32(6)) as u8
}

#[inline]
pub(crate) fn funct7(insn: u32) -> u8 {
    ((insn >> 25) & mask32(7)) as u8
}

#[inline]
pub(crate) fn rd(insn: u32) -> u8 {
    ((insn >> 7) & mask32(5)) as u8
}

#[inline]
pub(crate) fn rs1(insn: u32) -> u8 {
    ((insn >> 15) & mask32(5)) as u8
}

#[inline]
pub(crate) fn rs2(insn: u32) -> u8 {
    ((insn >> 20) & mask32(5)) as u8
}

#[inline]
pub(crate) fn rs3(insn: u32) -> u8 {
    ((insn >> 27) & mask32(5)) as u8
}

// Compressed Instruction Utils

#[inline]
pub(crate) fn quadrant(insn: u16) -> u8 {
    (insn & mask16(2)) as u8
}

pub(crate) fn c_funct3(insn: u16) -> u8 {
    ((insn >> 13) & mask16(3)) as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcode() {
        assert_eq!(opcode(0x03a5d593), 0b0010011);
    }
}
