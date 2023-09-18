use std::fmt;

#[derive(Debug, Clone)]
pub struct BadOffset {
    pub message: String,
}

impl fmt::Display for BadOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UTC Offset must be less than 14 and greater than -14.")
    }
}

impl std::error::Error for BadOffset {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
