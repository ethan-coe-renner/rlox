use thiserror::Error;

#[derive(Debug, Error)]
#[error("[line {line}] Error: {message}")]
pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn error(line: usize, message: String) -> Self {
	Self {
	    line,
	    message
	}
    }
}
