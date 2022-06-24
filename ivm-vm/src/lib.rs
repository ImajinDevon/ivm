use ivm_common::Instruction;

// We will have to just clone for now..TODO will restructure
#[derive(Clone)]
pub enum OwnedData {
    Bytes(Vec<u8>),
}

pub trait Function {
    /// Call this function.
    /// Returns `None` as `void`.
    fn call(&mut self, args: Vec<OwnedData>) -> Option<OwnedData>;
}

pub struct IntrinsicFunction {
    func: fn(Vec<OwnedData>) -> Option<OwnedData>,
}

impl IntrinsicFunction {
    pub fn new(func: fn(Vec<OwnedData>) -> Option<OwnedData>) -> Self {
        Self { func }
    }
}

impl Function for IntrinsicFunction {
    fn call(&mut self, args: Vec<OwnedData>) -> Option<OwnedData> {
        (self.func)(args)
    }
}

pub struct Environment {
    static_ptrs: Box<[OwnedData]>,
    static_functions: Box<[Box<dyn Function>]>,
}

impl Environment {
    pub fn call_static_function(
        &mut self,
        function_ptr_index: u32,
        args: Vec<OwnedData>,
    ) -> Option<OwnedData> {
        self.static_functions[function_ptr_index as usize].call(args)
    }

    pub fn mutate_static_ptr(&mut self, ptr_index: u32, value: OwnedData) {
        self.static_ptrs[ptr_index as usize] = value;
    }

    pub fn get_static_ptr(&mut self, ptr_index: u32) -> OwnedData {
        self.static_ptrs[ptr_index as usize].clone()
    }

    pub fn from_raw(
        static_ptrs: Box<[OwnedData]>,
        static_functions: Box<[Box<dyn Function>]>,
    ) -> Self {
        Self {
            static_ptrs,
            static_functions,
        }
    }
}

pub struct EnvironmentBuilder {
    static_ptrs: Vec<OwnedData>,
    static_functions: Vec<Box<dyn Function>>,
}

impl EnvironmentBuilder {
    pub fn set_intrinsic_fn(
        &mut self,
        function_ptr_index: u32,
        fun: fn(Vec<OwnedData>) -> Option<OwnedData>,
    ) {
        self.set_intrinsic_function(function_ptr_index, IntrinsicFunction::new(fun))
    }

    /// Utility method for boxing an intrinsic function.
    pub fn set_intrinsic_function(&mut self, function_ptr_index: u32, fun: IntrinsicFunction) {
        self.set_static_function(function_ptr_index, Box::new(fun))
    }

    pub fn set_static_ptr(&mut self, ptr_index: u32, ptr: OwnedData) {
        self.static_ptrs.insert(ptr_index as usize, ptr);
    }

    pub fn set_static_function(&mut self, function_ptr_index: u32, ptr: Box<dyn Function>) {
        self.static_functions
            .insert(function_ptr_index as usize, ptr);
    }

    pub fn into_environment(self) -> Environment {
        Environment::from_raw(
            self.static_ptrs.into_boxed_slice(),
            self.static_functions.into_boxed_slice(),
        )
    }

    pub fn new() -> Self {
        Self {
            static_ptrs: Vec::with_capacity(5),
            static_functions: Vec::with_capacity(5),
        }
    }
}

/// Options that the VM will use.
///
/// # Performance Tips
/// **Note**: These tips may increase performance in smaller environments.
///
/// - Use a lower initial stack size - Decreases memory usage by a small amount but may require more
/// allocations, decreasing performance over time
///
/// More options will be added as the VM adds more features.
#[derive(Clone, Debug)]
pub struct VmOptions {
    initial_stack_size: usize,
}

impl VmOptions {
    pub fn get_initial_stack_size(&self) -> usize {
        self.initial_stack_size
    }

    pub fn set_initial_stack_size(&mut self, initial_stack_size: usize) {
        self.initial_stack_size = initial_stack_size;
    }
}

impl Default for VmOptions {
    /// Create the default VmOptions.
    ///
    /// This uses an initial stack size of 5.
    fn default() -> Self {
        Self {
            initial_stack_size: 5,
        }
    }
}

pub struct VmOptionsBuilder {
    inner: VmOptions,
}

impl VmOptionsBuilder {
    pub fn into_inner(self) -> VmOptions {
        self.inner
    }

    pub fn with_initial_stack_size(
        mut self,
        default_initial_stack_size: usize,
    ) -> VmOptionsBuilder {
        self.inner
            .set_initial_stack_size(default_initial_stack_size);
        self
    }

    pub fn new() -> Self {
        Self {
            inner: VmOptions::default(),
        }
    }
}

pub struct VmInstance {
    options: VmOptions,
    env: Environment,
}

impl VmInstance {
    pub fn get_options(&self) -> &VmOptions {
        &self.options
    }

    pub fn set_options(&mut self, options: VmOptions) {
        self.options = options;
    }

    /// Execute the given instructions, and get the final stack.
    pub fn execute_instructions<I>(&mut self, instructions: I) -> Vec<OwnedData>
    where
        I: IntoIterator<Item = Instruction>,
    {
        let init_stack = || Vec::with_capacity(self.options.initial_stack_size);
        let mut stack = init_stack();

        for instruction in instructions {
            match instruction {
                Instruction::PushBytes(v) => stack.push(OwnedData::Bytes(v)),

                Instruction::InvokeStatic(ptr_index) => {
                    let return_value = self.env.call_static_function(ptr_index, stack);
                    stack = init_stack();

                    if let Some(data) = return_value {
                        stack.push(data);
                    }
                }

                Instruction::StaticVarLoad(ptr_index) => {
                    stack.push(self.env.get_static_ptr(ptr_index)) // clones the existing variable,
                                                                   // TODO will restructure
                }

                Instruction::StaticVarStore(ptr_index) => {
                    self.env.mutate_static_ptr(ptr_index, stack.pop().unwrap())
                }
            }
        }
        stack
    }

    pub fn new(options: VmOptions, env: Environment) -> Self {
        Self { options, env }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;

    use ivm_common::Instruction;

    use crate::{Environment, EnvironmentBuilder, OwnedData, VmInstance, VmOptionsBuilder};

    fn intrinsic_println_function(mut args: Vec<OwnedData>) -> Option<OwnedData> {
        let data = args.pop().unwrap();

        let bytes = if let OwnedData::Bytes(b) = data {
            b
        } else {
            panic!()
        };

        io::stdout().write_all(&bytes).unwrap();
        io::stdout().write_all(b"\n").unwrap();
        io::stdout().flush().unwrap();

        None
    }

    fn generate_intrinsic_environment() -> Environment {
        let mut env = EnvironmentBuilder::new();
        env.set_intrinsic_fn(0, intrinsic_println_function);
        env.into_environment()
    }

    #[test]
    fn raw_println_test() {
        let env = generate_intrinsic_environment();

        let options = VmOptionsBuilder::new()
            .with_initial_stack_size(1)
            .into_inner();

        let mut vm = VmInstance::new(options, env);

        let instructions = [
            Instruction::PushBytes(b"Hello, world!".to_vec()),
            Instruction::InvokeStatic(0), // as intrinsic `println` is defined at ptr_index 0
        ];

        vm.execute_instructions(instructions);
    }
}
