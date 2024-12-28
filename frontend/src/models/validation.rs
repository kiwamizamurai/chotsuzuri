#[derive(Debug)]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        Self {
            field: None,
            message,
        }
    }

    pub fn with_field(field: String, message: String) -> Self {
        Self {
            field: Some(field),
            message,
        }
    }
}