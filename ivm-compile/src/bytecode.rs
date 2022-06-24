use ivm_common::Instruction;

/// Turn a slice of instructions into IVM bytecode.
/// # Examples
/// ```
/// use ivm_common::Instruction;
/// use ivm_compile::bytecode;
///
/// let bytecode = bytecode::instructions_to_bytecode(&[
///     Instruction::PushBytes(b"Hello, world!".to_vec()),
/// ]);
/// ```
pub fn instructions_to_bytecode(instructions: &[Instruction]) -> Vec<u8> {
    let mut output = Vec::new();
    instructions
        .iter()
        .for_each(|instruction| to_bytecode(instruction, &mut output));
    output
}

/// Write the given instruction as bytecode into the given [Vec].
/// # Examples
/// ```
/// use ivm_common::Instruction;
/// use ivm_compile::bytecode;
///
/// let mut bytecode = Vec::new();
///
/// let instruction = Instruction::PushBytes(vec![0x00]);
/// println!("{:2x?}", bytecode::to_bytecode(&instruction, &mut bytecode));
/// ```
pub fn to_bytecode(instruction: &Instruction, output: &mut Vec<u8>) {
    match instruction {
        Instruction::InvokeStatic(pi)
        | Instruction::StaticVarStore(pi)
        | Instruction::StaticVarLoad(pi) => {
            let index_bytes = pi.to_be_bytes();
            output.push(instruction.get_byte());
            output.extend_from_slice(&index_bytes);
        }
        Instruction::PushBytes(ref raw) => {
            output.push(instruction.get_byte());
            output.extend((raw.len() as u32).to_be_bytes());
            output.extend(raw);
        }
    }
}
