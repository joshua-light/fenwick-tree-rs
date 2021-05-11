use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error in calculating a partial sum.
#[derive(Debug, PartialEq, Eq)]
pub enum SumError {
    /// Range is empty: `(0..0)`, `(1..1)`, etc.
    EmptyRange { start: usize },

    /// Range is decreasing: `(10..0)`, etc.
    DecreasingRange { start: usize, end: usize },
}

/// An error in adding a delta to a tree element.
#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    /// Index is greater than the size of the tree.
    IndexOutOfRange { index: usize, size: usize },
}

impl Display for SumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            SumError::EmptyRange { start } => write!(f, "Range ({}..{}) is empty", start, start),
            SumError::DecreasingRange { start, end } => {
                write!(f, "Range ({}..{}) is decreasing", start, end)
            }
        }
    }
}

impl Display for AddError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            AddError::IndexOutOfRange { index, size } => {
                write!(f, "Index `{}` is greater than the size `{}`", index, size)
            }
        }
    }
}

impl Error for AddError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Error for SumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
