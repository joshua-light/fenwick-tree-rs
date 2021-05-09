use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error in calculating a partial sum.
#[derive(Debug, PartialEq, Eq)]
pub enum SumError {
    /// Range is empty: `(0..0)`, `(1..1)`, etc.
    EmptyRange(usize),

    /// Range is decreasing: `(10..0)`, etc.
    DecreasingRange(usize, usize),
}

impl Error for SumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for SumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            SumError::EmptyRange(i) => write!(f, "Range ({}..{}) is empty", i, i),
            SumError::DecreasingRange(i, j) => write!(f, "Range ({}..{}) is decreasing", i, j),
        }
    }
}

// An error in adding a delta to a tree element.
#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    // Index is greater than the size of the tree.
    IndexOutOfRange(usize, usize),
}

impl Error for AddError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for AddError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            AddError::IndexOutOfRange(i, size) => {
                write!(f, "Index `{}` is greater than the size `{}`", i, size)
            }
        }
    }
}
