//! A module containing adapters for differing compile feature versions.
//!
//! See [options::header_format_doc] for a full guide regarding the ivm bytecode format.

use crate::options;
use crate::options::{InvalidHeaderCause, InvalidHeaderError, MemoryPointerLength, ProgramOptions};

/// The header length of the current compile feature version.
/// See [options::CCFV].
pub const CCFV_HEADER_LEN: u32 = 5;

/// Get the header size of this compile feature version.
/// Returns `None` if the compile feature version is not recognized.
pub fn get_header_size(compile_feature_version: u32) -> Option<u32> {
    match compile_feature_version {
        options::CCFV => Some(CCFV_HEADER_LEN),
        _ => None,
    }
}

/// This function will always support backwards compatibility, but forward compatibility is not
/// guaranteed.
///
/// Be sure to read on [crate::options::header_format_doc] to fully understand the bytecode format.
///
/// Returns a tuple containing the [ProgramOptions] and the length in bytes of the header
/// that was read.
pub fn get_program_options(bytes: &[u8]) -> Result<(ProgramOptions, usize), InvalidHeaderError> {
    if bytes.len() < 5 {
        return Err(InvalidHeaderError::from(
            InvalidHeaderCause::FormatNotFulfilled,
            "header input too short",
        ));
    }

    // irrelevant as of CFV 1
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
    Ok((ProgramOptions::new(cfv, mem_ptr_len), 5))
}
