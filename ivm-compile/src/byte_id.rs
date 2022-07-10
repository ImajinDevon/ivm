//! A module containing each [crate::Instruction]'s byte identifier.

/// See [crate::Instruction::Jump].
pub const I_JUMP: u8 = 0;

/// See [crate::Instruction::Push].
pub const I_PUSH: u8 = 1;

/// See [crate::Instruction::Mutate].
pub const I_MUTATE: u8 = 2;

/// See [crate::Instruction::ExternCall].
pub const I_EXTERN_CALL: u8 = 3;

/// See [crate::Instruction::Return].
pub const I_RETURN: u8 = 4;

/// See [crate::Instruction::Call].
pub const I_CALL: u8 = 5;

/// See [crate::ReadOperation::Local].
pub const RDOP_LOCAL: u8 = 0;

/// See [crate::ReadOperation::Point].
pub const RDOP_POINT: u8 = 1;
