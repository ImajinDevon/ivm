use std::io;
use std::io::Write;

use crate::{ExternMap, StackElement};

pub const STDOUT_WRITE: usize = 0;
pub const STDOUT_FLUSH: usize = 1;

pub struct IvmX32ExternMap;

impl ExternMap for IvmX32ExternMap {
    fn handle(&mut self, call_id: usize, stack: &mut Vec<StackElement>) -> Result<(), String> {
        match call_id {
            STDOUT_WRITE => {
                let element = stack
                    .pop()
                    .ok_or_else(|| "call with empty stack".to_string())?;

                io::stdout()
                    .write_all(&element.bytes)
                    .map_err(|err| format!("could not write to stdout: {err}"))
            }
            STDOUT_FLUSH => io::stdout()
                .flush()
                .map_err(|err| format!("could not flush stdout: {err}")),
            _ => return Err(format!("unrecognized ivm_x32 external '{}'", call_id)),
        }
    }
}
