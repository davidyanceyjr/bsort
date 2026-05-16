use crate::{AppError, AppResult, USAGE_OR_PARSE_ERROR};

pub fn parse_lines(input: &str, ignore_blank: bool) -> AppResult<Vec<i64>> {
    let mut values = Vec::new();

    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            if ignore_blank {
                continue;
            }

            return Err(AppError::new(
                USAGE_OR_PARSE_ERROR,
                format!("line {}: invalid integer '{}'", line_number, raw_line),
            ));
        }

        let value = trimmed.parse::<i64>().map_err(|_| {
            AppError::new(
                USAGE_OR_PARSE_ERROR,
                format!("line {}: invalid integer '{}'", line_number, raw_line),
            )
        })?;

        values.push(value);
    }

    Ok(values)
}
