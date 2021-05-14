use crate::*;

#[test]
fn sum_of_empty_range_is_err() {
    let tree = new_tree(3);

    let err = tree.sum(0..0).err().unwrap();

    assert_eq!(SumError::EmptyRange { start: 0 }, err);
}

#[test]
fn sum_of_decreasing_range_is_err() {
    let tree = new_tree(3);

    let err = tree.sum(10..0).expect_err("");

    assert_eq!(SumError::DecreasingRange { start: 10, end: 0 }, err);
}

#[test]
fn adding_at_invalid_index_is_err() {
    let mut tree = new_tree(3);

    let err = tree.add(4, 0).expect_err("");

    assert_eq!(AddError::IndexOutOfRange { index: 4, size: 3 }, err);
}

#[test]
fn range_sum_is_calculated_correctly_for_big_tree() {
    let mut tree = new_tree(100);

    for i in 1..101 {
        tree.add(i - 1, i as i32).unwrap();
    }

    assert_eq!(tree.sum(0..100).unwrap(), 5050);
    assert_eq!(tree.sum(1..100).unwrap(), 5049);
    assert_eq!(tree.sum(2..100).unwrap(), 5047);
    assert_eq!(tree.sum(3..100).unwrap(), 5044);
    assert_eq!(tree.sum(4..100).unwrap(), 5040);
}

fn new_tree(size: usize) -> FenwickTree<i32> {
    FenwickTree::<i32>::with_len(size)
}
