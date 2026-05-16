pub mod check;
pub mod cli;
pub mod exit_codes;
pub mod help;
pub mod io;
pub mod order;
pub mod output;
pub mod parse;
pub mod sort;
pub mod types;
pub mod unique;

pub use check::is_sorted;
pub use cli::{parse_args, CliConfig};
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

    let input = read_input(&config)?;
    let values = parse_lines(&input, config.ignore_blank)?;
    let output = run_mode(&config, values)?;

    print!("{output}");

    Ok(())
}

fn read_input(config: &CliConfig) -> AppResult<String> {
    match config.input_path.as_deref() {
        Some(path) => read_file_text(path),
        None => read_stdin_text().map_err(|err| AppError::new(IO_ERROR, format!("stdin: {err}"))),
    }
}

fn run_mode(config: &CliConfig, values: Vec<i64>) -> AppResult<String> {
    match config.mode {
        Mode::Sort => Ok(format_values(&apply_sort_options(
            values,
            config.order,
            config.unique,
        ))),
        Mode::Count => Ok(format_count(values.len())),
        Mode::Check => {
            if is_sorted(&values, config.order) {
                Ok(String::new())
            } else {
                Err(AppError::new(CHECK_FAILED, ""))
            }
        }
    }
}

fn apply_sort_options(values: Vec<i64>, order: Order, unique: bool) -> Vec<i64> {
    let mut sorted = bubble_sort(&values);

    if order == Order::Descending {
        sorted.reverse();
    }

    if unique {
        dedupe_sorted(&sorted)
    } else {
        sorted
    }
}
