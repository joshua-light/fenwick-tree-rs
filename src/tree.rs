//! A module that contains implementation of the Fenwick tree.
//!
//! # Explanation
//!
//! In this section an explanation of some of the implementation details is provided.
//!
//! ## Indexing
//!
//! The easiest explanation considers that indexing of the array is one-based (as in the
//! original paper).
//!
//! Sums are stored in the array by the following rule: an item with index `i` stores a cumulative sum
//! of `[i - (i & (-i)), i]` range.
//!
//! This looks a bit scary but the whole intuition behind Fenwick tree is very simple:
//! the position of the first `1` bit in the `i` equals to the size of the range covered by `i`.
//!     - `i = 4 = 100`, so it covers `4` items or `[1, 4]` range (note one-based indexing)
//!     - `i = 3 = 011`, so it covers `1` item  or `[3, 3]` range
//!     - `i = 2 = 010`, so it covers `2` items or `[1, 2]` range
//!     - `i = 1 = 001`, so it covers `1` item  or `[1, 1]` range
//!
//! Prefix sum of `n` items consists of sum of these ranges, depending on `n`.
//!     - `n = 4` sum of `[1, 4]` (4 index)
//!     - `n = 3` sum of `[3, 3]` (3 index) and `[1, 2]` (2 index)
//!     - `n = 2` sum of `[1, 2]` (2 index)
//!     - `n = 1` sum of `[1, 1]` (1 index)
//!
//! ## Querying
//!
//! In order to calculate the prefix sum, we need to traverse the array from right
//! to left, decreasing `i` by the size of the range it covers.
//!     - `i = 4` -> `0`
//!     - `i = 3` -> `2`
//!     - `i = 2` -> `0`
//!     - `i = 1` -> `0`
//!
//! From the binary perspective, this means flipping the first `1` bit in the `i`.
//!     - `i = 4 = 100` -> `000`
//!     - `i = 3 = 011` -> `010`
//!     - `i = 2 = 010` -> `000`
//!     - `i = 1 = 001` -> `000`
//!
//! In order to flip the first `1` bit in `i`, we need to subtract some number `N` from `i`,
//! where `N` has only one bit set at the same position.
//!     - `i = 4 = 100`, `N = 100`, `i - N = 000`
//!     - `i = 3 = 011`, `N = 001`, `i - N = 010`
//!     - `i = 2 = 010`, `N = 010`, `i - N = 010`
//!     - `i = 1 = 001`, `N = 001`, `i - N = 000`
//!
//! `N` can be obtained in a simple way, using the representation of negative numbers in two's
//! complement systems: `-i` means `!i + 1`.
//!
//! ```
//! # fn any_number() -> i32 { 4 }
//!
//! let i = any_number();
//! assert_eq!(!i + 1, -i);
//! ```
//!
//! In binary, `!i + 1` would invert all bits in the `i` and then flip all bits up to first `0`.
//!     - `i = 001`, `!i = 110`, `-i = 111`
//!     - `i = 010`, `!i = 101`, `-i = 110`
//!     - `i = 011`, `!i = 100`, `-i = 101`
//!     - `i = 100`, `!i = 011`, `-i = 100`
//!
//! This means that `i` and `-i` have all bits different except the first `1` bit.
//! Hence, `i & (-i)` is a number `N` from the statements above.
//!
//! That's why `i - (i & (-i))` would flip the first `1` bit in `i`, allowing to traverse the array
//! and calculating the prefix sum.
//!
//! ## Updating
//!
//! In the same way, updating a value at `i` means traversing the array from left to right, updating
//! ranges that contain value at `i`.
//!     - `i = 4` is contained in `[1, 4]`
//!     - `i = 3` is contained in `[3, 3]`, `[1, 4]`
//!     - `i = 2` is contained in `[1, 2]`, `[1, 4]`
//!     - `i = 1` is contained in `[1, 1]`, `[1, 2]`, `[1, 4]`
//!
//! Here the intuition is the same: we need to increase `i` by the size of the range it covers.
//! Following the symmetry, if `i - (i & (-i))` is used to iterate from right to left,
//! then `i + (i & (-i))` should be used to iterate from left to right.
//!     - `i = 4 = 100`, `i + (i & (-i)) = 100 + 100 = 1000`
//!     - `i = 3 = 011`, `i + (i & (-i)) = 011 + 001 = 0100`
//!     - `i = 2 = 010`, `i + (i & (-i)) = 010 + 010 = 0100`
//!     - `i = 1 = 001`, `i + (i & (-i)) = 001 + 001 = 0010`
//!
//! That's it!
//!
//! ## Notes
//!
//! The explanation above assumes one-based indexing, however, in Rust, as in most other programming
//! languages, indexing is zero-based.
//! For the sake of code simplicity and performance, the following changes were made:
//!     - querying is one-based:  `i & (i - 1)` (or `i - (i & (-i))`)
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
/// assert_eq!(tree.sum(0..3), 6);
///
/// ```
///
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

    /// A size of the tree.
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

/// Converts first trailing `1` bit into `0` in the binary representation of the `i`.
///
/// `0001` -> `0000`
/// `0010` -> `0000`
/// `0011` -> `0010`
/// `1010` -> `1000`
///
/// Applying `i = prev(i)` allows traversing the array from the initial `i` to `0` by N steps,
///     where N -- the number of `1`s in the binary representation of the initial `i`.
///
/// For example, traversing from `1111` involves the following sequence of indices:
///     `1111` (15) -> `1110` (14) -> `1100` (12) -> `1000` (8) -> `0000` (0).
///
/// Considering the way (see `FenwickTree` docs) in which sums are stored in the tree, this allows
/// fast calculating of prefix sums:
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

/// Converts first trailing `0` bit into `1` in the binary representation of the `i`.
///
/// `0000` -> `0001`
/// `0010` -> `0011`
/// `0011` -> `0111`
/// `1010` -> `1011`
///
/// In the same way as with `prev`, `i = next(i)` allows traversing the array but in the opposite
/// direction.
///
/// For example, traversing from `0000` to `1111` involves the following sequence of indices:
///     `0000` (0) -> `0001` (1) -> `0011` (3) -> `0111` (7) -> `1111` (15).
///
/// This allows to update the value at `i` and all cumulative sums that contains the `i`:
///     - call `i = next(i)` until `i` is less than length of the array
///     - access sums by `i`
///
/// Unlike `prev`, this function assumes that indexing is zero-based, hence we access sums by `i`.
const fn next(i: usize) -> usize {
    i | (i + 1)
}
