use crate::VM;

pub fn handle_halt(vm: &mut VM) {
    vm.halted = true;
    vm.exit_code = vm.reg(10);
}
