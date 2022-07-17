use std::io;
use std::io::Write;

use crate::{ExecutionContext, ExternMap, VmInstance};

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

/// Extern call id `2`.
///
/// Sets the VM's execution index to the length of the memory pool.\
/// This will cause the VM to return out of the [crate::VmInstance::continue_execution()] function.
///
/// This does not prevent the container of the VM from continuing execution.
pub const EXTC_JUMP_OVERFLOW: usize = 2;

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
#[inline]
pub fn write_register(reg_index: usize, data: &[u8], mem_pool: &mut [u8]) {
    mem_pool[reg_index..(data.len() + reg_index)].copy_from_slice(data)
}

/// Match the given result, then write the error code to the [REG_ERROR] register in the given
/// memory pool.
///
/// If the Result is [Ok], this function writes `0i32`.
pub fn write_io_err_register<T>(
    ctx: &mut ExecutionContext,
    mem_pool: &mut [u8],
    result: io::Result<T>,
) {
    match result {
        Ok(_) => {
            if !ctx.ext_1 {
                write_register(REG_ERROR, &0i32.to_le_bytes(), mem_pool);
                ctx.ext_1 = true;
            }
        }
        Err(err) => {
            ctx.ext_1 = false;

            write_register(
                REG_ERROR,
                &err.raw_os_error().unwrap_or(-1).to_le_bytes(),
                mem_pool,
            );
        }
    }
}

/// The `ivm_ext_x32` extern map.
///
/// This extern map may rely on memory registers defined in [this module](self).
pub struct IvmX32ExternMap;

impl ExternMap for IvmX32ExternMap {
    fn handle(&mut self, ctx: &mut ExecutionContext, call_id: usize, vm: &mut VmInstance) {
        match call_id {
            EXTC_STDOUT_WRITE => {
                let data = ctx.ext_a_slice();
                let res = io::stdout().write_all(data);
                write_io_err_register(ctx, &mut vm.mem_pool, res);
            }

            EXTC_STDOUT_FLUSH => write_io_err_register(ctx, &mut vm.mem_pool, io::stdout().flush()),

            EXTC_JUMP_OVERFLOW => vm.execution_index = vm.mem_pool.len(),

            _ => panic!("unrecognized ivm_x32 external '{call_id}'"),
        }
    }
}
