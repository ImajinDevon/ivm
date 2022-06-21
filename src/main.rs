cfg_if::cfg_if! {
    if #[cfg(feature = "compiler")] {
        use crate::compile::Instruction;

        use std::fs::File;
        use std::io;
        use std::io::{BufWriter, Write};
        use std::path::Path;

        pub mod compile;
        pub mod fmt;
        pub mod vm;
        pub mod cli;
    } else {
        use std::env;
    }
}

#[cfg(feature = "compiler")]
fn write_to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    writer.flush()
}

/// This function is mainly used for testing, as of now.
#[cfg(feature = "compiler")]
fn main() {
    let mut process = cli::LinearProcess::new("ivmc", 2);
    process.log("beginning compilation...");

    //TODO: add frontend

    // equivalent to the following (pseudocode):
    // var0 = "Hello, world!\n";
    // stdout.write(var0);
    // stdout.flush();
    let instructions = vec![
        Instruction::PushBytes(b"Hello, world!\n".to_vec()),
        Instruction::StaticVarStore(0),
        Instruction::StaticVarLoad(0),
        Instruction::Invoke(vm::intrinsics::STDOUT_WRITE),
        Instruction::Invoke(vm::intrinsics::STDOUT_FLUSH),
    ];

    // for testing
    vm::execute_instructions(
        vm::intrinsics::create_intrinsic_environment(),
        &instructions,
    );

    let bytecode = process.step_task("generating bytecode", || {
        compile::to_bytecode(&instructions)
    });

    let show = fmt::format_instruction_set(&bytecode);
    println!("# bytecode:\n{}# end of bytecode", show);

    match process.step_result("writing bytecode to file", || {
        write_to_file("output.ivm", &bytecode)
    }) {
        Ok(_) => process.log("finished compilation successfully"),
        Err(e) => {
            eprintln!("ivmc.failure: failed to write bytecode to output.ivm");
            eprintln!("ivmc.error: {e}");
        }
    }
}

#[cfg(not(any(feature = "compiler")))]
fn main() {
    let mut arg_iter = env::args().skip(1);

    let action = match arg_iter.next() {
        Some(action) => action,
        None => {
            eprintln!("ivm@error: no action specified");
            eprintln!("ivm@help: try `ivm help` for more information");
            return;
        }
    };

    let exit_code = match action.as_str() {
        "usage" => {
            println!("ivm: usage: ivm [help|version|usage]");
            0
        }
        "version" => {
            println!("ivm: installed ivm version: {}", env!("CARGO_PKG_VERSION"));
            0
        }
        "help" => {
            println!("ivm: note: for compilation, use: ivmc <*.iivm>");
            println!("ivm: usage: ivm [help|version|usage]");
            println!("  help     |  show this help message");
            println!("  version  |  show the version of ivm");
            println!("  usage    |  show the usage of the ivm cli");
            0
        }
        "compile" | "--compile" => {
            println!("ivm@help: try `ivmc <*.iivm>` instead");
            1
        }
        _ => {
            eprintln!("ivm@error: unknown action: {}", action);
            eprintln!("ivm@help: try `ivm help` for more information");
            1
        }
    };
    std::process::exit(exit_code);
}
