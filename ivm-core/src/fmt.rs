use ivm_compile::bytecode::byte_instructions;
use ivm_compile::options::{version_adapters, InvalidHeaderError};

//TODO Will probably be converted to use the Instruction enum

pub trait InstructionNameMapper {
    fn get_name(&self, byte: u8) -> String;
}

pub struct DefaultNameMapper;

impl InstructionNameMapper for DefaultNameMapper {
    fn get_name(&self, byte: u8) -> String {
        for bi in byte_instructions::values {
            if bi.get_byte() == byte {
                return bi.get_name().to_string();
            }
        }
        "unknown".to_string()
    }
}

pub struct BytecodeFormatter {
    byte_formatter: Box<dyn Fn(u8) -> String>,
    name_mapper: Box<dyn InstructionNameMapper>,
}

impl BytecodeFormatter {
    /// Format the given bytecode.
    pub fn format_bytecode(&self, bytecode: &[u8]) -> Result<Vec<String>, InvalidHeaderError> {
        let (options, mut i) = version_adapters::get_program_options(bytecode)?;
        let mut result = Vec::with_capacity(5);

        result.push(format!("compile feature version: {}", options.get_cfv()));

        while i < bytecode.len() {
            let byte = bytecode[i];

            result.push(format!(
                "{byte} {iname}",
                byte = (self.byte_formatter)(byte),
                iname = self.name_mapper.get_name(byte)
            ));
        }
        Ok(result)
    }
}

impl BytecodeFormatter {
    pub fn new(
        byte_formatter: Box<dyn Fn(u8) -> String>,
        name_mapper: Box<dyn InstructionNameMapper>,
    ) -> Self {
        Self {
            byte_formatter,
            name_mapper,
        }
    }
}

impl Default for BytecodeFormatter {
    fn default() -> Self {
        Self {
            byte_formatter: Box::new(|b| format!("{:02x}", b)),
            name_mapper: Box::new(DefaultNameMapper),
        }
    }
}
