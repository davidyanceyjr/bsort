pub mod cli;
pub mod check;
pub mod exit_codes;
pub mod help;
pub mod io;
pub mod order;
pub mod output;
pub mod parse;
pub mod sort;
pub mod types;
pub mod unique;

pub use cli::{parse_args, CliConfig};
pub use check::is_sorted;
pub use exit_codes::{CHECK_FAILED, IO_ERROR, SUCCESS, USAGE_OR_PARSE_ERROR};
pub use help::{usage_text, version_text};
pub use io::{read_file_text, read_stdin_text};
pub use order::{compare_pair, needs_swap};
pub use output::{format_count, format_stderr, format_values};
pub use parse::parse_lines;
pub use sort::bubble_sort;
pub use types::{AppError, AppResult, Mode, Order};
pub use unique::dedupe_sorted;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() -> AppResult<()> {
    let config = parse_args(std::env::args())?;

    if config.help {
        print!("{}", usage_text());
        return Ok(());
    }

    if config.version {
        print!("{}", version_text());
        return Ok(());
    }

    Ok(())
}
