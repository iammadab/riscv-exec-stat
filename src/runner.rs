use crate::decode::compressed::decode_compressed;
use crate::decode::Instruction;
use crate::{decode, HostIO, VM};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct ExecStats {
    pub insns: u64,
    pub compressed_insns: u64,
    pub insn_counts: BTreeMap<&'static str, u64>,
}

pub struct Runner {
    io: HostIO,
    stats: ExecStats,
}

impl Runner {
    pub const TOTAL_DEFINED_INSTRUCTIONS: usize = 95;

    pub fn new() -> Self {
        Self {
            io: HostIO::new(),
            stats: ExecStats::default(),
        }
    }

    pub fn set_input_stream(&mut self, input: Vec<u8>) {
        self.io.set_input_stream(input);
    }

    pub fn stats(&self) -> &ExecStats {
        &self.stats
    }

    pub fn cycles(&self) -> u64 {
        self.stats.insns
    }

    pub fn instruction_stats_sorted(&self) -> Vec<(&'static str, u64, f64)> {
        let total = self.stats.insns;
        let mut rows: Vec<(&'static str, u64, f64)> = self
            .stats
            .insn_counts
            .iter()
            .map(|(name, count)| {
                let pct = if total == 0 {
                    0.0
                } else {
                    (*count as f64 / total as f64) * 100.0
                };
                (*name, *count, pct)
            })
            .collect();

        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        rows
    }

    pub fn unique_instruction_count(&self) -> usize {
        self.stats.insn_counts.len()
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
            let name = mnemonic(&insn);
            *self.stats.insn_counts.entry(name).or_insert(0) += 1;
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

fn mnemonic(insn: &Instruction) -> &'static str {
    match insn {
        Instruction::Add(_) => "add",
        Instruction::Sub(_) => "sub",
        Instruction::Sll(_) => "sll",
        Instruction::Slt(_) => "slt",
        Instruction::Sltu(_) => "sltu",
        Instruction::Xor(_) => "xor",
        Instruction::Srl(_) => "srl",
        Instruction::Sra(_) => "sra",
        Instruction::Or(_) => "or",
        Instruction::And(_) => "and",
        Instruction::Addi(_) => "addi",
        Instruction::Slti(_) => "slti",
        Instruction::Sltiu(_) => "sltiu",
        Instruction::Xori(_) => "xori",
        Instruction::Ori(_) => "ori",
        Instruction::Andi(_) => "andi",
        Instruction::Slli(_) => "slli",
        Instruction::Srli(_) => "srli",
        Instruction::Srai(_) => "srai",
        Instruction::Lb(_) => "lb",
        Instruction::Lh(_) => "lh",
        Instruction::Lw(_) => "lw",
        Instruction::Lbu(_) => "lbu",
        Instruction::Lhu(_) => "lhu",
        Instruction::Sb(_) => "sb",
        Instruction::Sh(_) => "sh",
        Instruction::Sw(_) => "sw",
        Instruction::Beq(_) => "beq",
        Instruction::Bne(_) => "bne",
        Instruction::Blt(_) => "blt",
        Instruction::Bge(_) => "bge",
        Instruction::Bltu(_) => "bltu",
        Instruction::Bgeu(_) => "bgeu",
        Instruction::Jal(_) => "jal",
        Instruction::Jalr(_) => "jalr",
        Instruction::Lui(_) => "lui",
        Instruction::Auipc(_) => "auipc",
        Instruction::Ecall => "ecall",
        Instruction::Ebreak => "ebreak",
        Instruction::Nop => "nop",
        Instruction::Eother => "eother",
        Instruction::Addw(_) => "addw",
        Instruction::Subw(_) => "subw",
        Instruction::Sllw(_) => "sllw",
        Instruction::Srlw(_) => "srlw",
        Instruction::Sraw(_) => "sraw",
        Instruction::Addiw(_) => "addiw",
        Instruction::Slliw(_) => "slliw",
        Instruction::Srliw(_) => "srliw",
        Instruction::Sraiw(_) => "sraiw",
        Instruction::Ld(_) => "ld",
        Instruction::Lwu(_) => "lwu",
        Instruction::Sd(_) => "sd",
        Instruction::Mul(_) => "mul",
        Instruction::Mulh(_) => "mulh",
        Instruction::Mulhsu(_) => "mulhsu",
        Instruction::Mulhu(_) => "mulhu",
        Instruction::Div(_) => "div",
        Instruction::Divu(_) => "divu",
        Instruction::Rem(_) => "rem",
        Instruction::Remu(_) => "remu",
        Instruction::Mulw(_) => "mulw",
        Instruction::Divw(_) => "divw",
        Instruction::Divuw(_) => "divuw",
        Instruction::Remw(_) => "remw",
        Instruction::Remuw(_) => "remuw",
        Instruction::LrW(_) => "lr.w",
        Instruction::ScW(_) => "sc.w",
        Instruction::AmoSwapW(_) => "amoswap.w",
        Instruction::AmoAddW(_) => "amoadd.w",
        Instruction::AmoXorW(_) => "amoxor.w",
        Instruction::AmoAndW(_) => "amoand.w",
        Instruction::AmoOrW(_) => "amoor.w",
        Instruction::AmoMinW(_) => "amomin.w",
        Instruction::AmoMaxW(_) => "amomax.w",
        Instruction::AmoMinuW(_) => "amominu.w",
        Instruction::AmoMaxuW(_) => "amomaxu.w",
        Instruction::LrD(_) => "lr.d",
        Instruction::ScD(_) => "sc.d",
        Instruction::AmoSwapD(_) => "amoswap.d",
        Instruction::AmoAddD(_) => "amoadd.d",
        Instruction::AmoXorD(_) => "amoxor.d",
        Instruction::AmoAndD(_) => "amoand.d",
        Instruction::AmoOrD(_) => "amoor.d",
        Instruction::AmoMinD(_) => "amomin.d",
        Instruction::AmoMaxD(_) => "amomax.d",
        Instruction::AmoMinuD(_) => "amominu.d",
        Instruction::AmoMaxuD(_) => "amomaxu.d",
        Instruction::Csrrw(_) => "csrrw",
        Instruction::Csrrs(_) => "csrrs",
        Instruction::Csrrc(_) => "csrrc",
        Instruction::Csrrwi(_) => "csrrwi",
        Instruction::Csrrsi(_) => "csrrsi",
        Instruction::Csrrci(_) => "csrrci",
        Instruction::Illegal(_) => "illegal",
    }
}
