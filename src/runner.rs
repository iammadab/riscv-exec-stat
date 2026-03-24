use crate::decode::compressed::decode_compressed;
use crate::decode::Instruction;
use crate::{decode, HostIO, VM};

#[derive(Debug, Default, Clone, Copy)]
pub struct ExecStats {
    pub insns: u64,
    pub compressed_insns: u64,
}

pub struct Runner {
    io: HostIO,
    stats: ExecStats,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            io: HostIO::new(),
            stats: ExecStats::default(),
        }
    }

    pub fn set_input_stream(&mut self, input: Vec<u8>) {
        self.io.set_input_stream(input);
    }

    pub fn stats(&self) -> ExecStats {
        self.stats
    }

    pub fn cycles(&self) -> u64 {
        self.stats.insns
    }

    pub fn run(&mut self, vm: &mut VM) {
        while !vm.halted {
            self.step(vm);
        }
    }

    pub fn step(&mut self, vm: &mut VM) {
        loop {
            let current_pc = vm.pc();
            let (insn, is_compressed) = self.fetch_decode(vm, current_pc);
            let next_pc = current_pc.wrapping_add(if is_compressed { 2 } else { 4 });
            vm.set_pc(next_pc);
            vm.execute_instruction(&insn, is_compressed, current_pc, &mut self.io);

            self.stats.insns = self.stats.insns.wrapping_add(1);
            if is_compressed {
                self.stats.compressed_insns = self.stats.compressed_insns.wrapping_add(1);
            }

            let is_terminal = insn.is_branch_or_jmp()
                || matches!(insn, Instruction::Illegal(_))
                || matches!(insn, Instruction::Ebreak)
                || vm.halted;
            if is_terminal {
                break;
            }
        }
    }

    fn fetch_decode(&self, vm: &mut VM, pc: u64) -> (Instruction, bool) {
        let insn16 = vm.load_u16(pc as usize);
        let is_compressed = insn16 & 0b11 != 0b11;
        if is_compressed {
            (decode_compressed(insn16), true)
        } else {
            let raw = vm.load_u32(pc as usize);
            (decode::decode(raw), false)
        }
    }
}
