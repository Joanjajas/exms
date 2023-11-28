use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use colored::Colorize;

pub(crate) trait WithPath<T, E> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)>;
}

impl<T, E> WithPath<T, E> for Result<T, E> {
    fn with_path<P: AsRef<Path>>(self, path: P) -> Result<T, (E, P)> {
        self.map_err(|err| (err, path))
    }
}

/// This type represents all possible errors that can occur while parsing an
/// exam file
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    path: PathBuf,
}

impl ParseError {
    pub(crate) fn new<P: AsRef<Path>>(kind: ParseErrorKind, path: P) -> Self {
        Self {
            kind,
            path: path.as_ref().to_owned(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParseErrorKind {
    Io(io::Error),
    Toml(toml::de::Error),
    Json(serde_json::Error),
    UnsupportedFormat,
    MissingFormat,
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
