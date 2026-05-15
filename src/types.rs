#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Sort,
    Count,
    Check,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppError {
    pub exit_code: i32,
    pub message: String,
}

impl AppError {
    pub fn new(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
