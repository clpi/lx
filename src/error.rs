use std::{fmt, error, io};
use tokio::io as TokioIo;
use crossterm::ErrorKind;

#[derive(Debug)]
pub enum LxError {
    IoError(io::Error),
    CrosstermError(crossterm::ErrorKind)

}
impl error::Error for LxError {
}
impl fmt::Display for LxError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => f.write_fmt(format_args!("{}", e.to_string())),
            Self::CrosstermError(e) => f.write_fmt(format_args!("{}", e.to_string()))
        }
    }
}
