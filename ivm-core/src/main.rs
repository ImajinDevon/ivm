use ivm_core::ansi;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    ansi::init_ansi_terminal();
}
