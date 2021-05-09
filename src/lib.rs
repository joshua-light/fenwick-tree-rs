//! The crate contains a binary indexed tree ([Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree)) implementation and some of the extensions for it.
//!
//! The general idea of the Fenwick tree is that it allows for both queries and updates of partial
//! sums to be performed in _O_(log _n_) time.
//!
//! # Examples
//!
//! Constructing a new tree that is equal to `[1, 2, 3]` array.
//!
//! ```
//! use fenwick_tree::FenwickTree;
//!
//! let tree = FenwickTree::<i32>::of_size(3);
//!
//! // Both `add` and `sum` have complexity O(log n).
//! tree.add(0, 1); // Adds `1` to element at `0`.
//! tree.add(1, 2); // Adds `2` to element at `1`.
//! tree.add(2, 3); // Adds `3` to element at `2`.
//!
//! assert_eq!(tree.sum(0..1), 1);
//! assert_eq!(tree.sum(0..2), 3);
//! assert_eq!(tree.sum(0..3), 6);
//!
//! assert_eq!(tree.sum(1..2), 2);
//! assert_eq!(tree.sum(1..3), 5);
//!
//! assert_eq!(tree.sum(2..3), 3);
//! ```
//!
//! # Explanation
//!
//! In this section, an explanation about the tree implementation is provided.
//!
//! ## Indexing
//!
//! The easiest explanation considers that indexing of the array is one-based (as in the
//! [original paper](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.14.8917)).
//!
//! Sums are stored in the array by the following rule: an item with index `i` stores a cumulative sum
//! of `[i - g(i), i]` range, where `g(i) = i & (-i)` or the size of the range covered by `i`.
//!
//! This looks a bit scary but the whole intuition behind Fenwick tree is very simple:
//! the position of the first `1` bit in the `i` defines the value of `g(i)`.
//! * `i = 4 = 100`, `g(i) = 100 (4)` or `[1, 4]` range (note one-based indexing)
//! * `i = 3 = 011`, `g(i) = 001 (1)` or `[3, 3]` range
//! * `i = 2 = 010`, `g(i) = 010 (2)` or `[1, 2]` range
//! * `i = 1 = 001`, `g(i) = 001 (1)` or `[1, 1]` range
//!
//! ## Querying
//!
//! Prefix sum of `n` items consists of sum of some number of ranges, depending on `n`.
//! * `n = 4` sum of `[1, 4]` (4 index)
//! * `n = 3` sum of `[3, 3]` (3 index) and `[1, 2]` (2 index)
//! * `n = 2` sum of `[1, 2]` (2 index)
//! * `n = 1` sum of `[1, 1]` (1 index)
//!
//! In order to calculate a prefix sum, we need to traverse the array from right
//! to left, decreasing `i` by `g(i)`.
//! * `i = 100 (4)`, `i - g(i) = 000 (0)`
//! * `i = 011 (3)`, `i - g(i) = 010 (2)`
//! * `i = 010 (2)`, `i - g(i) = 000 (0)`
//! * `i = 001 (1)`, `i - g(i) = 000 (0)`
//!
//! So, how `g(i)` is calculated? This can easily be done by using the
//! representation of negative numbers in two's complement systems, where `-i` means `!i + 1`.
//!
//! ```
//! # fn any_number() -> i32 { 3 }
//! let i = any_number();
//! assert_eq!(!i + 1, -i);
//! ```
//!
//! In binary, `!i` inverts all bits in the `i`, and `!i + 1` flips all bits up to first `0`.
//! * `i = 001`, `!i = 110`, `-i = 111`
//! * `i = 010`, `!i = 101`, `-i = 110`
//! * `i = 011`, `!i = 100`, `-i = 101`
//! * `i = 100`, `!i = 011`, `-i = 100`
//!
//! This means that `i` and `-i` have all bits different except the first `1` bit.
//! So, `g(i)` is as simple as `i & (-i)`.
//!
//! ## Updating
//!
//! Updating a value at `i` means traversing the array from left to right, updating
//! ranges that contain the value.
//!
//! Here similar logic applies as for querying: we need to increase `i` by `g(i)`.
//! * `i = 4 = 100`, `i + g(i) = 1000 (8)`
//! * `i = 3 = 011`, `i + g(i) = 0100 (4)`
//! * `i = 2 = 010`, `i + g(i) = 0100 (4)`
//! * `i = 1 = 001`, `i + g(i) = 0010 (2)`
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
//! * querying is one-based:  `i & (i - 1)`
//! * updating is zero-based: `i | (i + 1)`
//!
//! # References
//! * [A New Data Structure for Cumulative Frequency Tables (1994)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.14.8917)

mod tree;

pub use tree::FenwickTree;

// Some of the tests are implemented as documentation tests.
#[cfg(test)]
mod tests {
    use crate::FenwickTree;

    #[test]
    fn range_sum_is_calculated_correctly_for_big_tree() {
        let mut tree = new_tree(100);

        for i in 1..101 {
            tree.add(i - 1, i as i32);
        }

        assert_eq!(tree.sum(0..100), 5050);
        assert_eq!(tree.sum(1..100), 5049);
        assert_eq!(tree.sum(2..100), 5047);
        assert_eq!(tree.sum(3..100), 5044);
        assert_eq!(tree.sum(4..100), 5040);
    }

    fn new_tree(size: usize) -> FenwickTree<i32> {
        FenwickTree::<i32>::of_size(size)
    }
}
