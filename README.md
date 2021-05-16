# fenwick-tree

[![crates.io](https://img.shields.io/crates/v/fenwick-tree)](https://crates.io/crates/fenwick-tree)
[![Build](https://github.com/JoshuaLight/fenwick-tree-rs/workflows/Build/badge.svg)](https://github.com/JoshuaLight/fenwick-tree-rs/actions/workflows/build.yml)

_An implementation of the binary indexed tree (or Fenwick tree) data structure in Rust._

## Overview

`fenwick-tree` provides a simple implementation of the [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree) that can be used as a building block in some algorithms like weighted random.

The basic API is simple and consists of `add` and `sum` methods (both take _O_(log _n_) time). Here is a quick example:
```rust
use fenwick_tree::*;

let mut tree = FenwickTree::<i32>::with_len(5);

for i in 0..5 {
    tree.add(i, i as i32)?;
}

assert_eq!(tree.sum(..)?, 0 + 1 + 2 + 3 + 4);
assert_eq!(tree.sum(1..)?, 1 + 2 + 3 + 4);
assert_eq!(tree.sum(2..5)?, 2 + 3 + 4);
assert_eq!(tree.sum(3..=4)?, 3 + 4);
```

### Panics

Both `add` and `sum` methods return `Result` and are not expected to panic.
However, `with_len` constructs the backing vector by using `vec![I::default(); len]`, and it actually may panic as in regular Rust code.

## Learn more

For more details see [documentation](https://docs.rs/fenwick_tree).

## License

The package is licensed under the [MIT](https://github.com/JoshuaLight/fenwick-tree-rs/blob/master/LICENSE) license.

## Contributing

There are no strict rules for contributing. Feel free to open an issue or submit a pull request.
