use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Bound;

/// An error in calculating a partial sum.
#[derive(Debug, PartialEq, Eq)]
pub enum SumError {
    /// Range is empty: `(0..0)`, `(1..1)`, etc.
    RangeEmpty {
        bounds: (Bound<usize>, Bound<usize>),
    },

    /// Range is decreasing: `(10..0)`, etc.
    RangeDecreasing {
        bounds: (Bound<usize>, Bound<usize>),
    },

    // Range is not withing the range of the tree.
    RangeOutsideTree {
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
            SumError::RangeEmpty { bounds } => {
                write!(f, "Bounds {:#?} represent empty range", bounds)
            }
            SumError::RangeDecreasing { bounds } => {
                write!(f, "Bounds {:#?} represent decreasing range", bounds)
            }
            SumError::RangeOutsideTree { bounds, len } => write!(
                f,
                "Bounds {:#?} are outside of the tree range (0..{})",
                bounds, len
            ),
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

impl Error for SumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Error for AddError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
