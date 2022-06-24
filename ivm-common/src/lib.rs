pub mod cli;

pub mod byte_instruction {
    //! For information on the instructions, see [super::Instruction].

    pub const INVOKE_STATIC: u8 = 5;
    pub const PUSH_BYTES: u8 = 6;
    pub const STATIC_VAR_STORE: u8 = 15;
    pub const STATIC_VAR_LOAD: u8 = 16;

    /// # Table of bytes (instructions) and their functionality
    /// ```txt
    /// 5  | invoke
    /// 6  | push_bytes
    /// 15 | static_var_store
    /// 16 | static_var_load
    /// ```
    pub fn get_instruction_name<'a>(instruction: u8) -> &'a str {
        match instruction {
            INVOKE_STATIC => "invoke_static",
            PUSH_BYTES => "push_bytes",
            STATIC_VAR_STORE => "static_var_store",
            STATIC_VAR_LOAD => "static_var_load",
            _ => panic!("unknown instruction: {:02x}", instruction),
        }
    }
}

/// An executable instruction.
///
/// # Examples
/// ```
/// use ivm_common::Instruction;
///
/// let instructions = vec![
///     Instruction::PushBytes(vec![
///         0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21
///     ]),
/// ];
/// ```
pub enum Instruction {
    /// Invoke a static function.
    InvokeStatic(u32),

    /// Push raw bytes to the stack.
    PushBytes(Vec<u8>),

    /// Store the stack into a static variable.
    StaticVarStore(u32),

    /// Load a static variable.
    StaticVarLoad(u32),
}

impl Instruction {
    /// Matches this [Instruction]'s byte constant.
    /// See the [byte_instruction] module for more information on byte instruction constants.
    ///
    /// # Examples
    /// ```
    /// use ivm_common::{byte_instruction, Instruction};
    ///
    /// let instruction = Instruction::PushBytes(vec![0x00, 0x01]);
    /// assert_eq!(instruction.get_byte(), byte_instruction::PUSH_BYTES);
    /// ```
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::InvokeStatic(_) => byte_instruction::INVOKE_STATIC,
            Self::PushBytes(_) => byte_instruction::PUSH_BYTES,
            Self::StaticVarStore(_) => byte_instruction::STATIC_VAR_STORE,
            Self::StaticVarLoad(_) => byte_instruction::STATIC_VAR_LOAD,
        }
    }

    /// Utility wrapper to [byte_instruction::get_instruction_name(u8)].
    ///
    /// This method is not natively defined in this implementation, because otherwise, custom
    /// bytecode formatters would either have to construct (mock) instructions to bypass
    /// instructions that take values, or define their own function to get the name of an
    /// instruction.
    ///
    /// # Table of bytes (instructions) and their functionality
    /// ```txt
    /// 5  | invoke
    /// 6  | push_bytes
    /// 15 | static_var_store
    /// 16 | static_var_load
    /// ```
    pub fn get_name<'a>(&self) -> &'a str {
        byte_instruction::get_instruction_name(self.get_byte())
    }
}
