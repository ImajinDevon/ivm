use std::time::Instant;

use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
use ivm_compile::{options, Instruction, ReadOperation};
use ivm_vm::ivm_ext_x32::IvmX32ExternMap;
use ivm_vm::{ivm_ext_x32, ExecutionEnvironment, VmInstance};

fn vm_ivm_ext_x32<I>(instructions: I) -> VmInstance
where
    I: IntoIterator<Item = Instruction>,
{
    let instructions = instructions.into_iter().collect::<Vec<_>>();
    let program_options = ProgramOptions::new(options::CCFV, MemoryPointerLength::X32b);

    //crate::fmt::print_instructions(&program_options, &instructions, true);

    let bytecode = ivm_compile::compile_all(&program_options, instructions);

    let mut vm = VmInstance::reserve_ivm_ext_x32(program_options);
    vm.introduce(bytecode);
    vm
}

#[test]
fn bad_helloworld_benchmark_no_warmup() {
    let mut vm = vm_ivm_ext_x32([
        Instruction::Push(ReadOperation::Local(b"Hello, world!\n".to_vec())),
        Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE),
        Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_FLUSH),
    ]);

    let mut extern_map = IvmX32ExternMap;
    let mut env = ExecutionEnvironment::unsecured(&mut extern_map);

    let start = Instant::now();
    vm.continue_execution(&mut env);
    println!("finished in: {:?}", start.elapsed());
}

/*#[test]
fn loop_forever() {
    let instructions = [Instruction::Jump(0)];
    let mut vm = vm_ivm_ext_x32(instructions);
    vm.continue_execution();
}*/
