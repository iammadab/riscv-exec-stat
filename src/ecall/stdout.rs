use crate::{ecall::constants, VM};

/// @dev this function would heavily be designed following the Linux ABI
pub fn handle_stdout(vm: &mut VM) {
    // Arguments according to RISC-V calling convention:
    // a0 (x10) = File Descriptor
    // a1 (x11) = Buffer Pointer (Guest Virtual Address)
    // a2 (x12) = Length to read
    let fd = vm.reg(10);
    let guest_ptr = vm.reg(11);
    let len = vm.reg(12);

    let output_slice = vm.read_bytes(guest_ptr as usize, len as usize);

    match fd {
        constants::STDOUT_FILENO => {
            let s = String::from_utf8_lossy(&output_slice);
            println!("{}", s);
        }
        constants::STDERR_FILENO => {
            let s = String::from_utf8_lossy(&output_slice);
            eprintln!("{}", s);
        }
        _ => {
            // Return -1 (error) in a0
            vm.reg_mut(10, (-1i64) as u64);
            return;
        }
    }

    vm.reg_mut(10, len);
}
