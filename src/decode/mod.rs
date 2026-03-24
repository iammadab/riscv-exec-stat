mod a;
pub(crate) mod compressed;
mod i;
mod imm;
mod insn;
mod insn_formats;
mod m;
mod util;
mod zicsr;

pub(crate) use insn::Instruction;
pub(crate) use insn_formats::{Sh, B, I, J, R, R4, RF, S, U};
use util::{funct3, opcode};

pub(crate) fn decode(insn: u32) -> Instruction {
    match opcode(insn) {
        0b0110011 => i::decode_op(insn),
        0b0010011 => i::decode_op_imm(insn),
        0b0111011 => i::decode_op_32(insn),
        0b0011011 => i::decode_op_imm_32(insn),

        0b0000011 => i::decode_load(insn),
        0b0100011 => i::decode_store(insn),
        0b1100011 => i::decode_branch(insn),
        0b1101111 => i::decode_jal(insn),
        0b1100111 => i::decode_jalr(insn),
        0b0110111 => i::decode_lui(insn),
        0b0010111 => i::decode_auipc(insn),
        0b1110011 => zicsr::decode_system(insn),
        0b0001111 => i::decode_fence(insn),

        // Atomics
        0b0101111 => a::decode_atomics(insn),

        0b0000111 | 0b0100111 | 0b1000011 | 0b1000111 | 0b1001011 | 0b1001111 | 0b1010011 => {
            Instruction::Illegal(insn)
        }

        _ => Instruction::Illegal(insn),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_ts() {
        assert_eq!(
            decode(0x03a5d593),
            Instruction::Srli(Sh {
                rd: 11,
                rs1: 11,
                shamt: 58
            })
        );
    }
}
