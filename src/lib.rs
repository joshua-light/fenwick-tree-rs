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
