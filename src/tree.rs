use std::ops::{AddAssign, Bound, RangeBounds, SubAssign};

use num_traits::Zero;

use crate::errors::{AddError, SumError};

/// An implementation of the binary indexed tree (Fenwick tree) data structure.
///
/// The tree is backed by a simple vec of the fixed size where each item is
/// responsible for storing cumulative sum of some range, allowing to perform
/// queries and updates in _O_(log _n_) time.
pub struct FenwickTree<I>
where
    I: Zero + Copy + AddAssign + SubAssign,
{
    tree: Vec<I>,
}

#[allow(clippy::len_without_is_empty)]
impl<I> FenwickTree<I>
where
    I: Zero + Copy + AddAssign + SubAssign,
{
    /// Constructs a new Fenwick tree with the specified `len` with each element set as
    /// `I::default()`.
    ///
    /// The vector is initialized with `vec![I::default(); len]`.
    ///
    /// # Panics
    ///
    /// Vector initialization may panic if `len` is too big.
    pub fn with_len(len: usize) -> Self {
        Self {
            tree: vec![I::zero(); len],
        }
    }

    /// A length of the backing vector of the tree.
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// A partial sum of the specified `bounds`.
    ///
    /// Complexity: _O_(log _n_).
    ///
    /// Note that `sum` for empty range (a range `(i..j)` where `i >= j`) is `0`.
    ///
    /// Also, `bounds` are converted into a pair `(start, end)` that represents `[start,
    /// end)` range. This means that boundary case `tree.sum(i..=usize::MAX)` fallbacks to `tree.sum(0..usize::MAX)`.
    /// However, in practice, it's not possible to construct such a big tree ([`Vec`] panics with
    /// `capacity overflow`).
    pub fn sum<T>(&self, bounds: T) -> Result<I, SumError>
    where
        T: RangeBounds<usize>,
    {
        let len = self.len();

        let mut sum = I::zero();
        let mut start = start(bounds.start_bound());
        let mut end = end(bounds.end_bound(), len);

        if start >= len || end > len {
            return Err(SumError::OutOfRange {
                bounds: as_pair(bounds),
                len,
            });
        }

        while end > start {
            sum += self.tree[end - 1];
            end = prev(end);
        }

        while start > end {
            sum -= self.tree[start - 1];
            start = prev(start);
        }

        Ok(sum)
    }

    /// Updates the value at `i` by `delta`.
    ///
    /// Complexity: _O_(log _n_).
    pub fn add(&mut self, mut i: usize, delta: I) -> Result<(), AddError> {
        let size = self.len();

        if i >= size {
            return Err(AddError::IndexOutOfRange { index: i, size });
        }

        while i < size {
            self.tree[i] += delta;
            i = next(i);
        }

        Ok(())
    }
}

/// Flips first trailing `1` in the binary representation of the `i`. Same as `i - (i & (-i))` (see
/// crate docs).
///
/// This allows fast calculating of prefix sums:
///     - call `i = prev(i)` until `i` is greater than 0
///     - access sums by `i - 1`
///
/// This function assumes that indexing is one-based, hence we access sums by `i - 1`.
/// However, it's worth to note that zero-based solution (`i & (i + 1)`) produces less cleaner code
/// because to iterate we need to call `i = prev(i) - 1`, which involves additional checks when `i`
/// is of `usize` (decrement may result in panic).
#[inline(always)]
const fn prev(i: usize) -> usize {
    i & (i - 1)
}

/// Flips first trailing `0` in the binary representation of the `i`.
///
/// In the same way as with `prev`, `i = next(i)` allows traversing the array but in the opposite
/// direction.
/// However, unlike `prev`, this function assumes that indexing is zero-based, hence we access sums by `i`.
#[inline(always)]
const fn next(i: usize) -> usize {
    i | (i + 1)
}

// As inclusive.
#[inline(always)]
fn start(bound: Bound<&usize>) -> usize {
    match bound {
        Bound::Excluded(&usize::MAX) => usize::MAX,
        Bound::Excluded(x) => *x + 1,
        Bound::Included(x) => *x,
        Bound::Unbounded => usize::MIN,
    }
}

// As exclusive.
#[inline(always)]
fn end(bound: Bound<&usize>, len: usize) -> usize {
    match bound {
        Bound::Included(&usize::MAX) => usize::MAX,
        Bound::Included(x) => *x + 1,
        Bound::Excluded(x) => *x,
        Bound::Unbounded => len,
    }
}

#[inline(always)]
fn as_pair<T>(bounds: T) -> (Bound<usize>, Bound<usize>)
where
    T: RangeBounds<usize>,
{
    (bounds.start_bound().cloned(), bounds.end_bound().cloned())
}
