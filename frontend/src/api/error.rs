#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status_code: Option<u16>,
}

impl ApiError {
    pub fn new(message: String, status_code: Option<u16>) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        Self {
            message,
            status_code: None,
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;