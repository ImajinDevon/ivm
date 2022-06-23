use ivm_common::{byte_instruction, Instruction};

fn get_digit_amount(n: usize) -> usize {
    ((n as f32).log10().floor() + 1.0) as usize
}

pub fn format_instruction_set(instructions: &[Instruction]) -> Vec<String> {
    let mut result = Vec::with_capacity(instructions.len());
    let max_digits = get_digit_amount(instructions.len());

    for (i, instruc) in instructions.iter().enumerate() {
        let fmt = match instruc {
            Instruction::InvokeStatic(pi)
            | Instruction::StaticVarStore(pi)
            | Instruction::StaticVarLoad(pi) => {
                format!("-> {pi}")
            }

            Instruction::PushBytes(bytes) => {
                format!("{bytes:2x?}")
            }
        };

        let byte = instruc.get_byte();

        result.push(format!(
            "{indent}{i}: {byte} {} {fmt}  ;",
            byte_instruction::get_instruction_name(byte),
            indent = " ".repeat(max_digits - get_digit_amount(i)),
        ))
    }
    result
}
