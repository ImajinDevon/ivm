use crate::{Instruction, ProgramOptions};

#[derive(Default)]
pub struct Label {
    instructions: Vec<Instruction>,
}

impl Label {
    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }

    pub fn compile(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        self.instructions
            .iter()
            .for_each(|i| i.compile(dest, program_options))
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}
