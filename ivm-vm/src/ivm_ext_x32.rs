use std::io;
use std::io::Write;

use crate::{ExternMap, StackElement};

/// Extern call id `0`.
///
/// Pops bytes from the stack, then writes them to stdout.
///
/// See [io::Stdout::write_all(&\[u8\])].
pub const EXTC_STDOUT_WRITE: usize = 0;

/// Extern call id `1`.
///
/// Flushes stdout.
///
/// See [io::Stdout::flush()].
pub const EXTC_STDOUT_FLUSH: usize = 1;

/// The error register.
///
/// Stores [i32] types, (4 bytes).
pub const REG_ERROR: usize = 0;

/// How many bytes the VM should reserve purely for registers.
///
/// Although this value will likely change in the future, this does not sacrifice
/// backwards-compatibility, because the VM does not interfere, let alone *acknowledge* custom
/// memory registries as such.
pub const REGISTER_RESERVED: usize = 4;

/// Copy the data into the memory pool at the given register index.
pub fn write_register(reg_index: usize, data: &[u8], mem_pool: &mut [u8]) {
    mem_pool[reg_index..(data.len() + reg_index)].copy_from_slice(data);
}

/// Match the given result, then write the error code to the [REG_ERROR] register in the given
/// memory pool.
///
/// If the Result is [Ok], this function writes `0i32`.
pub fn write_io_err_register<T>(mem_pool: &mut [u8], result: io::Result<T>) {
    write_register(
        REG_ERROR,
        &match result {
            Ok(_) => 0i32.to_le_bytes().to_vec(),
            Err(err) => err.raw_os_error().unwrap_or(-1).to_le_bytes().to_vec(),
        },
        mem_pool,
    )
}

/// The `ivm_ext_x32` extern map.
///
/// This extern map may rely on memory registers defined in [this module](self).
pub struct IvmX32ExternMap;

impl ExternMap for IvmX32ExternMap {
    fn handle(&mut self, call_id: usize, mem_pool: &mut Vec<u8>, stack: &mut Vec<StackElement>) {
        match call_id {
            EXTC_STDOUT_WRITE => {
                let element = stack
                    .pop()
                    .expect("call to ivm_ext_x32@STDOUT_WRITE with empty stack");

                write_io_err_register(mem_pool, io::stdout().write_all(&element.bytes));
            }

            EXTC_STDOUT_FLUSH => write_io_err_register(mem_pool, io::stdout().flush()),

            _ => panic!("unrecognized ivm_x32 external '{call_id}'"),
        }
    }
}
