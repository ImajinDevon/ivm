use ivm_compile::options::ProgramOptions;
use ivm_compile::{Compile, Instruction, ReadOperation};

fn format_read_op(read_op: &ReadOperation) -> String {
    match read_op {
        ReadOperation::Local(bytes) => format!(
            "<\x1b[92mlocal\x1b[0m> ({})",
            bytes
                .iter()
                .map(|b| format!("\x1b[94m{b:02x}\x1b[0m"))
                .collect::<Vec<_>>()
                .join(", ")
        ),

        ReadOperation::Point(size, index) => {
            format!("<\x1b[95mpoint\x1b[0m> (size: {size}, index: {index})")
        }
    }
}

fn fmt_ptr(ptr: usize) -> String {
    format!("\x1b[91m\x1b[1m{ptr}")
}

fn get_instruction_prefix(instruction: &Instruction) -> String {
    format!(
        "\x1b[1m{}\x1b[0m",
        match instruction {
            Instruction::Push(_) => "\x1b[91mpush",
            Instruction::Jump(_) => "\x1b[92mjump",
            Instruction::Mutate(_, _) => "\x1b[93mmutate",
            Instruction::Return => "\x1b[34mreturn",
            Instruction::ExternCall(_) => "\x1b[95mextern_call",
            Instruction::Call(_) => "\x1b[36mcall",
            Instruction::LoadA(_) => "\x1b[33mload %a%",
        }
    )
}

fn display_value(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Push(rd) | Instruction::LoadA(rd) => format_read_op(rd),

        Instruction::ExternCall(ptr) | Instruction::Jump(ptr) | Instruction::Call(ptr) => {
            fmt_ptr(*ptr)
        }

        Instruction::Mutate(ptr, rd) => format!("{} -> {}", fmt_ptr(*ptr), format_read_op(rd)),

        _ => unreachable!(),
    }
}

/// Format the given instruction.
pub fn format_instruction(instruction: &Instruction) -> String {
    let name = get_instruction_prefix(instruction);

    match instruction {
        Instruction::Return => name,
        _ => format!("{} {}\x1b[0m", name, display_value(instruction)),
    }
}

pub fn print_instructions<'a, I>(
    program_options: &ProgramOptions,
    instructions: I,
    show_bytecode: bool,
) where
    I: IntoIterator<Item = &'a Instruction>,
{
    for instruction in instructions {
        println!("{}", format_instruction(instruction));

        let temp = instruction.compile(program_options);

        if !show_bytecode {
            continue;
        }

        let raw = temp
            .into_iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");

        println!("\x1b[30m| bytecode: {raw}\x1b[0m");
    }
}
