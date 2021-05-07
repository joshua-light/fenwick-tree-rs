use std::ops::{AddAssign, SubAssign};
use std::{fmt::Debug, ops::Range};

pub struct FenwickTree<I>
where
    I: Debug + Default + Copy + AddAssign + SubAssign,
{
    buf: Vec<I>,
}

impl<I> FenwickTree<I>
where
    I: Debug + Default + Copy + AddAssign + SubAssign,
{
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![I::default(); size],
        }
    }

    pub fn size(&self) -> usize {
        self.buf.len()
    }

    pub fn sum(&self, range: Range<usize>) -> I {
        let mut s = I::default();
        let mut i = range.start;
        let mut j = range.end;

        while j > i {
            s += self.buf[j - 1];
            j = prev(j);
        }

        while i > j {
            s -= self.buf[i - 1];
            i = prev(i);
        }

        s
    }

    pub fn add(&mut self, mut i: usize, delta: I) {
        while i < self.buf.len() {
            self.buf[i] += delta;
            i = next(i);
        }
    }
}

const fn prev(i: usize) -> usize {
    i & (i - 1)
}

const fn next(i: usize) -> usize {
    i | (i + 1)
}
