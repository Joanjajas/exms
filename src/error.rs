use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use colored::Colorize;

/// A trait for adding a path to a result to add more context to
/// an error
pub(crate) trait WithPath<T, E> {
    /// Adds a path to a result to add more context to an error
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)>;
}

impl<T, E> WithPath<T, E> for Result<T, E> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)> {
        self.map_err(|err| (err, path))
    }
}

/// This type represents all possible errors that can occur while parsing a file
#[derive(Debug)]
pub struct ParseError {
    /// Kind of error occurred
    kind: ParseErrorKind,

    /// Path of the file that caused the error
    path: PathBuf,
}

impl ParseError {
    /// Creates a new `ParseError` from the kind and path provided
    pub(crate) fn new<P: AsRef<Path>>(kind: ParseErrorKind, path: P) -> Self {
        Self {
            kind,
            path: path.as_ref().to_owned(),
        }
    }
}

/// Enum that categorizes a 'exms::error::ParseError'
#[derive(Debug)]
pub(crate) enum ParseErrorKind {
    /// An error encountered while opening or reading a file
    Io(io::Error),

    /// An error encountered while parsing a TOML file
    Toml(toml::de::Error),

    /// An error encountered while parsing a JSON file
    Json(serde_json::Error),

    /// A file doesn't have any file extension
    MissingFormat,

    /// A file has an unsupported extension
    UnsupportedFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_path = self.path.to_str().unwrap_or_default().yellow();
        let colored_extension = self
            .path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .yellow();

        match &self.kind {
            ParseErrorKind::Io(err) => {
                write!(f, "Error while reading {colored_path}: {err}",)
            }

            ParseErrorKind::Toml(err) => {
                write!(f, "Error while parsing file {colored_path}: {err}",)
            }

            ParseErrorKind::Json(err) => {
                write!(f, "Error while parsing file {colored_path}: {err}",)
            }

            ParseErrorKind::MissingFormat => write!(
                f,
                "Error while parsing file {colored_path}: Unable to recognize file extension",
            ),

            ParseErrorKind::UnsupportedFormat => {
                write!(
                    f,
                    "Error while parsing file {colored_path}: unsupported file format: {}{colored_extension}",
                    ".".yellow(),
                )
            }
        }
    }
}

impl<P: AsRef<Path>> From<(io::Error, P)> for ParseError {
    fn from((io_err, path): (io::Error, P)) -> Self {
        ParseError::new(ParseErrorKind::Io(io_err), path)
    }
}

impl<P: AsRef<Path>> From<(toml::de::Error, P)> for ParseError {
    fn from((toml_err, path): (toml::de::Error, P)) -> Self {
        ParseError::new(ParseErrorKind::Toml(toml_err), path)
    }
}

impl<P: AsRef<Path>> From<(serde_json::Error, P)> for ParseError {
    fn from((json_err, path): (serde_json::Error, P)) -> Self {
        ParseError::new(ParseErrorKind::Json(json_err), path)
    }
}

impl std::error::Error for ParseError {}
