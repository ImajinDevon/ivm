The official virtual machine/instruction set for [ivm](https://github.com/imajindevon/ivm).\
See the [ivm wiki](https://github.com/imajindevon/ivm) on GitHub for more information.

# What is ivm?
ivm is an experimental, well-documented, and expansion-ready virtual machine/instruction set written purely in Rust.

ivm provides a fairly medium level Instruction wrapper, which makes porting your language to ivm easier than
imaginable.

# Example
When combined with [ivm-compile](https://crates.io/crates/ivm-compile), we can achieve some pretty simple results
with impressive ease.

```rust
let program_options = ProgramOptions::new(options::CCFV, MemoryPointerLength::X32b);

let instructions = [
    Instruction::Push(ReadOperation::Local(b"Hello, world!\n".to_vec())),
    Instruction::ExternCall(ivm_ext_x32::STDOUT_WRITE),
    Instruction::ExternCall(ivm_ext_x32::STDOUT_FLUSH),
];

let bytecode = ivm_compile::compile_all(&program_options, instructions);

let mut vm = VmInstance::init(program_options);

vm.introduce(bytecode);
vm.continue_execution().unwrap();
```
