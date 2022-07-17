use std::str::FromStr;

use pest::error::ErrorVariant;
use pest::iterators::Pairs;
use pest::Parser;
use pest::{error, Span};

use crate::{Error, Function, Program, Value, ValueType, Variable};

pub type ParseResult<'a, T> = Result<T, Error<'a>>;

#[derive(pest_derive::Parser)]
#[grammar = "../ivm_ir_grammar.pest"]
pub struct IvmIrParser;

fn parse_function(mut it: Pairs<Rule>) -> ParseResult<Function> {
    let param_collector = it.next().unwrap().into_inner();
    let mut function = Function::default();

    for param in param_collector {
        let mut it = param.into_inner();

        let value_type_str = it.next().unwrap().as_str();
        let value_type = ValueType::from_str(value_type_str).unwrap();

        let param_pair = it.next().unwrap();
        let param_name = param_pair.as_str();

        if function
            .insert_param(param_name.to_string(), value_type)
            .is_some()
        {
            return Err(custom_err(
                param_pair.as_span(),
                "function parameter declared more than once",
                format!("function parameter '{param_name}' also declared here",),
            ));
        }
    }
    Ok(function)
}

fn custom_err<'a>(span: Span, short: &'a str, message: String) -> Error<'a> {
    Error::from(
        short,
        error::Error::new_from_span(ErrorVariant::CustomError::<Rule> { message }, span),
    )
}

pub fn parse(contents: &str) -> ParseResult<Program> {
    let program_rule = IvmIrParser::parse(Rule::program, contents)
        .map_err(|err| Error::from("failed to parse input", err))?;

    let mut program = Program::default();

    for pair in program_rule {
        //FIXME invalid parsing
        println!("{pair:#?}");
        let global = pair.into_inner().next().unwrap();

        let mut it = global.into_inner();
        let pair = it.next().unwrap();

        match pair.as_rule() {
            Rule::function => {
                let name_pair = it.next().unwrap();
                let name = name_pair.as_str();

                let function = parse_function(it)?;

                if program
                    .insert_function(name.to_string(), function)
                    .is_none()
                {
                    continue;
                }
                return Err(custom_err(
                    name_pair.as_span(),
                    "function declared more than once",
                    format!("function '{name}' also declared here"),
                ));
            }
            Rule::declare_static => {
                let type_bound = ValueType::from_str(it.next().unwrap().as_str()).unwrap();

                let name_pair = it.next().unwrap();
                let name = name_pair.as_str();

                let value_pair = it.next().unwrap();

                let value = Value::try_from(value_pair.as_str(), &type_bound).map_err(|err| {
                    custom_err(value_pair.as_span(), "invalid constant conversion", err)
                })?;

                let var = Variable::new(value, type_bound);

                if program.insert_static_var(name.to_string(), var).is_none() {
                    continue;
                }
                return Err(custom_err(
                    name_pair.as_span(),
                    "static variable declared more than once",
                    format!("static variable '{name}' also declared here"),
                ));
            }
            _ => (),
        }
    }

    println!("{program:#?}");
    Ok(program)
}
