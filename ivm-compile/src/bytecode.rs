pub struct ByteInstruction(&'static str, u8);

impl ByteInstruction {
    /// Get the ivm-fe name of this instruction.
    pub const fn get_name(&self) -> &'static str {
        self.0
    }

    /// Get the bytecode byte that represents the instruction.
    pub const fn get_byte(&self) -> u8 {
        self.1
    }
}

constant_pool! {
    /// A constant pool of the instructions by their bytes.
    /// # Example usage
    /// ```
    /// use ivm_compile::bytecode::byte_instructions;
    ///
    /// const fn assertions() {
    ///     assert_eq!(1, byte_instructions::CALL.get_byte());
    ///     assert_eq!("PUSH", byte_instructions::PUSH.get_name())
    /// }
    /// ```
    pub byte_instructions: ByteInstruction {
        CALL(1),
        PUSH(2),
        DROP(3),
        RET(4)
    }
}
