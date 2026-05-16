use crate::AppError;

pub fn format_values(values: &[i64]) -> String {
    if values.is_empty() {
        return String::new();
    }

    let mut output = values
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    output.push('\n');
    output
}

pub fn format_count(count: usize) -> String {
    format!("{count}\n")
}

pub fn format_stderr(err: &AppError) -> String {
    format!("{}\n", err.message)
}
