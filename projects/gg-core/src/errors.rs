use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type Result<T> = std::result::Result<T, PlotError>;

/// The error type for this crate.
#[derive(Debug, Clone)]
pub struct PlotError {
    kind: Box<PlotErrorKind>,
}

/// The kind of error.
#[derive(Debug, Clone)]
pub enum PlotErrorKind {}

impl Error for PlotError {}

impl Error for PlotErrorKind {}

impl Display for PlotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for PlotErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
