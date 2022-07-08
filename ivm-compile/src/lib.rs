//! This crate provides a medium-level "instruction" wrapper for ivm bytecode.

use crate::options::ProgramOptions;

pub mod byte_id;
pub mod options;
pub mod version_adapters;

/// When the VM encounters an instruction that requires a value, it will perform a read operation.
pub enum ReadOperation {
    /// The bytes are hardcoded in the file, after this point.
    /// The **size** of this operation is declared after the identifier byte.
    Local(Vec<u8>),

    /// The bytes are located at a different memory pointer index. The **size** of this operation
    /// is first declared at the index.
    Point(usize),
}

impl ReadOperation {
    /// Get the byte the VM will use to identify this kind of read operation.
    pub fn get_identifier_byte(&self) -> u8 {
        match self {
            Self::Local(_) => byte_id::RDOP_LOCAL,
            Self::Point(_) => byte_id::RDOP_POINT,
        }
    }

    /// Compile this read operation to its bytecode representation.
    pub fn compile(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        dest.push(self.get_identifier_byte());

        match self {
            Self::Local(v) => {
                dest.extend_from_slice(&program_options.ptr_len.fit(v.len()));
                dest.extend(v);
            }
            Self::Point(d) => dest.extend_from_slice(&program_options.ptr_len.fit(*d)),
        }
    }
}

/// An enum representing a bytecode instruction.
/// For bytecode mapping, see the [byte_id] module.
pub enum Instruction {
    /// Set the execution pointer index.
    /// This is also known as the `call` or `goto` instruction.
    Visit(usize),

    /// Push data to the stack.
    Push(ReadOperation),

    /// Mutate a location in the memory pool.
    Mutate(usize, ReadOperation),

    /// Call an external function.
    /// Similar to the `syscall` instruction in machine code.
    ExternCall(usize),
}

impl Instruction {
    /// Get the bytecode identifier of this instruction.
    /// See the [byte_id] module for the byte mapping.
    pub fn get_identifier_byte(&self) -> u8 {
        match self {
            Self::Visit(_) => byte_id::I_VISIT,
            Self::Push(_) => byte_id::I_PUSH,
            Self::Mutate(_, _) => byte_id::I_MUTATE,
            Self::ExternCall(_) => byte_id::I_EXTERN_CALL,
        }
    }

    /// Compile this instruction to its bytecode representation.
    pub fn compile(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        dest.push(self.get_identifier_byte());

        let size = &program_options.ptr_len;

        match self {
            Self::Visit(ptr) | Self::ExternCall(ptr) => dest.extend(size.fit(*ptr)),

            Self::Push(rdop) => rdop.compile(dest, program_options),

            Self::Mutate(ptr_dest, value) => {
                dest.extend(size.fit(*ptr_dest));
                value.compile(dest, program_options);
            }
        }
    }
}

/// Compiles every instruction in the given [IntoIterator], then returns the combined bytecode.
///
/// See [Instruction::compile].
pub fn compile_all<I>(program_options: &ProgramOptions, it: I) -> Vec<u8>
where
    I: IntoIterator<Item = Instruction>,
{
    let mut bytecode = Vec::new();

    it.into_iter()
        .for_each(|i: Instruction| i.compile(&mut bytecode, &program_options));

    bytecode
}
