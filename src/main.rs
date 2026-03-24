use std::env;
use std::fs;
use std::process;

use riscv_exec_stat::{init_from_elf, Runner};

enum CliError {
    Usage(String),
    Runtime(String),
}

struct CliArgs {
    elf_path: String,
    stdin_hex_path: Option<String>,
}

fn parse_args() -> Result<CliArgs, CliError> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "riscv-exec-stat".to_string());

    let mut elf_path: Option<String> = None;
    let mut stdin_hex_path: Option<String> = None;

    while let Some(arg) = args.next() {
        if arg == "--stdin" {
            if stdin_hex_path.is_some() {
                return Err(CliError::Usage(format!(
                    "--stdin provided more than once\nusage: {program} <path-to-elf> [--stdin <hex-file>]"
                )));
            }
            let path = args.next().ok_or_else(|| {
                CliError::Usage(format!(
                    "missing value for --stdin\nusage: {program} <path-to-elf> [--stdin <hex-file>]"
                ))
            })?;
            stdin_hex_path = Some(path);
            continue;
        }

        if arg.starts_with('-') {
            return Err(CliError::Usage(format!(
                "unknown flag: {arg}\nusage: {program} <path-to-elf> [--stdin <hex-file>]"
            )));
        }

        if elf_path.is_some() {
            return Err(CliError::Usage(format!(
                "unexpected extra positional argument: {arg}\nusage: {program} <path-to-elf> [--stdin <hex-file>]"
            )));
        }
        elf_path = Some(arg);
    }

    let elf_path = elf_path.ok_or_else(|| {
        CliError::Usage(format!(
            "usage: {program} <path-to-elf> [--stdin <hex-file>]"
        ))
    })?;

    Ok(CliArgs {
        elf_path,
        stdin_hex_path,
    })
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
