use ivm_common::cli::LinearProcess;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let process = LinearProcess::new("ivm", 0);
    process.log(format!("ivm version {VERSION}"));

    // TODO add proper cli functionality
}
