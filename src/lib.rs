pub mod exit_codes;
pub mod io;
pub mod order;
pub mod sort;
pub mod types;

pub use exit_codes::{CHECK_FAILED, IO_ERROR, SUCCESS, USAGE_OR_PARSE_ERROR};
pub use io::read_stdin_text;
pub use order::{compare_pair, needs_swap};
pub use sort::bubble_sort;
pub use types::{AppError, AppResult, Mode, Order};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() -> Result<(), String> {
    Ok(())
}
