//! A module containing adapters for differing compile feature versions.
//!
//! See [options::header_format_doc] for a full guide regarding the ivm bytecode format.

use crate::options;
use crate::options::{InvalidHeaderCause, InvalidHeaderError, MemoryPointerLength, ProgramOptions};

/// The header length of the current compile feature version.
///
/// See [options::CCFV].
pub const CCFV_HEADER_LEN: usize = CFV1_HEADER_LEN;

/// The header length of compile feature version 1.
///
/// Constant value `13`.
pub const CFV1_HEADER_LEN: usize = 13;

pub struct Adapt {
    pub header_len: usize,
    pub function_start: u64,
    pub options: ProgramOptions,
}

impl Adapt {
    #[inline]
    pub const fn new(header_len: usize, function_start: u64, options: ProgramOptions) -> Self {
        Self {
            header_len,
            function_start,
            options,
        }
    }
}

/// Get the header size of the given compile feature version.
///
/// Returns `None` if the compile feature version is not recognized.
#[inline]
pub fn get_header_size(cfv: u32) -> Option<usize> {
    match cfv {
        options::CCFV => Some(CCFV_HEADER_LEN),
        _ => None,
    }
}

pub type AdapterResult = Result<Adapt, InvalidHeaderError>;

/// Try to retrieve an [Adapt] using the bytecode header format defined for CFV 1.
pub fn try_retrieve_cfv1(bytes: &[u8]) -> AdapterResult {
    if bytes.len() < CFV1_HEADER_LEN {
        return Err(InvalidHeaderError::from(
            InvalidHeaderCause::FormatNotFulfilled,
            "header input too short",
        ));
    }

    let cfv = u32::from_le_bytes(bytes[..4].try_into().unwrap());

    let mem_ptr_len = match MemoryPointerLength::from_byte_identifier(bytes[4]) {
        Some(mpl) => mpl,
        None => {
            return Err(InvalidHeaderError::from(
                InvalidHeaderCause::UnrecognizedValue,
                "unrecognized memory pointer length",
            ));
        }
    };

    let execution_start = u64::from_le_bytes(bytes[5..13].try_into().unwrap());

    Ok(Adapt::new(
        CFV1_HEADER_LEN,
        execution_start,
        ProgramOptions::new(cfv, mem_ptr_len),
    ))
}

/// This function will always support backwards compatibility, but forward compatibility is not
/// guaranteed.
///
/// Returns a tuple containing the [ProgramOptions] and the length in bytes of the header
/// that was read.
///
/// See the [crate::options::header_format_doc] module for full documentation regarding the official
/// bytecode header.
#[inline(always)]
pub fn get_program_options(bytes: &[u8]) -> AdapterResult {
    try_retrieve_cfv1(bytes)
}
