//! This crate provides a medium-level "instruction" wrapper for ivm bytecode.

use crate::options::ProgramOptions;

pub mod byte_id;
pub mod options;
pub mod version_adapters;
pub mod vmenv;

/// When the VM encounters an instruction that requires a value, it will perform a read operation.
pub enum ReadOperation {
    /// The bytes are hardcoded in the file, after this point.
    ///
    /// The **size** of this operation is declared after the identifier byte.
    Local(Vec<u8>),

    /// The bytes must be read at a memory pointer index.
    ///
    /// The first arg tells how many bytes shall be read, and the latter provides the memory pointer
    /// index of the data.
    Point(usize, usize),
}

impl ReadOperation {
    /// Get the byte the VM will use to identify this kind of read operation.
    pub fn get_identifier_byte(&self) -> u8 {
        match self {
            Self::Local(_) => byte_id::RDOP_LOCAL,
            Self::Point(_, _) => byte_id::RDOP_POINT,
        }
    }

    /// Compile this read operation to its bytecode representation.
    pub fn compile(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        dest.push(self.get_identifier_byte());

        match self {
            Self::Local(v) => {
                dest.extend_from_slice(&program_options.get_ptr_len().fit(v.len()));
                dest.extend(v);
            }
            Self::Point(len, index) => {
                dest.extend_from_slice(&program_options.get_ptr_len().fit(*len));
                dest.extend_from_slice(&program_options.get_ptr_len().fit(*index));
            }
        }
    }
}

/// An enum representing a bytecode instruction.
///
/// For bytecode mapping, see the [byte_id] module.
pub enum Instruction {
    /// Set the execution index.
    ///
    /// The current execution index is not pushed to the call stack.
    /// If that is wished to be achieved, use [Self::Call].
    ///
    /// This is also known as the `goto` instruction.
    Jump(usize),

    /// Push bytes to the stack.
    Push(ReadOperation),

    /// Mutate a location in the memory pool.
    Mutate(usize, ReadOperation),

    /// Push the current execution index to the call stack, then visit this location.
    ///
    /// If the target returns, the current execution index will be jumped to.
    Call(usize),

    /// Call an external function.
    ///
    /// Similar to the `syscall` instruction in machine code.
    ExternCall(usize),

    /// Pop an index from the call stack, then continue execution at said index.
    ///
    /// If the call stack is empty, this will halt execution.
    Return,
}

impl Instruction {
    /// Get the bytecode identifier of this instruction.
    ///
    /// See the [byte_id] module for the byte mappings.
    pub fn get_identifier_byte(&self) -> u8 {
        match self {
            Self::Jump(_) => byte_id::I_JUMP,
            Self::Push(_) => byte_id::I_PUSH,
            Self::Mutate(_, _) => byte_id::I_MUTATE,
            Self::ExternCall(_) => byte_id::I_EXTERN_CALL,
            Self::Return => byte_id::I_RETURN,
            Self::Call(_) => byte_id::I_CALL,
        }
    }

    /// Compile this instruction to its bytecode representation.
    pub fn compile(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        dest.push(self.get_identifier_byte());

        match self {
            Self::ExternCall(ptr) | Self::Call(ptr) | Self::Jump(ptr) => {
                dest.extend(program_options.get_ptr_len().fit(*ptr))
            }

            Self::Push(rd) => rd.compile(dest, program_options),

            Self::Mutate(ptr_dest, value) => {
                dest.extend(program_options.get_ptr_len().fit(*ptr_dest));
                value.compile(dest, program_options);
            }

            _ => (),
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
        .for_each(|i: Instruction| i.compile(&mut bytecode, program_options));

    bytecode
}
