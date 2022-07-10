use ivm_compile::{Instruction, options};
use ivm_compile::options::{MemoryPointerLength, ProgramOptions};
use ivm_vm::{ExternMap, ivm_ext_x32, StackElement, VmInstance};

//FIXME unfinished

fn execute(bytes: Vec<u8>) {
    let mut vm = VmInstance::with_ivm_ext_x32(ProgramOptions::new(
        options::CCFV,
        MemoryPointerLength::X32b,
    ));

    vm.introduce(bytes);
    vm.continue_execution();
}

enum Operation {
    Println,
    Print,
}

impl Operation {
    fn get_call_id(&self) -> usize {
        match self {
            Self::Println => 0,
            Self::Print => 1,
        }
    }
}

enum TokenKind {
    Operation(Operation),
    Semicolon,
}

struct Location<'a> {
    ln: usize,
    txt: &'a str,
    column: usize,
}

impl Location<'_> {
    #[rustfmt::skip]
    fn point(&self) {
        let trim = self.txt.trim_start();
        let pointer = " ".repeat(self.txt.len() - trim.len() + self.txt.len());
        println!(" src @    {}", trim);
        println!("     |    {}^", pointer);
        println!("     |  {}here", pointer);
    }

    fn print_help(&self, help: &str) {
        println!("help + {help}");
    }

    fn new(ln: usize, txt: &str, column: usize) -> Self {
        Self { ln, txt, column }
    }
}

struct Token<'a> {
    kind: TokenKind,
    loc: Location<'a>,
}

impl Token<'_> {
    fn new(kind: TokenKind, loc: Location<'_>) -> Self {
        Self { kind, loc }
    }
}

fn warn(msg: &str) {
    println!("ps(warn): {msg}");
}

//FIXME finish me!
fn compile(tokens: Vec<Token>) -> Vec<u8> {
    let mut instructions = Vec::new();

    for token in tokens {
        match token {
            Token::Operation(op) => {
                instructions.push(match op {
                    Operation::Print => {
                        Instruction::ExternCall(ivm_ext_x32::EXTC_STDOUT_WRITE),
                    }
                    Operation::Println => {
                        instructions.p
                    }
                });
            },
            
            Token::Semicolon => {
                warn("redundant semicolon");
                token.loc.point();
                token.loc.print_help("remove this semicolon");
            }
        }
    }
}

fn lex(source: String) {
    for (no, line) in source.lines().enumerate() {}
}

fn main() {}
