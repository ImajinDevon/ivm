use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
use ivm_compile::{options, Instruction, ReadOperation};
use ivm_vm::{ivm_ext_x32, VmInstance};
use std::time::Instant;

fn hello_world_instructions() -> [Instruction; 3] {
    [
        Instruction::Push(ReadOperation::Local(b"Hello, world!\n".to_vec())),
        Instruction::ExternCall(ivm_ext_x32::STDOUT_WRITE),
        Instruction::ExternCall(ivm_ext_x32::STDOUT_FLUSH),
    ]
}

fn create_helloworld_vm() -> VmInstance {
    let program_options = ProgramOptions::new(options::CCFV, MemoryPointerLength::X32b);

    let bytecode = ivm_compile::compile_all(&program_options, hello_world_instructions());

    let mut vm = VmInstance::init(program_options);
    vm.introduce(bytecode);
    vm
}

fn hello_world() -> Result<(), String> {
    let mut vm = create_helloworld_vm();
    vm.continue_execution()
}

#[test]
fn bad_helloworld_benchmark_no_warmup() {
    let now = Instant::now();
    hello_world().expect("an error occurred");
    println!("{:?}", now.elapsed());
}
