use std::collections::HashMap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use ivm_compile::options::ProgramOptions;
use ivm_compile::Compile;

pub mod parse;

#[derive(Debug)]
pub enum ValueType {
    I128,
    I64,
    I32,
    I16,
    I8,
    U8,
    U16,
    U32,
    U64,
    U128,
    String,
}

impl FromStr for ValueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i128" => Self::I128,
            "i64" => Self::I64,
            "i32" => Self::I32,
            "i16" => Self::I16,
            "i8" => Self::I8,
            "u8" => Self::U8,
            "u16" => Self::U16,
            "u32" => Self::U32,
            "u64" => Self::U64,
            "u128" => Self::U128,
            "string" => Self::String,
            _ => return Err(()),
        })
    }
}

#[derive(Default, Debug)]
pub struct Function {
    params: HashMap<String, ValueType>,
}

impl Function {
    #[inline]
    pub fn insert_param(&mut self, name: String, value_type: ValueType) -> Option<ValueType> {
        self.params.insert(name, value_type)
    }

    #[inline]
    pub const fn new(params: HashMap<String, ValueType>) -> Self {
        Self { params }
    }
}

pub struct Error<'a> {
    message: &'a str,
    error: String,
}

impl<'a> Error<'a> {
    pub fn display(&self) {
        eprintln!("ivm-ir: {}", self.message);
        eprintln!("{}", self.error);
    }

    #[inline]
    pub fn from<D>(message: &'a str, error: D) -> Self
    where
        D: Display,
    {
        Self {
            message,
            error: error.to_string(),
        }
    }
}

#[derive(Default)]
pub struct CompileOptions {
    initial_reserve: usize,
}

impl CompileOptions {
    #[inline]
    pub const fn new(initial_reserve: usize) -> Self {
        Self { initial_reserve }
    }

    #[inline]
    pub const fn initial_reserve(&self) -> usize {
        self.initial_reserve
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    I128(i128),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    String(String),
}

impl Value {
    pub fn try_from(data: &str, type_bound: &ValueType) -> Result<Self, String> {
        fn inner(data: &str, type_bound: &ValueType) -> Result<Value, ParseIntError> {
            Ok(match type_bound {
                ValueType::I128 => Value::I128(data.parse()?),
                ValueType::I64 => Value::I64(data.parse()?),
                ValueType::I32 => Value::I64(data.parse()?),
                ValueType::I16 => Value::I64(data.parse()?),
                ValueType::I8 => Value::I64(data.parse()?),
                ValueType::U128 => Value::U64(data.parse()?),
                ValueType::U64 => Value::U64(data.parse()?),
                ValueType::U32 => Value::U32(data.parse()?),
                ValueType::U16 => Value::U16(data.parse()?),
                ValueType::U8 => Value::U8(data.parse()?),
                ValueType::String => Value::String(data.to_string()),
            })
        }
        inner(data, type_bound).map_err(|err| err.to_string())
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        match self {
            Self::I128(_) => std::mem::size_of::<i128>(),
            Self::I64(_) => std::mem::size_of::<i64>(),
            Self::I32(_) => std::mem::size_of::<i32>(),
            Self::I16(_) => std::mem::size_of::<i16>(),
            Self::I8(_) => std::mem::size_of::<i8>(),
            Self::U8(_) => std::mem::size_of::<u8>(),
            Self::U16(_) => std::mem::size_of::<u16>(),
            Self::U32(_) => std::mem::size_of::<u32>(),
            Self::U64(_) => std::mem::size_of::<u64>(),
            Self::U128(_) => std::mem::size_of::<u128>(),
            Self::String(v) => v.len(),
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    value: Value,
    type_bound: ValueType,
}

impl Variable {
    #[inline]
    pub const fn new(value: Value, type_bound: ValueType) -> Self {
        Self { value, type_bound }
    }

    #[inline]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    #[inline]
    pub const fn type_bound(&self) -> &ValueType {
        &self.type_bound
    }
}

#[derive(Default, Debug)]
pub struct Program {
    functions: HashMap<String, Function>,
    static_vars: HashMap<String, Variable>,
    options: ProgramOptions,
}

impl Program {
    #[inline]
    pub fn insert_static_var(&mut self, name: String, var: Variable) -> Option<Variable> {
        self.static_vars.insert(name, var)
    }

    #[inline]
    pub fn insert_function(&mut self, name: String, function: Function) -> Option<Function> {
        self.functions.insert(name, function)
    }

    #[inline]
    pub const fn new(
        functions: HashMap<String, Function>,
        static_vars: HashMap<String, Variable>,
        options: ProgramOptions,
    ) -> Self {
        Self {
            functions,
            static_vars,
            options,
        }
    }
}

impl Program {
    /// Calculate the amount of bytes this program will reserve before declaring functionality.
    pub fn calculate_reserve(&self) -> usize {
        self.static_vars
            .iter()
            .map(|(_, var)| var.value.get_size())
            .sum()
    }
}

impl Compile for Program {
    fn compile_into(&self, dest: &mut Vec<u8>, program_options: &ProgramOptions) {
        self.options.compile_into(dest, program_options);
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn it_works() {
        if let Err(err) = parse::parse(
            r#"
        static string helloWorld = "Hi";
        static string helloWorld = "duplicate";

        let main() {

        }
        "#,
        ) {
            err.display();
            panic!("failed");
        }
    }
}
