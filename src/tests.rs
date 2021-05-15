use std::ops::Bound;

use crate::*;

#[test]
fn sum_of_empty_range_is_0() {
    let tree = new_tree(3);

    assert_eq!(tree.sum(0..0).unwrap(), 0);
}

#[test]
fn sum_of_decreasing_range_is_0() {
    let tree = new_tree(3);

    assert_eq!(tree.sum(2..0).unwrap(), 0);
}

#[test]
fn sum_of_range_with_too_big_lower_bound_is_err() {
    let tree = new_tree(3);

    let err = tree.sum(3..4).expect_err("");

    assert_eq!(
        SumError::OutOfRange {
            bounds: (Bound::Included(3), Bound::Excluded(4)),
            len: 3
        },
        err
    );
}

#[test]
fn sum_of_range_with_too_big_upper_bound_is_err() {
    let tree = new_tree(3);

    let err = tree.sum(0..=4).expect_err("");

    assert_eq!(
        SumError::OutOfRange {
            bounds: (Bound::Included(0), Bound::Included(4)),
            len: 3
        },
        err
    );
}

#[test]
fn adding_at_invalid_index_is_err() {
    let mut tree = new_tree(3);

    let err = tree.add(4, 0).expect_err("");

    assert_eq!(AddError::IndexOutOfRange { index: 4, size: 3 }, err);
}

#[test]
fn range_sum_is_calculated_correctly_for_range() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(0..3).unwrap(), 1 + 2 + 3);
}

#[test]
fn range_sum_is_calculated_correctly_for_range_from() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(0..).unwrap(), 1 + 2 + 3);
    assert_eq!(tree.sum(1..).unwrap(), 2 + 3);
}

#[test]
fn range_sum_is_calculated_correctly_for_range_to() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(..3).unwrap(), 1 + 2 + 3);
    assert_eq!(tree.sum(..2).unwrap(), 1 + 2);
}

#[test]
fn range_sum_is_calculated_correctly_for_range_full() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(..).unwrap(), 1 + 2 + 3);
}

#[test]
fn range_sum_is_calculated_correctly_for_range_inclusive() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(0..=2).unwrap(), 1 + 2 + 3);
    assert_eq!(tree.sum(0..=1).unwrap(), 1 + 2);
}

#[test]
fn range_sum_is_calculated_correctly_for_range_to_inclusive() {
    let tree = new_filled_tree(3);

    assert_eq!(tree.sum(..=2).unwrap(), 1 + 2 + 3);
    assert_eq!(tree.sum(..=1).unwrap(), 1 + 2);
}

#[test]
fn range_sum_is_calculated_correctly_for_custom_bounds() {
    let tree = new_filled_tree(3);

    assert_eq!(
        tree.sum((Bound::Included(0), Bound::Excluded(1))).unwrap(),
        1
    );

    assert_eq!(
        tree.sum((Bound::Excluded(0), Bound::Included(1))).unwrap(),
        2
    );
}

fn new_filled_tree(size: usize) -> FenwickTree<i32> {
    let mut tree = new_tree(size);

    for i in 1..=size {
        tree.add(i - 1, i as i32).unwrap();
    }

    tree
}

fn new_tree(size: usize) -> FenwickTree<i32> {
    FenwickTree::<i32>::with_len(size)
}
