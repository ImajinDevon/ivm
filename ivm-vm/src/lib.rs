use ivm_compile::byte_id;
use ivm_compile::options::ProgramOptions;

pub mod ivm_ext_x32;
pub mod security;

pub trait ExternMap {
    fn handle(&mut self, call_id: usize, vm: &mut VmInstance);
}

pub struct ExecutionEnvironment<'a> {
    extern_map: &'a mut dyn ExternMap,
}

impl<'a> ExecutionEnvironment<'a> {
    /// Create a new ExecutionEnvironment.
    pub fn new(extern_map: &'a mut dyn ExternMap) -> Self {
        Self { extern_map }
    }

    /// Get the contained [ExternMap].
    pub fn get_extern_map(&mut self) -> &mut dyn ExternMap {
        self.extern_map
    }
}

pub struct EmptyExternMap;

impl ExternMap for EmptyExternMap {
    fn handle(&mut self, call_id: usize, _vm: &mut VmInstance) {
        panic!(
            "call ({call_id}) to an EmptyExternMap [ivm-vm/lib.rs @line {}]",
            line!()
        );
    }
}

pub type Stack = Vec<Vec<u8>>;

/// An instance of the ivm VM.
///
/// See the [wiki](https://github.com/imajindevon/ivm/wiki) for a full guide on getting started with
/// ivm.
///
/// # Examples
/// ```
/// use ivm_compile::{Instruction, ReadOperation};
/// use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
/// use ivm_vm::ivm_ext_x32::IvmX32ExternMap;
/// use ivm_vm::{ExecutionEnvironment, ivm_ext_x32, VmInstance};
///
/// let program_options = ProgramOptions::new(1, MemoryPointerLength::X32b);
///
/// let bytecode = ivm_compile::compile_all(&program_options, [
///     Instruction::Push(ReadOperation::Local(b"Hello, world!".to_vec())),
///     Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE)
/// ]);
///
/// let mut extern_map = IvmX32ExternMap;
/// let mut env = ExecutionEnvironment::new(&mut extern_map);
/// let mut vm = VmInstance::reserve_ivm_ext_x32(program_options);
///
/// vm.introduce(bytecode);
/// vm.continue_execution(&mut env);
/// ```
pub struct VmInstance {
    pub options: ProgramOptions,
    pub mem_pool: Vec<u8>,
    pub execution_index: usize,
    pub stack: Stack,
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
        let span = self.options.ptr_len().get_span();
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
    #[inline]
    fn _extract_ptr(&self, index: usize) -> usize {
        self.options.ptr_len().extract(index, &self.mem_pool)
    }

    /// Extract a pointer at the current execution index.
    #[inline]
    fn extract_ptr(&self) -> usize {
        self._extract_ptr(self.execution_index)
    }

    /// Extract a pointer at the current execution index, then skip the required amount of bytes.
    ///
    /// The required amount of bytes is defined in
    /// [ivm_compile::options::MemoryPointerLength::get_span()], and will vary depending on the
    /// [ProgramOptions] contained within this VmInstance.
    fn extract_ptr_skip(&mut self) -> usize {
        let ptr = self.extract_ptr();
        self.execution_index += self.options.ptr_len().get_span();
        ptr
    }

    /// Starts or resumes execution at the current execution index.
    ///
    /// If the execution index is greater than the length of the memory pool, this function will
    /// return immediately.
    ///
    /// # Examples
    /// ```
    /// use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
    /// use ivm_vm::ivm_ext_x32::IvmX32ExternMap;
    /// use ivm_vm::{EmptyExternMap, ExecutionEnvironment, VmInstance};
    ///
    /// let mut extern_map = IvmX32ExternMap;
    /// let mut env = ExecutionEnvironment::new(&mut extern_map);
    ///
    /// let mut vm = VmInstance::new(
    ///     ProgramOptions::new(1, MemoryPointerLength::X32b),
    ///     Vec::new(), // Memory pool of length 0
    ///     1 // Execution index of 1
    /// );
    ///
    /// // Nothing will be happen.
    /// vm.continue_execution(&mut env);
    /// ```
    pub fn continue_execution(&mut self, env: &mut ExecutionEnvironment) {
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

                    self.stack.push(data);
                    self.execution_index += skip;
                }

                byte_id::I_EXTERN_CALL => {
                    let ptr = self.extract_ptr_skip();
                    env.get_extern_map().handle(ptr, self);
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
    /// If you want to use the `ivm_ext_x32` extern map, you may want to use
    /// [Self::reserve_ivm_ext_x32(ProgramOptions)].
    pub fn new(program_options: ProgramOptions, mem_pool: Vec<u8>, ptr_index: usize) -> Self {
        Self {
            options: program_options,
            mem_pool,
            execution_index: ptr_index,
            stack: Stack::with_capacity(3),
            call_stack: Vec::new(),
        }
    }

    /// Create a new VmInstance using the provided program options.
    ///
    /// The VM will allocate enough room to fit the amount of bytes declared at
    /// [ivm_ext_x32::REGISTER_RESERVED].
    pub fn reserve_ivm_ext_x32(program_options: ProgramOptions) -> Self {
        let initial_mem_pool = vec![0; ivm_ext_x32::REGISTER_RESERVED];

        Self::new(
            program_options,
            initial_mem_pool,
            ivm_ext_x32::REGISTER_RESERVED,
        )
    }

    /// Create a mock VmInstance.
    ///
    /// This method is only available in test builds.
    #[cfg(test)]
    pub fn mock() -> Self {
        use ivm_compile::options::MemoryPointerLength;

        Self::new(
            ProgramOptions::new(0, MemoryPointerLength::X32b),
            Vec::new(),
            0,
        )
    }
}
