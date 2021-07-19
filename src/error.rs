use std::{fmt, error, io};
use tokio::io as TokioIo;
use crossterm::ErrorKind;

pub type LxResult<T> = Result<T, LxError>;

#[derive(Debug)]
pub enum LxError {
    IoError(io::Error),
    CrosstermError(crossterm::ErrorKind),
    ConfigError(LxConfigError),

}
#[derive(Debug)]
pub enum LxConfigError {
    InvalidKeymap(String),
    InvalidColor(String),
    UnrecognizedKeyword(String),
    IoError(io::Error),
}
impl error::Error for LxError {
}
impl error::Error for LxConfigError {

}
impl fmt::Display for LxConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => f.write_fmt(format_args!("{}", e.to_string())),
            Self::UnrecognizedKeyword(e) => f.write_fmt(format_args!("{}", e)),
            Self::InvalidColor(e) => f.write_fmt(format_args!("{}", e)),
            Self::InvalidKeymap(e) => f.write_fmt(format_args!("{}", e)),
        }
    }
}
impl fmt::Display for LxError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => f.write_fmt(format_args!("{}", e.to_string())),
            Self::CrosstermError(e) => f.write_fmt(format_args!("{}", e.to_string())),
            Self::ConfigError(e) => f.write_fmt(format_args!("{}", e)),
        }
    }
}
impl From<io::Error> for LxError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
impl From<io::Error> for LxConfigError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
impl From<LxConfigError> for LxError {
    fn from(e: LxConfigError) -> Self {
        Self::ConfigError(e)
    }
}
