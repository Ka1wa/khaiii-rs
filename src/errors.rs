use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    KhaiiiExcept(String),
    ResourcesFailure,
    Unknown
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::KhaiiiExcept(error) => write!(f, "Khaiii exception: {}", error),
            Error::ResourcesFailure => write!(f, "Khaiii resources were not loaded"),
            Error::Unknown => write!(f, "Unknown exception"),
        }
    }
}
