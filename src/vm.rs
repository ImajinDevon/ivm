use crate::Instruction;

#[derive(Clone)]
pub enum Owned {
    Bytes(Vec<u8>),
    Object(Vec<Pointer>),
}

#[derive(Clone)]
pub enum Pointer {
    Static(u32),
    /// A pointer to nothing.
    Null,
    Owned(Owned),
}

impl Pointer {
    pub fn get_owned(self) -> Owned {
        match self {
            Pointer::Owned(owned) => owned,
            _ => panic!("Expected owned pointer"),
        }
    }
}

pub trait Function {
    fn call<'a>(&mut self, args: &mut Vec<Pointer>) -> Pointer;
}

pub struct IntrinsicFunction {
    func: fn(&mut Vec<Pointer>) -> Pointer,
}

impl IntrinsicFunction {
    pub fn new(func: fn(&mut Vec<Pointer>) -> Pointer) -> Self {
        Self { func }
    }
}

impl Function for IntrinsicFunction {
    fn call(&mut self, args: &mut Vec<Pointer>) -> Pointer {
        (self.func)(args)
    }
}

pub mod intrinsics {
    use std::io;
    use std::io::Write;

    use crate::vm::{Environment, Function, IntrinsicFunction, Owned, Pointer};

    pub const STDOUT_WRITE: u32 = 0;
    pub const STDOUT_FLUSH: u32 = 1;
    pub const INTRINSIC_COUNT: u32 = 2;

    fn intrinsic_stdout_write(args: &mut Vec<Pointer>) -> Pointer {
        debug_assert_eq!(args.len(), 1);

        let mut bytes = match args.pop().unwrap() {
            Pointer::Owned(Owned::Bytes(bytes)) => bytes,
            _ => panic!("Expected bytes to print"),
        };
        io::stdout()
            .write_all(&mut bytes)
            .expect("Failed to write to stdout");
        Pointer::Null
    }

    fn intrinsic_stdout_flush(_args: &mut Vec<Pointer>) -> Pointer {
        debug_assert!(_args.is_empty());
        io::stdout().flush().expect("Failed to flush stdout");
        Pointer::Null
    }

    pub fn create_intrinsic_environment() -> Environment {
        let fnbox = |f: fn(&mut Vec<Pointer>) -> Pointer| {
            let b: Box<dyn Function> = Box::new(IntrinsicFunction::new(f));
            b
        };

        let mut env = Environment::with_capacity(INTRINSIC_COUNT as usize);

        env.insert_static_function(STDOUT_WRITE, fnbox(intrinsic_stdout_write));
        env.insert_static_function(STDOUT_FLUSH, fnbox(intrinsic_stdout_flush));
        env
    }
}

pub struct Environment {
    statics: Vec<Pointer>,
    static_functions: Vec<Box<dyn Function>>,
}

impl Environment {
    pub fn insert_static_function(&mut self, index: u32, value: Box<dyn Function>) {
        self.static_functions.insert(index as usize, value);
    }

    pub fn insert_static_ptr(&mut self, index: u32, value: Pointer) {
        self.statics.insert(index as usize, value);
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            statics: Vec::with_capacity(initial_capacity),
            static_functions: Vec::with_capacity(initial_capacity),
        }
    }
}

pub fn execute_instructions(mut env: Environment, instructions: &[Instruction]) {
    let mut stack = Vec::new();
    let mut pos = 0;

    while pos < instructions.len() {
        let instruction = &instructions[pos];

        match instruction {
            Instruction::Invoke(ptr_index) => {
                let function = &mut env.static_functions[*ptr_index as usize];
                function.call(&mut stack);
                stack.clear();
            }

            Instruction::PushBytes(raw) => {
                stack.push(Pointer::Owned(Owned::Bytes(raw.clone())));
            }

            Instruction::StaticVarStore(ptr_index) => {
                debug_assert_eq!(stack.len(), 1);
                env.insert_static_ptr(*ptr_index, stack.pop().unwrap());
            }

            Instruction::StaticVarLoad(ptr_index) => {
                stack.push(env.statics[*ptr_index as usize].clone());
            }
        }
        pos += 1;
    }
}
