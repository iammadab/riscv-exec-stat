pub const ECALL_HALT: u64 = 93;
pub const ECALL_STD_INPUT: u64 = 63;
pub const ECALL_STD_OUTPUT: u64 = 64;

// This is not very useful to zmVMs, it is just to keep consistence with the Linux ABI
pub const STDIN_FILENO: u64 = 0;
pub const STDOUT_FILENO: u64 = 1;
pub const STDERR_FILENO: u64 = 2;
