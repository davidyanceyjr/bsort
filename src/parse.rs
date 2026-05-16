use crate::{AppError, AppResult, USAGE_OR_PARSE_ERROR};

pub fn parse_lines(input: &str, ignore_blank: bool, valid_only: bool) -> AppResult<Vec<i64>> {
    let mut values = Vec::new();

    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            if ignore_blank || valid_only {
                continue;
            }

            return Err(AppError::new(
                USAGE_OR_PARSE_ERROR,
                format!("line {}: invalid integer '{}'", line_number, raw_line),
            ));
        }

        let value = match trimmed.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                if valid_only {
                    continue;
                }

                return Err(AppError::new(
                    USAGE_OR_PARSE_ERROR,
                    format!("line {}: invalid integer '{}'", line_number, raw_line),
                ));
            }
        };

        values.push(value);
    }

    Ok(values)
}
