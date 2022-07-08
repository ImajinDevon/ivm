use ivm_compile::{Instruction, ReadOperation};

fn format_read_op(read_op: &ReadOperation) -> String {
    match read_op {
        ReadOperation::Local(bytes) => format!("local({bytes:02x?})"),
        ReadOperation::Point(index) => format!("point({index})"),
    }
}

/// Format the given instructions.
pub fn format_instructions(instructions: &[Instruction]) -> Vec<String> {
    let mut result = Vec::new();

    for (i, instruction) in instructions.iter().enumerate() {
        result.push(format!(
            "{}, {:02x}  {}",
            i,
            instruction.get_identifier_byte(),
            match instruction {
                Instruction::Push(rdop) => format!("push: {}", format_read_op(rdop)),

                Instruction::ExternCall(index) => format!("extern call: {index}"),

                Instruction::Visit(index) => format!("visit: {index}"),

                Instruction::Mutate(index, rdop) => {
                    format!("mutate({index}) into {}", format_read_op(rdop))
                }
            }
        ));
    }
    result
}
