# fenwick-tree

_An implementation of a binary indexed tree (or Fenwick tree) data structure in Rust._

## Overview

`fenwick-tree-rs` is a simple yet useful implementation of the Fenwick tree that can be used as a building block in some of the algorithms (e.g. weighted random).

The basic API is simple and consists of `add` and `sum` methods. Here is a quick example:
```rust
use fenwick_tree::FenwickTree;

let mut tree = FenwickTree::<i32>::with_len(5);

for i in 0..5 {
    tree.add(i, i as i32)?; // O(log n).
}

// `sum` takes O(log n) as well.
assert_eq!(tree.sum(0..5)?, 0 + 1 + 2 + 3 + 4);
assert_eq!(tree.sum(2..5)?, 2 + 3 + 4);
```

## Learn more

For more details see [documentation](https://docs.rs/fenwick_tree/0.1.0/fenwick_tree).

## License

The package is licensed under the [MIT](https://github.com/JoshuaLight/fenwick-tree-rs/blob/master/LICENSE) license.
