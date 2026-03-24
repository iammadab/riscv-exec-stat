use std::env;
use std::fs;
use std::process;

use riscv_exec_stat::{init_from_elf, Runner, VM};

enum CliError {
    Usage(String),
    Runtime(String),
}

struct CliArgs {
    elf_path: String,
    stdin_hex_path: Option<String>,
    reg_stats: bool,
    insn_stats: bool,
}

fn parse_args() -> Result<CliArgs, CliError> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "riscv-exec-stat".to_string());

    let mut elf_path: Option<String> = None;
    let mut stdin_hex_path: Option<String> = None;
    let mut reg_stats = false;
    let mut insn_stats = false;

    while let Some(arg) = args.next() {
        if arg == "--stdin" {
            if stdin_hex_path.is_some() {
                return Err(CliError::Usage(format!(
                    "--stdin provided more than once\nusage: {program} <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]"
                )));
            }
            let path = args.next().ok_or_else(|| {
                CliError::Usage(format!(
                    "missing value for --stdin\nusage: {program} <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]"
                ))
            })?;
            stdin_hex_path = Some(path);
            continue;
        }

        if arg == "--reg-stats" {
            reg_stats = true;
            continue;
        }

        if arg == "--insn-stats" {
            insn_stats = true;
            continue;
        }

        if arg.starts_with('-') {
            return Err(CliError::Usage(format!(
                "unknown flag: {arg}\nusage: {program} <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]"
            )));
        }

        if elf_path.is_some() {
            return Err(CliError::Usage(format!(
                "unexpected extra positional argument: {arg}\nusage: {program} <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]"
            )));
        }
        elf_path = Some(arg);
    }

    let elf_path = elf_path.ok_or_else(|| {
        CliError::Usage(format!(
            "usage: {program} <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]"
        ))
    })?;

    Ok(CliArgs {
        elf_path,
        stdin_hex_path,
        reg_stats,
        insn_stats,
    })
}

fn format_with_commas(value: u64) -> String {
    let s = value.to_string();
    let mut out = String::with_capacity(s.len() + (s.len().saturating_sub(1) / 3));
    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

fn print_reg_stats(vm: &VM) {
    const ABI_NAMES: [&str; 32] = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
        "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
        "t5", "t6",
    ];

    let reads = vm.reg_reads();
    let writes = vm.reg_writes();
    let accesses = vm.reg_accesses();
    let probs = vm.reg_access_probabilities();

    println!();
    println!(
        "{:<4} {:<5} {:>14} {:>14} {:>14} {:>10}",
        "reg", "name", "reads", "writes", "accesses", "percent"
    );
    println!(
        "{:-<4} {:-<5} {:-<14} {:-<14} {:-<14} {:-<10}",
        "", "", "", "", "", ""
    );
    for i in 0..32 {
        let reads_fmt = format_with_commas(reads[i]);
        let writes_fmt = format_with_commas(writes[i]);
        let accesses_fmt = format_with_commas(accesses[i]);
        let pct = probs[i] * 100.0;
        println!(
            "{:<4} {:<5} {:>14} {:>14} {:>14} {:>9.2}%",
            format!("x{i}"),
            ABI_NAMES[i],
            reads_fmt,
            writes_fmt,
            accesses_fmt,
            pct
        );
    }
}

fn print_insn_stats(runner: &Runner) {
    let stats = runner.stats();
    let unique = runner.unique_instruction_count();
    let defined = Runner::TOTAL_DEFINED_INSTRUCTIONS;
    println!();
    println!("total instructions: {}", format_with_commas(stats.insns));
    println!(
        "unique executed / total defined: {} / {}",
        format_with_commas(unique as u64),
        format_with_commas(defined as u64)
    );
    println!("{:<16} {:>14} {:>10}", "instruction", "count", "percent");
    println!("{:-<16} {:-<14} {:-<10}", "", "", "");

    for (name, count, pct) in runner.instruction_stats_sorted() {
        println!(
            "{:<16} {:>14} {:>9.2}%",
            name,
            format_with_commas(count),
            pct
        );
    }
}

fn run() -> Result<i32, CliError> {
    let cli = parse_args()?;

    let mut vm = init_from_elf(&cli.elf_path)
        .map_err(|e| CliError::Runtime(format!("failed to read ELF '{}': {e}", cli.elf_path)))?;

    let mut runner = Runner::new();

    if let Some(stdin_hex_path) = cli.stdin_hex_path {
        let input_hex = fs::read_to_string(&stdin_hex_path).map_err(|e| {
            CliError::Runtime(format!(
                "failed to read stdin hex file '{}': {e}",
                stdin_hex_path
            ))
        })?;
        let input_hex = input_hex.trim();
        let input_bytes = hex::decode(input_hex).map_err(|e| {
            CliError::Runtime(format!(
                "failed to decode stdin hex file '{}': {e}",
                stdin_hex_path
            ))
        })?;
        runner.set_input_stream(input_bytes);
    }

    runner.run(&mut vm);

    if cli.reg_stats {
        print_reg_stats(&vm);
    }

    if cli.insn_stats {
        print_insn_stats(&runner);
    }

    Ok(vm.exit_code() as i32)
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(CliError::Usage(msg)) => {
            eprintln!("{msg}");
            process::exit(2);
        }
        Err(CliError::Runtime(msg)) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    }
}
