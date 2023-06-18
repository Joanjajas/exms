use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use colored::Colorize;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    path: PathBuf,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    /// An error encountered while opening or reading a file
    Io(io::Error),

    /// An error encountered while parsing a TOML file
    ParseToml(toml::de::Error),

    /// An error encountered while parsing a JSON file
    ParseJson(serde_json::Error),

    /// A file doesn't have any file extension
    MissingFormat,

    /// A file has an unsupported extension
    UnsupportedFormat,
}

impl Error {
    /// Creates a new error from the ErrorKind and path provided
    pub(crate) fn new<P: AsRef<Path>>(kind: ErrorKind, path: P) -> Self {
        Self {
            kind,
            path: path.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get path extension as a &str and colour it
        let path_extension = self
            .path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .yellow();

        // Get path as &str and colour it
        let path_str = self.path.to_str().unwrap_or_default().yellow();

        match self.kind {
            ErrorKind::Io(ref err) => {
                write!(f, "Error while reading {path_str}: {err}")
            }

            ErrorKind::ParseToml(ref err) => {
                write!(f, "Error while parsing file {path_str}: {err}")
            }

            ErrorKind::ParseJson(ref err) => {
                write!(f, "Error while parsing file {path_str}: {err}")
            }

            ErrorKind::MissingFormat => write!(
                f,
                "Error while parsing file {path_str}: Unable to recognize file extension"
            ),

            ErrorKind::UnsupportedFormat => {
                write!(
                    f,
                    "Error while parsing file {path_str}: unsupported file format: {}{path_extension}",
                    ".".yellow(),
                )
            }
        }
    }
}

impl std::error::Error for Error {}
