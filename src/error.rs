use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use colored::Colorize;

/// A trait for converting a result into a result with a path to add context to
/// an error
pub(crate) trait WithPath<T, E> {
    /// Adds a path to an error
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)>;
}

impl<T, E> WithPath<T, E> for Result<T, E> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)> {
        self.map_err(|err| (err, path))
    }
}

/// Enum representing all errors that can occur while parsing a file
#[derive(Debug)]
pub enum ParseError {
    /// An error encountered while opening or reading a file
    Io(io::Error, PathBuf),

    /// An error encountered while parsing a TOML file
    Toml(toml::de::Error, PathBuf),

    /// An error encountered while parsing a JSON file
    Json(serde_json::Error, PathBuf),

    /// A file doesn't have any file extension
    MissingFormat(PathBuf),

    /// A file has an unsupported extension
    UnsupportedFormat(PathBuf),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(ref err, path) => {
                write!(
                    f,
                    "Error while reading {}: {err}",
                    path.to_str().unwrap_or_default().yellow()
                )
            }

            ParseError::Toml(ref err, path) => {
                write!(
                    f,
                    "Error while parsing file {}: {err}",
                    path.to_str().unwrap_or_default().yellow()
                )
            }

            ParseError::Json(ref err, path) => {
                write!(
                    f,
                    "Error while parsing file {}: {err}",
                    path.to_str().unwrap_or_default().yellow()
                )
            }

            ParseError::MissingFormat(path) => write!(
                f,
                "Error while parsing file {}: Unable to recognize file extension",
                path.to_str().unwrap_or_default().yellow()
            ),

            ParseError::UnsupportedFormat(path) => {
                write!(
                    f,
                    "Error while parsing file {}: unsupported file format: {}{}",
                    path.to_str().unwrap_or_default().yellow(),
                    ".".yellow(),
                    path.extension()
                        .and_then(OsStr::to_str)
                        .unwrap_or_default()
                        .yellow()
                )
            }
        }
    }
}

impl<P: AsRef<Path>> From<(io::Error, P)> for ParseError {
    fn from((io_err, path): (io::Error, P)) -> Self {
        ParseError::Io(io_err, path.as_ref().to_owned())
    }
}

impl<P: AsRef<Path>> From<(toml::de::Error, P)> for ParseError {
    fn from((toml_err, path): (toml::de::Error, P)) -> Self {
        ParseError::Toml(toml_err, path.as_ref().to_owned())
    }
}

impl<P: AsRef<Path>> From<(serde_json::Error, P)> for ParseError {
    fn from((json_err, path): (serde_json::Error, P)) -> Self {
        ParseError::Json(json_err, path.as_ref().to_owned())
    }
}

impl std::error::Error for ParseError {}
