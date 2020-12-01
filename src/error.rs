
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "io error: {}", err),
            Error::Other(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Other(err.into())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Other(err.to_string().into())
    }
}
