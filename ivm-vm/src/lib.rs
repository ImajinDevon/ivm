#![feature(slice_ptr_len)]
#![feature(const_slice_from_raw_parts)]

use ivm_compile::byte_id;
use ivm_compile::options::{MemoryPointerLength, ProgramOptions};

pub mod ivm_ext_x32;
pub mod security;

pub trait ExternMap {
    fn handle(&mut self, ctx: &mut ExecutionContext, call_id: usize, vm: &mut VmInstance);
}

pub struct EmptyExternMap;

impl ExternMap for EmptyExternMap {
    fn handle(&mut self, _ctx: &mut ExecutionContext, call_id: usize, _vm: &mut VmInstance) {
        panic!(
            "call ({call_id}) to an EmptyExternMap [ivm-vm/lib.rs @line {}]",
            line!()
        );
    }
}

#[inline(always)]
const fn empty_slice() -> *const [u8] {
    std::ptr::slice_from_raw_parts(std::ptr::null(), 0)
}

pub struct ExecutionContext {
    pub ext_a: *const [u8], // The ExternMap MUST provide safety checks!
    pub ext_1: bool,
    // ^ The IvmExtX32 extern map will rely on this to quickly decide whether to write to the error
    // | register.
}

impl ExecutionContext {
    #[track_caller]
    pub fn ext_a_slice(&mut self) -> &[u8] {
        if self.ext_a.len() == 0 {
            panic!("ext_a is null ~ prevented dereference of nullptr (VM#00001)");
        }
        unsafe { &*self.ext_a }
    }

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            ext_a: empty_slice(),
            ext_1: true,
        }
    }
}

pub struct ExecutionEnvironment<'a> {
    extern_map: &'a mut dyn ExternMap,
    pub ctx: ExecutionContext,
}

impl<'a> ExecutionEnvironment<'a> {
    #[inline(always)]
    pub fn call_extern(&mut self, call_id: usize, vm: &mut VmInstance) {
        self.extern_map.handle(&mut self.ctx, call_id, vm);
    }

    /// Create a new ExecutionEnvironment.
    #[inline(always)]
    pub fn new(extern_map: &'a mut dyn ExternMap) -> Self {
        Self {
            extern_map,
            ctx: ExecutionContext::new(),
        }
    }
}

pub type Stack = Vec<*const [u8]>;

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
/// let bytecode = ivm_compile::compile_all([
///     Instruction::Push(ReadOperation::Local(b"Hello, world!".to_vec())),
///     Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE)
/// ], &program_options);
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

    /// Returns a tuple containing the read bytes, the length of the pointer, and how many bytes
    /// were traversed.
    fn handle_read_op(&self, mut index: usize) -> (*const [u8], usize) {
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

        let data = &self.mem_pool[location..][..read_size] as *const [u8];
        (data, skip)
    }

    #[inline]
    fn handle_read_op_skip(&mut self) -> *const [u8] {
        let (data, skip) = self.handle_read_op(self.execution_index);
        self.execution_index += skip;
        data
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
    #[inline]
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
                    let data = self.handle_read_op_skip();

                    unsafe {
                        self.mem_pool[dest..][..data.len()].copy_from_slice(&*data);
                    }
                }

                byte_id::I_PUSH => {
                    let data = self.handle_read_op_skip();
                    self.stack.push(data);
                }

                byte_id::I_EXTERN_CALL => {
                    let ptr = self.extract_ptr_skip();
                    env.call_extern(ptr, self);
                }

                byte_id::I_CALL => {
                    let ptr = self.extract_ptr_skip();
                    self.call_stack.push(self.execution_index);
                    self.execution_index = ptr;
                }

                byte_id::I_RETURN => match self.call_stack.pop() {
                    Some(caller) => self.execution_index = caller,
                    None => return,
                },

                byte_id::I_LOAD_A => {
                    let data = self.handle_read_op_skip();
                    env.ctx.ext_a = data;
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
    #[inline]
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
    /// The VM will reserve the amount of bytes declared at
    /// [ivm_ext_x32::REGISTER_RESERVED], then fill the memory pool with null (0x00) bytes.
    #[inline]
    pub fn reserve_ivm_ext_x32(program_options: ProgramOptions) -> Self {
        let initial_mem_pool = vec![0; ivm_ext_x32::REGISTER_RESERVED];

        Self::new(
            program_options,
            initial_mem_pool,
            ivm_ext_x32::REGISTER_RESERVED,
        )
    }

    /// Create an empty VmInstance.
    ///
    /// This VmInstance will use a [ProgramOptions] with a CFV of 0.
    ///
    /// It is recommended to only use this function for testing purposes.
    #[inline]
    pub fn mock() -> Self {
        Self::new(
            ProgramOptions::new(0, MemoryPointerLength::X32b),
            Vec::new(),
            0,
        )
    }
}
