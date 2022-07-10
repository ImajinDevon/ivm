use ivm_compile::byte_id;
use ivm_compile::options::ProgramOptions;

use crate::ivm_ext_x32::IvmX32ExternMap;

pub mod ivm_ext_x32;

pub struct StackElement {
    pub bytes: Vec<u8>,
}

impl StackElement {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

pub trait ExternMap {
    fn handle(&mut self, call_id: usize, mem_pool: &mut Vec<u8>, stack: &mut Vec<StackElement>);
}

#[cfg(test)]
pub struct EmptyExternMap;

pub struct VmInstance {
    pub options: ProgramOptions,
    pub mem_pool: Vec<u8>,
    pub execution_index: usize,
    pub extern_map: Box<dyn ExternMap>,
    pub stack: Vec<StackElement>,
    pub call_stack: Vec<usize>,
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
        let span = self.options.ptr_len.get_span();
        let identifier = self.mem_pool[index];

        index += 1;

        let read_size = self._extract_ptr(index);
        let mut skip = 1 + read_size;

        let location = match identifier {
            byte_id::RDOP_LOCAL => {
                skip += span;
                index + span
            }

            byte_id::RDOP_POINT => {
                skip += span << 1;
                self._extract_ptr(index)
            }

            _ => panic!("unrecognized read operation '{identifier:02x}'"),
        };

        let data = &self.mem_pool[location..][..read_size];
        (data.to_vec(), skip)
    }

    /// Extract a pointer at the given index.
    fn _extract_ptr(&self, index: usize) -> usize {
        self.options.ptr_len.extract(index, &self.mem_pool)
    }

    /// Extract a pointer at the current execution index.
    fn extract_ptr(&self) -> usize {
        self._extract_ptr(self.execution_index)
    }

    /// Extract a pointer at the current execution index, then skip the required amount of bytes.
    ///
    /// See [ivm_compile::options::MemoryPointerLength::get_span()].
    fn extract_ptr_skip(&mut self) -> usize {
        let ptr = self.extract_ptr();
        self.execution_index += self.options.ptr_len.get_span();
        ptr
    }

    /// Starts or resumes execution at the current execution index.
    pub fn continue_execution(&mut self) {
        while self.execution_index < self.mem_pool.len() {
            let byte_instruction = self.mem_pool[self.execution_index];

            self.execution_index += 1;

            match byte_instruction {
                byte_id::I_JUMP => self.execution_index = self.extract_ptr(),

                byte_id::I_MUTATE => {
                    let dest = self.extract_ptr_skip();

                    let (data, skip) = self.handle_read_op(self.execution_index);
                    self.execution_index += skip;

                    self.mem_pool[dest..(data.len() + dest)].copy_from_slice(&data[..]);
                }

                byte_id::I_PUSH => {
                    let (data, skip) = self.handle_read_op(self.execution_index);

                    self.stack.push(StackElement::new(data));
                    self.execution_index += skip;
                }

                byte_id::I_EXTERN_CALL => {
                    let ptr = self.extract_ptr_skip();

                    self.extern_map
                        .handle(ptr, &mut self.mem_pool, &mut self.stack)
                }

                byte_id::I_RETURN => match self.call_stack.pop() {
                    Some(caller) => self.execution_index = caller,
                    None => return,
                },

                byte_id::I_CALL => {
                    let ptr = self.extract_ptr_skip();
                    self.call_stack.push(self.execution_index);
                    self.execution_index = ptr;
                }

                _ => panic!(
                    "unrecognized instruction (hex): {byte_instruction:02x} at execution index {}",
                    self.execution_index - 1
                ),
            }
        }
    }

    /// Create a new VmInstance.
    ///
    /// If you want to use the `ivm_ext_x32` extern map, use
    /// [Self::with_ivm_ext_x32(ProgramOptions)].
    pub fn new(
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
            call_stack: Vec::new(),
        }
    }

    /// Create a new VmInstance using the provided program options.
    /// The *[IvmX32ExternMap]* is used.
    ///
    /// The VM will allocate enough room to fit the amount of bytes declared at
    /// [ivm_ext_x32::REGISTER_RESERVED].
    pub fn with_ivm_ext_x32(program_options: ProgramOptions) -> Self {
        let initial_mem_pool = vec![0; ivm_ext_x32::REGISTER_RESERVED];

        Self::new(
            program_options,
            initial_mem_pool,
            ivm_ext_x32::REGISTER_RESERVED,
            Box::from(IvmX32ExternMap),
        )
    }
}
