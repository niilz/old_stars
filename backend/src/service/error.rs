use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct OldStarsServiceError {
    pub message: String,
}
impl OldStarsServiceError {
    pub fn new(context: &str, error: &(dyn fmt::Display)) -> Self {
        OldStarsServiceError {
            message: format!("Error during {}: {}", context, error),
        }
    }
}

impl fmt::Display for OldStarsServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OldStarsServiceError: {}", self.message)
    }
}
impl Error for OldStarsServiceError {}

impl From<diesel::result::Error> for OldStarsServiceError {
    fn from(error: diesel::result::Error) -> Self {
        Self::new("db-communication", &error)
    }
}
