use ivm_compile::byte_id;
use ivm_compile::options::ProgramOptions;

use crate::ivm_ext_x32::IvmX32ExternMap;

pub mod ivm_ext_x32;

// Wrapper for now.
pub struct StackElement {
    pub bytes: Vec<u8>,
}

impl StackElement {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

pub trait ExternMap {
    fn handle(&mut self, call_id: usize, stack: &mut Vec<StackElement>) -> Result<(), String>;
}

pub struct VmInstance {
    pub options: ProgramOptions,
    pub mem_pool: Vec<u8>,
    pub execution_index: usize,
    pub extern_map: Box<dyn ExternMap>,
    pub stack: Vec<StackElement>,
}

impl VmInstance {
    /// Introduce new bytes to the memory pool.
    pub fn introduce<I>(&mut self, bytes: I)
    where
        I: IntoIterator<Item = u8>,
    {
        self.mem_pool.extend(bytes);
    }

    /// Returns a tuple containing the read bytes, and how many bytes were traversed.
    fn handle_read_op(&self, mut index: usize) -> (Vec<u8>, usize) {
        let identifier = self.mem_pool[index];
        index += 1;

        let arg = self.options.ptr_len.extract(index, &self.mem_pool);
        index += self.options.ptr_len.get_span();

        match identifier {
            byte_id::RDOP_LOCAL => {
                // `arg` is the size of the read operation.

                let data = &self.mem_pool[index..][..arg];
                (data.to_vec(), 1 + self.options.ptr_len.get_span() + arg)
            }

            byte_id::RDOP_POINT => {
                // `arg` is the memory pointer index of the data we are pointing to.

                let rd_size = self.options.ptr_len.extract(arg, &self.mem_pool);
                let data = &self.mem_pool[arg..][..rd_size];

                (data.to_vec(), 1 + self.options.ptr_len.get_span())
            }
            _ => panic!("unrecognized read operation '{identifier:02x}'"),
        }
    }

    pub fn continue_execution(&mut self) -> Result<(), String> {
        while self.execution_index < self.mem_pool.len() {
            let byte_instruction = self.mem_pool[self.execution_index];
            self.execution_index += 1;

            match byte_instruction {
                byte_id::I_VISIT => {
                    self.execution_index = self
                        .options
                        .ptr_len
                        .extract(self.execution_index, &self.mem_pool)
                }

                byte_id::I_MUTATE => {
                    let dest = self
                        .options
                        .ptr_len
                        .extract(self.execution_index, &self.mem_pool);

                    self.execution_index += self.options.ptr_len.get_span();

                    let (data, skip) = self.handle_read_op(self.execution_index);
                    self.execution_index += skip;

                    for i in 0..data.len() {
                        self.mem_pool[dest + i] = data[i];
                    }
                }

                byte_id::I_PUSH => {
                    let (data, skip) = self.handle_read_op(self.execution_index);

                    self.stack.push(StackElement::new(data.to_vec()));
                    self.execution_index += skip;
                }

                byte_id::I_EXTERN_CALL => {
                    let call_id = self
                        .options
                        .ptr_len
                        .extract(self.execution_index, &self.mem_pool);

                    self.extern_map.handle(call_id, &mut self.stack)?;
                    self.execution_index += self.options.ptr_len.get_span();
                }

                _ => {
                    return Err(format!(
                        "unrecognized instruction (hex): {byte_instruction:02x}"
                    ))
                }
            }
        }
        Ok(())
    }

    pub fn from_raw(
        program_options: ProgramOptions,
        mem_pool: Vec<u8>,
        ptr_index: usize,
        extern_map: Box<dyn ExternMap>,
    ) -> Self {
        Self {
            options: program_options,
            mem_pool,
            execution_index: ptr_index,
            extern_map,
            stack: Vec::new(),
        }
    }

    /// Uses the latest ivm_ext_x32 extern map.
    ///
    /// See [ivm_ext_x32], [IvmX32ExternMap].
    pub fn init(program_options: ProgramOptions) -> Self {
        Self::from_raw(
            program_options,
            Vec::with_capacity(128),
            0,
            Box::new(IvmX32ExternMap),
        )
    }
}
