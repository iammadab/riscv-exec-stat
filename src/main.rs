use std::env;
use std::process;

use riscv_exec_stat::{init_from_elf, Runner};

enum CliError {
    Usage(String),
    Runtime(String),
}

fn run() -> Result<i32, CliError> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "riscv-exec-stat".to_string());

    let elf_path = match args.next() {
        Some(path) => path,
        None => {
            return Err(CliError::Usage(format!("usage: {program} <path-to-elf>")));
        }
    };

    if args.next().is_some() {
        return Err(CliError::Usage(format!("usage: {program} <path-to-elf>")));
    }

    let mut vm = init_from_elf(&elf_path)
        .map_err(|e| CliError::Runtime(format!("failed to read ELF '{elf_path}': {e}")))?;
    let mut runner = Runner::new();
    runner.run(&mut vm);

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
