mod tree;

pub use tree::FenwickTree;

// Some of the tests are implemented as documentation tests.
#[cfg(test)]
mod tests {
    use crate::FenwickTree;

    #[test]
    fn new_tree_has_correct_size() {
        let tree = new_tree(3);

        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn new_tree_total_sum_is_zero() {
        let tree = new_tree(3);

        assert_eq!(tree.sum(0..3), 0);
    }

    #[test]
    fn range_sum_is_calculated_correctly() {
        let mut tree = new_tree(3);

        tree.add(0, 1);
        tree.add(1, 2);
        tree.add(2, 3);

        assert_eq!(tree.sum(0..1), 1);
        assert_eq!(tree.sum(0..2), 3);
        assert_eq!(tree.sum(0..3), 6);

        assert_eq!(tree.sum(1..2), 2);
        assert_eq!(tree.sum(1..3), 5);

        assert_eq!(tree.sum(2..3), 3);
    }

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
