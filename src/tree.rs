use std::ops::{AddAssign, Range, SubAssign};

/// An implementation of the binary indexed tree (Fenwick tree) data structure.
///
/// The tree is backed by a simple array/vec of the fixed size where each item is
/// responsible for storing cumulative sum of some range, allowing to perform
/// queries and updates in O(log n) time.
pub struct FenwickTree<I>
where
    I: Default + Copy + AddAssign + SubAssign,
{
    tree: Vec<I>,
}

impl<I> FenwickTree<I>
where
    I: Default + Copy + AddAssign + SubAssign,
{
    /// Constructs a new Fenwick tree of the specified `size` with each element set as
    /// `I::default()`.
    ///
    /// The vector is initialized with `vec![I::default(); size]`.
    pub fn of_size(size: usize) -> Self {
        Self {
            tree: vec![I::default(); size],
        }
    }

    /// A size of the backing vector of the tree.
    pub fn size(&self) -> usize {
        self.tree.len()
    }

    /// A partial sum of the specified range.
    ///
    /// Complexity: _O_(log _n_).
    pub fn sum(&self, range: Range<usize>) -> I {
        let mut s = I::default();
        let mut i = range.start;
        let mut j = range.end;

        while j > i {
            s += self.tree[j - 1];
            j = prev(j);
        }

        while i > j {
            s -= self.tree[i - 1];
            i = prev(i);
        }

        s
    }

    /// Updates the value at `i` by `delta`.
    ///
    /// Complexity: _O_(log _n_).
    pub fn add(&mut self, mut i: usize, delta: I) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i = next(i);
        }
    }
}

/// Flips first trailing `1` in the binary representation of the `i`. Same as `i - (i & (-i))` (see
/// module docs).
///
/// This allows fast calculating of prefix sums:
///     - call `i = prev(i)` until `i` is greater than 0
///     - access sums by `i - 1`
///
/// This function assumes that indexing is one-based, hence we access sums by `i - 1`.
/// However, it's worth to note that zero-based solution (`i & (i + 1)`) produces less cleaner code
/// because to iterate we need to call `i = prev(i) - 1`, which involves additional checks when `i`
/// is of `usize` (decrement may result in panic).
const fn prev(i: usize) -> usize {
    i & (i - 1)
}

/// Flips first trailing `0` in the binary representation of the `i`.
///
/// In the same way as with `prev`, `i = next(i)` allows traversing the array but in the opposite
/// direction.
/// However, unlike `prev`, this function assumes that indexing is zero-based, hence we access sums by `i`.
const fn next(i: usize) -> usize {
    i | (i + 1)
}
