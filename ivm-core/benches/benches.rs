use std::time::Instant;

use ivm_compile::options::ProgramOptions;
use ivm_compile::{Instruction, ReadOperation};
use ivm_vm::ivm_ext_x32::IvmX32ExternMap;
use ivm_vm::{ivm_ext_x32, ExecutionEnvironment, VmInstance};

#[inline(always)]
pub fn vm_ivm_ext_x32<I>(instructions: I) -> VmInstance
where
    I: IntoIterator<Item = Instruction>,
{
    let program_options = ProgramOptions::default();
    let bytecode = ivm_compile::compile_all(instructions, &program_options);

    let mut vm = VmInstance::reserve_ivm_ext_x32(program_options);
    vm.introduce(bytecode);
    vm
}

fn hello_world() {
    let mut vm = vm_ivm_ext_x32([
        Instruction::LoadA(ReadOperation::Local(b"Hello, world!\n".to_vec())),
        Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE),
    ]);

    let mut extern_map = IvmX32ExternMap;
    let mut env = ExecutionEnvironment::new(&mut extern_map);

    let start = Instant::now();

    vm.continue_execution(&mut env);
    println!("{:?}", start.elapsed());
}

fn main() {
    hello_world();
}
