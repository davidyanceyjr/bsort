use crate::{AppError, AppResult, Mode, Order, USAGE_OR_PARSE_ERROR};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliConfig {
    pub input_path: Option<String>,
    pub order: Order,
    pub unique: bool,
    pub mode: Mode,
    pub ignore_blank: bool,
    pub help: bool,
    pub version: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            input_path: None,
            order: Order::Ascending,
            unique: false,
            mode: Mode::Sort,
            ignore_blank: false,
            help: false,
            version: false,
        }
    }
}

pub fn parse_args<I, S>(args: I) -> AppResult<CliConfig>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut config = CliConfig::default();
    let mut positionals = Vec::new();

    for arg in args.into_iter().skip(1).map(Into::into) {
        match arg.as_str() {
            "--desc" => config.order = Order::Descending,
            "--unique" => config.unique = true,
            "--count" => config.mode = Mode::Count,
            "--check" => config.mode = Mode::Check,
            "--ignore-blank" => config.ignore_blank = true,
            "--help" => config.help = true,
            "--version" => config.version = true,
            _ if arg.starts_with('-') => {
                return Err(usage_error(format!("unknown option: {arg}")));
            }
            _ => positionals.push(arg),
        }
    }

    if positionals.len() > 1 {
        return Err(usage_error("too many positional arguments"));
    }

    if config.order == Order::Descending && config.mode == Mode::Count {
        return Err(usage_error("cannot combine --desc with --count"));
    }

    if config.order == Order::Descending && config.mode == Mode::Check {
        return Err(usage_error("cannot combine --desc with --check"));
    }

    config.input_path = positionals.into_iter().next();

    Ok(config)
}

fn usage_error(message: impl Into<String>) -> AppError {
    AppError::new(USAGE_OR_PARSE_ERROR, message)
}
