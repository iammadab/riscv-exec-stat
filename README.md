# riscv-exec-stat

RISC-V RV64 IMAC interpreter executor with execution statistics.

## Quick Start

Run from the project root.

```bash
# 1) Basic run (fib)
cargo run -- guest-bin/fib-imac

# 2) Fib + register stats
cargo run -- guest-bin/fib-imac --reg-stats

# 3) Fib + instruction stats
cargo run -- guest-bin/fib-imac --insn-stats

# 4) Fib + both stats
cargo run -- guest-bin/fib-imac --reg-stats --insn-stats

# 5) Exec-block with hex stdin
cargo run -- guest-bin/exec-block-imac --stdin guest-input/exec-block.input

# 6) Exec-block + instruction stats
cargo run -- guest-bin/exec-block-imac --stdin guest-input/exec-block.input --insn-stats

# 7) Exec-block + both stats
cargo run -- guest-bin/exec-block-imac --stdin guest-input/exec-block.input --reg-stats --insn-stats
```

## CLI Usage

```bash
cargo run -- <path-to-elf> [--stdin <hex-file>] [--reg-stats] [--insn-stats]
```

Flags:
- `--stdin <hex-file>`: Reads a hex-encoded file, decodes to bytes, and feeds guest stdin.
- `--reg-stats`: Prints per-register read/write/access counts and normalized access percent.
- `--insn-stats`: Prints per-instruction dynamic counts sorted by most used, plus percent.

## Bundled Example Assets

- `guest-bin/fib-imac`
- `guest-bin/exec-block-imac`
- `guest-input/exec-block.input` (hex-encoded stdin payload)

## Output Semantics

- The CLI process exits with the guest program exit code.
- Register stats percentages are normalized over total register accesses.
- Instruction stats percentages are normalized over total dynamic instruction count.
- `unique executed / total defined` in instruction stats means:
  - `unique executed`: number of instruction mnemonics that appeared in this run
  - `total defined`: `95` instruction enum buckets in this codebase (includes `illegal`)
  - compressed encodings are not counted as separate instruction buckets

## Notes

- `exec-block` is substantially heavier than `fib` and may take much longer to finish.
- `--stdin` currently assumes the input file is hex text.
