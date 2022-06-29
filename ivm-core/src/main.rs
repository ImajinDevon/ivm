use ivm_common::cli::LinearLogger;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let process = LinearLogger::new("ivm", 0);
    process.log(format!("ivm version {VERSION}"));

    // TODO add proper cli functionality
}
