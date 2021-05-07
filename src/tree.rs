use std::ops::{AddAssign, SubAssign};
use std::{fmt::Debug, ops::Range};

/// An implementation of the binary indexed tree (Fenwick tree) data structure.
///
/// The tree is backed by a simple array/vec of the fixed size where each item is
/// responsible for storing cumulative sum of some range, allowing to perform
/// queries and updates in O(log n) time.
///
/// # Examples
///
/// ```
/// # use crate::tree::FenwickTree;
///
/// let tree = FenwickTree::<i32>::of_size(10);
///
/// ```
///
/// # Explanation
///
/// ## Storage
///
/// The easiest explanation considers that indexing of the array is one-based (as in the
/// original paper).
///
/// Sums are stored in the array by the following rule: an item with index `i` stores a cumulative sum
/// of `[i - (i & (-i)), i]` range.
///
/// This looks a bit scary but the intuition behind is very simple: the position of the first
/// set bit in the `i` equals to the size of the range covered by `i`.
/// For example:
///     - `i = 4` is `100`, so it covers `4` items or simply `[1, 4]` range (note one-based indexing)
///     - `i = 3` is `011`, so it covers `1` item  or simply `[3, 3]` range
///     - `i = 2` is `010`, so it covers `2` items or simply `[1, 2]` range
///     - `i = 1` is `001`, so it covers `1` item  or simply `[1, 1]` range
///
/// Prefix sum of `n` items consists of sum of these ranges, depending on `n`.
/// For example:
///     - `n = 4` is sum of `[1, 4]` (4 index)
///     - `n = 3` is sum of `[3, 3]` (3 index) and `[1, 2]` (2 index)
///     - `n = 2` is sum of `[1, 2]` (2 index)
///     - `n = 1` is sum of `[1, 1]` (1 index)
///
/// ## Querying
///
/// In order to calculate the prefix sum, we need to traverse the array from right
/// to left, decreasing `i` by the size of the range it covers.
/// For example:
///     - `i = 4` -> `0`
///     - `i = 3` -> `2`
///     - `i = 2` -> `0`
///     - `i = 1` -> `0`
///
/// So, what is `i - (i & (-i))`?
/// This operation decreases `i` by some step that is equal to `i & (-i)`.
/// In two's complement systems `-i` means inverting the `i` and adding `1`:
/// ```
/// # fn any_number() -> i32 { 4 }
///
/// let i = any_number();
/// assert_eq!(!i + 1, -i);
/// ```
/// The behaviour of "adding `1`" is flipping all trailing set bits and flipping first unset bit.
/// For example:
///     - `i = 000`, `!i = 110`, `-i = 111`
///     - `i = 001`, `!i = 101`, `-i = 110`
///     - `i = 011`, `!i = 100`, `-i = 101`
///     - `i = 100`, `!i = 011`, `-i = 100`
///
/// Hence, `i & (-i)` extracts first set bit from the `i`.
/// For example:
///     - `i = 000`, `i & (-i) = 001 & 111 = 001`
///     - `i = 001`, `i & (-i) = 010 & 110 = 010`
///     - `i = 011`, `i & (-i) = 011 & 101 = 001`
///     - `i = 100`, `i & (-i) = 100 & 100 = 100`
///
/// Thereby, decreasing `i` by `i & (-i)` means flipping the first set bit in `i`.
/// We know that this bit is responsible for the size of the range covered by `i`.
/// Moreover, flipping the bit means subtracting the size from the `i`.
/// Thereby, iteration `i = i & (i - 0)` while `i > 0` will traverse the
/// array in the same way as needed for prefix sum calculation.
///
/// ## Updating
///
/// In the same way updating a value at `i` means traversing the array from left to right, updating
/// cumulative sums that contain value at `i`.
/// For example:
///     - `i = 4` updates ranges `[1, 4]`
///     - `i = 3` updates ranges `[3, 3]`, `[1, 4]`
///     - `i = 2` updates ranges `[1, 2]`, `[1, 4]`
///     - `i = 1` updates ranges `[1, 1]`, `[1, 2]`, `[1, 4]`
///
/// Here the intuition is the same: we need to increase `i` by the size of the range it covers.
/// For example:
///     - `i = 4` -> `8`
///     - `i = 3` -> `4`
///     - `i = 2` -> `4`
///     - `i = 1` -> `2`
///
/// In binary format this looks like:
///     - `i = 4 = 0100` -> `1000`
///     - `i = 3 = 0011` -> `0100`
///     - `i = 2 = 0010` -> `0100`
///     - `i = 1 = 0001` -> `0010`
///
/// Following the symmetry, if `i - (i & (-i))` decreases `i`, then `i + (i & (-i))` should
/// increase it, allowing to iterate from left to right.
/// `i + (i & (-i))` means adding the first set bit to `i`.
/// For example:
///     - `i = 4 = 0100`, `i + (i & (-i)) = 0100 + 0100 = 1000`
///     - `i = 3 = 0011`, `i + (i & (-i)) = 0011 + 0001 = 0100`
///     - `i = 2 = 0010`, `i + (i & (-i)) = 0010 + 0010 = 0100`
///     - `i = 1 = 0001`, `i + (i & (-i)) = 0001 + 0001 = 0010`
///
/// That's it!
///
/// ## Notes
///
/// The explanation above assumes one-based indexing, however, in Rust, as in other programming
/// languages, indexing is zero-based. For the sake of code simplicity and performance, the
/// following changes were made:
///     - querying is one-based:  `i & (i - 1)` (or `i - (i & (-i))`)
///     - updating is zero-based: `i | (i + 1)
pub struct FenwickTree<I>
where
    I: Debug + Default + Copy + AddAssign + SubAssign,
{
    tree: Vec<I>,
}

impl<I> FenwickTree<I>
where
    I: Debug + Default + Copy + AddAssign + SubAssign,
{
    pub fn of_size(size: usize) -> Self {
        Self {
            tree: vec![I::default(); size],
        }
    }

    pub fn size(&self) -> usize {
        self.tree.len()
    }

    pub fn sum(&self, range: Range<usize>) -> I {
        let mut s = I::default();
        let mut i = range.start;
        let mut j = range.end;

        while j > i {
            println!("{:?}", j);
            s += self.tree[j - 1];
            j = prev(j);
        }

        while i > j {
            s -= self.tree[i - 1];
            i = prev(i);
        }

        s
    }

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
