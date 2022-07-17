use crate::Compile;
use std::fmt::{Display, Formatter};

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
    //! // 4 bytes: little endian u32.
    //! // **required - since CFV 1**
    //! CompileFeatureVersion: [u8; 4],
    //!
    //! // Will be mapped to the MemoryPointerLength enum.
    //! // See [MemoryPointerLength::get_byte_identifier].
    //! // **required - since CFV 1**
    //! MemoryPointerLength: MemoryPointerLength#get_byte_identifier()
    //! ```
}

/// An enum deciding the amount of bytes required to point to a location in memory.
/// If the extra memory is not needed, using a smaller MemoryPointerLength can be file size
/// optimization.
///
/// # Pointing to the max memory index
/// ```txt
/// X32b => [0xFF, 0xFF, 0xFF, 0xFF]
/// X64b => [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
/// ```
#[derive(Clone, Debug)]
pub enum MemoryPointerLength {
    /// 32 bit memory pointers - (4 bytes).
    X32b,

    /// 64 bit memory pointers - (8 bytes).
    X64b,
}

impl MemoryPointerLength {
    /// Extract a usize from the memory pool at the given start index, until this
    /// MemoryPointerLength's [span](Self::get_span()) is satisfied.
    ///
    /// See [Self::get_span()], [Self::to_usize(&\[u8\])].
    #[inline]
    pub fn extract(&self, index: usize, pool: &[u8]) -> usize {
        self.to_usize(&pool[index..][..self.get_span()])
    }

    /// Convert the given input to a usize.
    ///
    /// Panics if this length cannot be fit into a usize.
    #[inline]
    pub fn to_usize(&self, input: &[u8]) -> usize {
        debug_assert_eq!(self.get_span(), input.len());

        match self {
            Self::X32b => u32::from_le_bytes(input.try_into().unwrap()) as usize,
            Self::X64b => u64::from_le_bytes(input.try_into().unwrap()) as usize,
        }
    }

    /// Convert a memory pointer index to its little-endian byte representation.
    #[inline]
    pub fn fit(&self, mem_ptr_index: usize) -> Vec<u8> {
        mem_ptr_index.to_le_bytes()[..self.get_span()].to_vec()
    }

    /// Get the byte size of this memory size.
    ///
    /// `X32b => 4 bytes, X64b => 8 bytes.`
    #[inline]
    pub const fn get_span(&self) -> usize {
        match self {
            Self::X32b => 4,
            Self::X64b => 8,
        }
    }

    /// Get the this memory pointer length's byte identifier.
    ///
    /// This will be placed into the bytecode header.
    ///
    /// See [options::header_format_doc] for a full guide regarding the ivm bytecode format.
    #[inline]
    pub const fn get_byte_identifier(&self) -> u8 {
        match self {
            Self::X32b => 0,
            Self::X64b => 1,
        }
    }

    /// Match the memory pointer length from the given byte.
    ///
    /// See [MemoryPointerLength::get_byte_identifier()].
    #[inline]
    pub const fn from_byte_identifier(byte: u8) -> Option<Self> {
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
/// See [ProgramOptions::write_bytecode(Vec)].
#[derive(Clone, Debug)]
pub struct ProgramOptions {
    cfv: u32,
    ptr_len: MemoryPointerLength,
}

impl ProgramOptions {
    /// Get the compile feature version that this program was compiled on.
    ///
    /// See the [current compile feature version](CCFV).
    #[inline]
    pub const fn cfv(&self) -> u32 {
        self.cfv
    }

    /// Create a new ProgramOptions.
    ///
    /// The [Default] implementation for this struct uses the
    /// [current compile feature version](CCFV) and a
    /// [32 bit memory pointer length](MemoryPointerLength::X32b).
    #[inline]
    pub const fn new(cfv: u32, ptr_len: MemoryPointerLength) -> Self {
        Self { cfv, ptr_len }
    }

    /// Get the [MemoryPointerLength] that this program uses.
    #[inline]
    pub const fn ptr_len(&self) -> &MemoryPointerLength {
        &self.ptr_len
    }
}

impl Compile for ProgramOptions {
    fn compile_into(&self, dest: &mut Vec<u8>, _program_options: &ProgramOptions) {
        dest.extend(self.cfv.to_le_bytes());
        dest.push(self.ptr_len.get_byte_identifier());
    }
}

impl Default for ProgramOptions {
    fn default() -> Self {
        Self::new(CCFV, MemoryPointerLength::X32b)
    }
}

pub enum InvalidHeaderCause {
    /// The header format was not fulfilled.
    ///
    /// For example, the header did not specify a CFV, and/or the memory pointer length.
    FormatNotFulfilled,

    /// The value was not recognized.
    ///
    /// For example, a CFV is too large.
    UnrecognizedValue,
}

impl InvalidHeaderCause {
    #[inline]
    pub const fn get_help<'a>(&self) -> &'a [&'a str] {
        const DOC_HELP: &str =
            "see [ivm_compile::options::header_format_doc] for the ivm bytecode format";

        match self {
            Self::FormatNotFulfilled => &[DOC_HELP],
            Self::UnrecognizedValue => &[
                "this bytecode input may have been compiled by a later version of ivmc",
                DOC_HELP,
            ],
        }
    }
}

impl Display for InvalidHeaderCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnrecognizedValue => "an unrecognized value was encountered",
                Self::FormatNotFulfilled => "the header format was not fulfilled",
            }
        )
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
    #[inline]
    pub const fn cause(&self) -> &InvalidHeaderCause {
        &self.cause
    }

    /// Get the message of this error.
    #[inline]
    pub const fn message(&self) -> &String {
        &self.message
    }

    #[inline]
    pub const fn new(cause: InvalidHeaderCause, message: String) -> Self {
        Self { cause, message }
    }

    #[inline]
    pub fn from(cause: InvalidHeaderCause, message: &str) -> Self {
        Self::new(cause, message.to_string())
    }
}

impl Display for InvalidHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.cause, self.message)
    }
}
