use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
use ivm_compile::{options, Instruction, ReadOperation, Compile};
use ivm_vm::ivm_ext_x32::IvmX32ExternMap;
use ivm_vm::{ivm_ext_x32, ExecutionEnvironment, VmInstance};

fn main() {
    let options = ProgramOptions::default();
    let bytecode = ivm_compile::compile_all([
        Instruction::Push(ReadOperation::Local(b"Hello, world!\n".to_vec())),
        Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE),
    ], &options);

    let mut vm = VmInstance::reserve_ivm_ext_x32(options);
    let mut extern_map = IvmX32ExternMap;

    let mut env = ExecutionEnvironment::new(&mut extern_map);

    vm.introduce(bytecode);
    vm.continue_execution(&mut env);
}
