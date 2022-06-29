/// The current compile feature version of this build.
///
/// The compile feature version is incremented whenever a new feature is added to the ivmc compiler,
/// or a change was implemented to the bytecode format.
///
/// The VM shall be backwards compatible between small version changes, but should not be held to
/// the standard of forward compatibility.
///
/// However, this does not mean that the VM will never deprecate features and/or mark them for
/// removal.
pub const CCFV: u32 = 1;

pub mod header_format_doc {
    //! This module's purpose is purely for documentation.
    //!
    //! The documentation within this module declares the ivmc bytecode header format.
    //!
    //! # Format (pseudo)
    //! ```txt
    //! /// 4 bytes - big endian u32.
    //! /// **required - since CFV 1**
    //! CompileFeatureVersion: [u8; 4],
    //!
    //! /// Will be mapped to the MemoryPointerLength enum
    //! /// See [MemoryPointerLength::as_header_byte].
    //! /// **required - since CFV 1**
    //! MemoryPointerLength: u8
    //! ```
}

/// An enum deciding the amount of bytes required to point to a location in memory.
///
/// # Pointing to the max memory index
/// ```txt
/// X32b => [0xFF, 0xFF, 0xFF, 0xFF]
/// X64b => [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
/// ```
pub enum MemoryPointerLength {
    /// 32 bit memory pointers.
    /// 4 bytes ([u32]).
    /// See [std::mem::size_of()].
    X32b,

    /// 64 bit memory pointers.
    /// 8 bytes ([u64]).
    /// See [std::mem::size_of()].
    X64b,
}

impl MemoryPointerLength {
    /// Get the byte size of this memory size.
    ///
    /// `X32b => 4 bytes, X64b => 8 bytes.`
    pub fn get_span(&self) -> usize {
        match self {
            Self::X32b => 4,
            Self::X64b => 8,
        }
    }

    /// Get the representation of this memory size as a byte.
    /// This will be placed into the bytecode header.
    pub fn as_byte_repr(&self) -> u8 {
        match self {
            Self::X32b => 0,
            Self::X64b => 1,
        }
    }

    /// Match the memory pointer length from the given byte.
    ///
    /// See [MemoryPointerLength::as_byte_repr()].
    pub fn from_byte_repr(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::X32b),
            1 => Some(Self::X64b),
            _ => None,
        }
    }
}

/// A struct containing the required options for the VM.
/// This struct represents an ivmc bytecode header.
///
/// See [ProgramOptions::write_bytecode()].
pub struct ProgramOptions {
    cfv: u32,
    memory_size: MemoryPointerLength,
}

impl ProgramOptions {
    /// Get the compile feature version these program options are based on.
    pub fn get_cfv(&self) -> u32 {
        self.cfv
    }

    /// Write these options as a bytecode header into the given [Vec].
    pub fn write_bytecode(&self, output: &mut Vec<u8>) {
        output.extend(CCFV.to_be_bytes());
        output.push(self.memory_size.as_byte_repr());
    }

    pub fn new(cfv: u32, memory_size: MemoryPointerLength) -> Self {
        Self { cfv, memory_size }
    }
}

pub enum InvalidHeaderCause {
    /// The header format was not fulfilled.
    /// For example, the header did not specify a CFV, and/or the memory pointer length.
    FormatNotFulfilled,

    /// This value was not recognized.
    UnrecognizedValue,
}

impl InvalidHeaderCause {
    pub fn get_help(&self) -> &[&str] {
        const DOC_HELP: &str =
            "see [ivm_compile::options::header_format_doc] for documentation on matching the ivmc b\
            ytecode header format";

        match self {
            Self::FormatNotFulfilled => &[DOC_HELP],
            Self::UnrecognizedValue => &[
                "this bytecode input may have been compiled by a later version of ivmc",
                DOC_HELP,
            ],
        }
    }
}

/// An error returned when the header of a bytecode input did not meet the official ivmc bytecode
/// header format.
pub struct InvalidHeaderError {
    cause: InvalidHeaderCause,
    message: String,
}

impl InvalidHeaderError {
    /// Get the cause of this error.
    pub fn get_cause(&self) -> &InvalidHeaderCause {
        &self.cause
    }

    /// Get the message of this error.
    pub fn get_message(&self) -> &String {
        &self.message
    }

    fn from(cause: InvalidHeaderCause, message: &str) -> Self {
        Self {
            cause,
            message: message.to_string(),
        }
    }
}

pub mod version_adapters {
    //! A module containing adapters for differing compile feature versions.
    //!
    //! Be sure to read on [super::header_format_doc] to fully understand the bytecode format.

    use crate::options::{
        InvalidHeaderCause, InvalidHeaderError, MemoryPointerLength, ProgramOptions,
    };

    /// The header length of the current compile feature version.
    /// See [super::CCFV].
    pub const CCFV_HEADER_LEN: u32 = 5;

    /// Get the header size of this compile feature version.
    /// Returns `None` if the compile feature version is not recognized.
    pub fn get_header_size(compile_feature_version: u32) -> Option<u32> {
        match compile_feature_version {
            super::CCFV => Some(CCFV_HEADER_LEN),
            _ => None,
        }
    }

    /// This function will always support backwards compatibility, but forward compatibility is not
    /// guarenteed.
    ///
    /// Be sure to read on [super::header_format_doc] to fully understand the bytecode format.
    ///
    /// Returns a tuple containing the [ProgramOptions] and the length in bytes of the header
    /// that was read.
    pub fn get_program_options(
        bytes: &[u8],
    ) -> Result<(ProgramOptions, usize), InvalidHeaderError> {
        if bytes.len() < 5 {
            return Err(InvalidHeaderError::from(
                InvalidHeaderCause::FormatNotFulfilled,
                "header input too short",
            ));
        }

        // irrelevant as of CFV 1
        let cfv = u32::from_be_bytes(bytes[..4].try_into().unwrap());

        let mem_ptr_len = match MemoryPointerLength::from_byte_repr(bytes[4]) {
            Some(mpl) => mpl,
            None => {
                return Err(InvalidHeaderError::from(
                    InvalidHeaderCause::UnrecognizedValue,
                    "unrecognized memory pointer length",
                ));
            }
        };
        Ok((ProgramOptions::new(cfv, mem_ptr_len), 5))
    }
}
