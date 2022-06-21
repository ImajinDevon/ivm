pub mod b_instruc {
    /// The amount of bytes used to store and retrieve a memory entry.
    /// See [std::mem::size_of::<u32>()], where it declares a u32 is 4 bytes.\
    /// This constant may change in the future.
    pub const PTR_LEN: usize = std::mem::size_of::<u32>();

    pub const INVOKE: u8 = 5;
    pub const PUSH_BYTES: u8 = 6;
    pub const STATIC_VAR_STORE: u8 = 15;
    pub const STATIC_VAR_LOAD: u8 = 16;

    pub fn get_instruction_name<'a>(instruction: u8) -> &'a str {
        match instruction {
            INVOKE => "invoke",
            PUSH_BYTES => "push_bytes",
            STATIC_VAR_STORE => "static_var_store",
            STATIC_VAR_LOAD => "static_var_load",
            _ => panic!("unknown instruction: {:02x}", instruction),
        }
    }
}

pub enum Instruction {
    Invoke(u32),
    PushBytes(Vec<u8>),
    StaticVarStore(u32),
    StaticVarLoad(u32),
}

impl Instruction {
    pub fn get_byte(&self) -> u8 {
        match self {
            Self::Invoke(_) => b_instruc::INVOKE,
            Self::PushBytes(_) => b_instruc::PUSH_BYTES,
            Self::StaticVarStore(_) => b_instruc::STATIC_VAR_STORE,
            Self::StaticVarLoad(_) => b_instruc::STATIC_VAR_LOAD,
        }
    }

    pub fn write_bytes(&self, output: &mut Vec<u8>) {
        match self {
            Self::Invoke(ptr_index)
            | Self::StaticVarStore(ptr_index)
            | Self::StaticVarLoad(ptr_index) => {
                let index_bytes = ptr_index.to_be_bytes();
                output.push(self.get_byte());
                output.extend_from_slice(&index_bytes);
            }
            Self::PushBytes(raw) => {
                output.push(self.get_byte());
                output.extend((raw.len() as u32).to_be_bytes());
                output.extend(raw);
            }
        }
    }
}

/// Turn a slice of instructions into IVM bytecode.
/// # Examples
/// ```
/// to_bytecode(&[
///     Instruction::StoreBytes(b"Hello, world!".to_vec()),
///     Instruction::Invoke(0),
/// ]);
/// ```
pub fn to_bytecode(instructions: &[Instruction]) -> Vec<u8> {
    let mut output = Vec::new();
    instructions
        .iter()
        .for_each(|instruction| instruction.write_bytes(&mut output));
    output
}
