use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

use getset::Getters;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Getters)]
pub struct Error {
    #[getset(get = "pub")]
    id: Uuid,

    #[getset(get = "pub")]
    code: Code,

    #[getset(get = "pub")]
    kind: ErrorKind,

    #[getset(get = "pub")]
    message: String,
}

impl Error {
    pub fn new(code: Code, kind: ErrorKind, message: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            code,
            kind,
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.code(), self.message(), self.id())
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub struct Code(&'static str);

impl Code {
    pub fn new(code: &'static str) -> Self {
        Self(code)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /// An error of unknown origin
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::error::{Code, Error, ErrorKind};

    #[test]
    fn display() {
        let error = Error::new(Code::new("LUN0000"), ErrorKind::Unknown, "display test");
        assert_eq!(
            format!("{}: display test ({})", error.code, error.id),
            error.to_string()
        );
    }
}
