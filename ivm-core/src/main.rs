use ivm_common::cli::LinearLogger;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let logger = LinearLogger::new("ivm", 0);
    logger.log(format!("ivm version {VERSION}"));

    // TODO add proper cli functionality
}
