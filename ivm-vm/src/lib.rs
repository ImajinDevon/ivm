use std::cell::RefCell;

use ivm_common::Instruction;

pub mod data;

/// A pointer to data.
#[derive(Clone)]
pub enum Pointer {
    Static(u32),

    /// A pointer to nothing.
    Null,

    Owned(RefCell<data::OwnedData>),
}

impl Pointer {
    /// Unwrap the [data::OwnedData] inside of this enum.
    /// Panics if this pointer does not match [Pointer::Owned].
    pub fn unwrap_data(self) -> RefCell<data::OwnedData> {
        match self {
            Pointer::Owned(owned) => owned,
            _ => panic!("Expected owned pointer"),
        }
    }
}

impl Unpin for data::OwnedData {}

pub trait Function {
    fn call(&mut self, args: &mut Vec<Pointer>) -> Pointer;
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

pub struct Environment {
    static_pointers: Vec<Pointer>,
    static_functions: Vec<Box<dyn Function>>,
}

impl Environment {
    pub fn insert_static_function(&mut self, index: u32, value: Box<dyn Function>) {
        self.static_functions.insert(index as usize, value);
    }

    pub fn insert_static_ptr(&mut self, index: u32, value: Pointer) {
        self.static_pointers.insert(index as usize, value);
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            static_pointers: Vec::with_capacity(initial_capacity),
            static_functions: Vec::with_capacity(initial_capacity),
        }
    }
}

pub fn execute_instructions(mut env: Environment, instructions: &[Instruction]) {
    let mut stack = Vec::with_capacity(5);
    let mut pos = 0;

    while pos < instructions.len() {
        let instruction = &instructions[pos];

        match instruction {
            Instruction::InvokeStatic(ptr_index) => {
                let function = &mut env.static_functions[*ptr_index as usize];
                function.call(&mut stack);
                stack.clear();
            }

            Instruction::PushBytes(raw) => {
                stack.push(Pointer::Owned(RefCell::new(data::OwnedData::Bytes(raw.clone()))));
            }

            Instruction::StaticVarStore(ptr_index) => {
                debug_assert_eq!(stack.len(), 1);
                env.insert_static_ptr(*ptr_index, stack.pop().unwrap());
            }

            Instruction::StaticVarLoad(ptr_index) => {
                stack.push(env.static_pointers[*ptr_index as usize].clone());
            }
        }
        pos += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use ivm_common::Instruction;

    use crate::{data, Environment, Function, IntrinsicFunction, Pointer};

    fn box_intrinsic(f: fn(&mut Vec<Pointer>) -> Pointer) -> Box<dyn Function> {
        Box::new(IntrinsicFunction::new(f))
    }

    fn intrinsic_fn_println(args: &mut Vec<Pointer>) -> Pointer {
        let pointer = args.pop().unwrap();
        let mut refc = pointer.unwrap_data();

        let data = refc.get_mut();
        let bytes = if let data::OwnedData::Bytes(bytes) = data { bytes } else { panic!() };

        io::stdout().write_all(bytes).expect("cannot write to stdout");
        io::stdout().write_all(b"\n").expect("cannot write newline to stdout");
        io::stdout().flush().expect("cannot flush stdout");

        Pointer::Null
    }

    fn generate_intrinsic_environment() -> Environment {
        let mut env = Environment::with_capacity(1);
        env.insert_static_function(0, box_intrinsic(intrinsic_fn_println));
        env
    }

    #[test]
    fn println_test() {
        let env = generate_intrinsic_environment();
        let instructions = vec![
            Instruction::PushBytes(b"Hello, world!".to_vec()),
            Instruction::InvokeStatic(0) // as intrinsic `println` is defined at ptr_index 0
        ];
        crate::execute_instructions(env, &instructions);
    }
}
