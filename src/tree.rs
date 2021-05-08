//! A module that contains an implementation of the Fenwick tree.
//!
//! # Explanation
//!
//! In this section, an explanation of some of the implementation details is provided.
//!
//! ## Indexing
//!
//! The easiest explanation considers that indexing of the array is one-based (as in the
//! original paper).
//!
//! Sums are stored in the array by the following rule: an item with index `i` stores a cumulative sum
//! of `[i - g(i), i]` range, where `g(i) = i & (-i)` or the size of the range covered by `i`.
//!
//! This looks a bit scary but the whole intuition behind Fenwick tree is very simple:
//! the position of the first `1` bit in the `i` defines the value of `g(i)`.
//!     - `i = 4 = 100`, so it covers `g(i) = 100 (4)` items or `[1, 4]` range (note one-based indexing)
//!     - `i = 3 = 011`, so it covers `g(i) = 001 (1)` item  or `[3, 3]` range
//!     - `i = 2 = 010`, so it covers `g(i) = 010 (2)` items or `[1, 2]` range
//!     - `i = 1 = 001`, so it covers `g(i) = 001 (1)` item  or `[1, 1]` range
//!
//! ## Querying
//!
//! Prefix sum of `n` items consists of sum of some number of ranges, depending on `n`.
//!     - `n = 4` sum of `[1, 4]` (4 index)
//!     - `n = 3` sum of `[3, 3]` (3 index) and `[1, 2]` (2 index)
//!     - `n = 2` sum of `[1, 2]` (2 index)
//!     - `n = 1` sum of `[1, 1]` (1 index)
//!
//! In order to calculate a prefix sum, we need to traverse the array from right
//! to left, decreasing `i` by `g(i)`.
//!     - `i = 100 (4)`, `i - g(i) = 000 (0)`
//!     - `i = 011 (3)`, `i - g(i) = 010 (2)`
//!     - `i = 010 (2)`, `i - g(i) = 000 (0)`
//!     - `i = 001 (1)`, `i - g(i) = 000 (0)`
//!
//! In order to do so, we need a way to calculate `g(i)`. This can easily be done by using the
//! representation of negative numbers in two's complement systems, where `-i` means `!i + 1`.
//!
//! ```
//! # fn any_number() -> i32 { 3 }
//! let i = any_number();
//! assert_eq!(!i + 1, -i);
//! ```
//!
//! In binary, `!i` inverts all bits in the `i`, and `!i + 1` flips all bits up to first `0`.
//!     - `i = 001`, `!i = 110`, `-i = 111`
//!     - `i = 010`, `!i = 101`, `-i = 110`
//!     - `i = 011`, `!i = 100`, `-i = 101`
//!     - `i = 100`, `!i = 011`, `-i = 100`
//!
//! This means that `i` and `-i` have all bits different except the first `1` bit.
//! So, `g(i)` is as simple as `i & (-i)`.
//!
//! ## Updating
//!
//! Updating a value at `i` means traversing the array from left to right, updating
//! ranges that contain the value.
//!     - `i = 4` -> `8`
//!     - `i = 3` -> `4`
//!     - `i = 2` -> `4`
//!     - `i = 1` -> `2`
//!
//! Here similar logic applies as for querying: we need to increase `i` by `g(i)`.
//!     - `i = 4 = 100`, `i + (i & (-i)) = 100 + 100 = 1000 (8)`
//!     - `i = 3 = 011`, `i + (i & (-i)) = 011 + 001 = 0100 (4)`
//!     - `i = 2 = 010`, `i + (i & (-i)) = 010 + 010 = 0100 (4)`
//!     - `i = 1 = 001`, `i + (i & (-i)) = 001 + 001 = 0010 (2)`
//!
//! That's it!
//!
//! # Notes
//!
//! The explanation above assumes one-based indexing, however, in Rust, as in most other programming
//! languages, indexing is zero-based. Thus, it's not that easy to calculate `i & (-i)` if `i` is
//! of `usize`.
//!
//! For the sake of code simplicity and performance, the following changes were made:
//!     - querying is one-based:  `i & (i - 1)`
//!     - updating is zero-based: `i | (i + 1)

use std::ops::{AddAssign, Range, SubAssign};

/// An implementation of the binary indexed tree (Fenwick tree) data structure.
///
/// The tree is backed by a simple array/vec of the fixed size where each item is
/// responsible for storing cumulative sum of some range, allowing to perform
/// queries and updates in O(log n) time.
///
/// # Examples
///
/// ```
/// use fenwick_tree::FenwickTree;
///
/// let mut tree = FenwickTree::<i32>::of_size(3);
///
/// tree.add(0, 1);
/// tree.add(1, 2);
/// tree.add(2, 3);
///
/// assert_eq!(tree.sum(0..1), 1);
/// assert_eq!(tree.sum(0..2), 3);
/// assert_eq!(tree.sum(0..3), 6);
///
/// assert_eq!(tree.sum(1..2), 2);
/// assert_eq!(tree.sum(1..3), 5);
///
/// assert_eq!(tree.sum(2..3), 3);
///
/// ```
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
    /// Complexity: O(log n).
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
    /// Complexity: O(log n).
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
