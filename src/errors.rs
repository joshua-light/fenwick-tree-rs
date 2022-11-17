use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Bound;

/// An error in calculating a partial sum.
#[derive(Debug, PartialEq, Eq)]
pub enum SumError {
    // Range is not within the range of the tree.
    OutOfRange {
        bounds: (Bound<usize>, Bound<usize>),
        len: usize,
    },
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
            SumError::OutOfRange { bounds, len } => {
                write!(f, "Bounds {:#?} are not in range (0..{})", bounds, len)
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

impl Error for SumError {}

impl Error for AddError {}
