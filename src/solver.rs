#![allow(unused_imports)]
#![allow(dead_code)]
// libraries: https://github.com/White-Green/competitive_programming_library_rs

use ac_library::{BitwiseXor, LazySegtree, MapMonoid, Min, ModInt998244353, Monoid, SccGraph, Segtree};
use bitvec::macros::internal::funty::Integral;
use itertools::Itertools;
use libraries::*;
use petgraph::visit::Walker;
use rand::Rng;
use smallvec::SmallVec;
use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::ops::{Bound, Range};
use std::rc::{Rc, Weak};
use std::sync::atomic::AtomicUsize;
use std::thread::current;
use std::{convert, iter, mem};
use tap::Tap;
// ac-library-rsにあるので実装しなくてよいものリスト
// convolution
// Dsu
// FenwickTree
// LazySegtree
// MapMonoid
// crt
// floor_sum
// inv_mod
// pow_mod
// Edge
// MfGraph
// MinCostFlowGraph
// ModInt
// SccGraph
// Segtree
// lcp_array
// suffix_array
// z_algorithm
// TwoSat

pub fn solve(InputObject { cin }: InputObject) {
}

pub struct InputObject<'a> {
    cin: &'a mut ConsoleInput,
}

fn input(cin: &mut ConsoleInput) -> InputObject<'_> {
    InputObject { cin }
}

pub fn main() {
    let mut cin = ConsoleInput::new();
    let input = input(&mut cin);
    solve(input);
}

// libraries below
mod libraries {
    use std::cmp::Ordering;
    use std::collections::{HashMap, VecDeque};
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;
    use std::ops::{Deref, Range};
    use std::str::FromStr;
    use std::{fmt, io};

    pub struct ConsoleInput {
        queue: VecDeque<String>,
    }

    pub trait Input {
        fn next<T: FromStr>(&mut self) -> T;
        fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }
    }

    pub trait Output {
        fn write(&self);
    }

    impl Output for () {
        fn write(&self) {}
    }

    impl Input for ConsoleInput {
        fn next<T: FromStr>(&mut self) -> T {
            match self.read().parse() {
                Ok(value) => value,
                _ => panic!("parse error"),
            }
        }
    }

    impl ConsoleInput {
        pub fn new() -> Self {
            Self { queue: VecDeque::new() }
        }

        fn read(&mut self) -> String {
            loop {
                if let Some(s) = self.queue.pop_front() {
                    return s;
                }
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("failed to read");
                for s in s.split_ascii_whitespace() {
                    self.queue.push_back(String::from(s));
                }
            }
        }
    }

    pub fn binary_search(range: Range<usize>, mut f: impl FnMut(usize) -> Ordering) -> Result<usize, usize> {
        let mut start = range.start;
        let mut len = range.len();
        if len == 0 {
            return Err(start);
        }
        while len > 1 {
            let half = len / 2;
            let mid = start + half;
            if f(mid) != Ordering::Greater {
                start = mid;
            }
            len -= half;
        }
        match f(start) {
            Ordering::Equal => Ok(start),
            Ordering::Less => Err(start + 1),
            Ordering::Greater => Err(start),
        }
    }

    pub fn ternary_search_min(range: Range<f64>, mut f: impl FnMut(f64) -> f64) -> f64 {
        let mut left = range.start;
        let mut right = range.end;
        while right - left > 1e-8 {
            let keys = [left + (right - left) / 3.0, left + (right - left) * 2.0 / 3.0];
            let values = keys.map(|x| f(x));
            if values[0] < values[1] {
                right = keys[1];
            } else {
                left = keys[0];
            }
        }
        f(left)
    }

    pub fn all_bits() -> impl Iterator<Item = impl Iterator<Item = (usize, bool)>> {
        (0usize..).map(|i| (0usize..).map(move |j| (j, (i >> j) & 1 != 0)))
    }

    pub struct AsciiString {
        inner: Vec<u8>,
    }

    impl Debug for AsciiString {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Debug::fmt(str::from_utf8(&self.inner).unwrap(), f)
        }
    }

    impl Display for AsciiString {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Display::fmt(str::from_utf8(&self.inner).unwrap(), f)
        }
    }

    impl Deref for AsciiString {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct NotAsciiString;

    impl FromStr for AsciiString {
        type Err = NotAsciiString;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.is_ascii() {
                return Err(NotAsciiString);
            }
            Ok(AsciiString { inner: s.as_bytes().to_vec() })
        }
    }

    #[derive(Clone)]
    pub struct MultiSet<T> {
        inner: HashMap<T, usize>,
        len: usize,
    }

    impl<T: Hash + Eq> MultiSet<T> {
        pub fn new() -> MultiSet<T> {
            MultiSet { inner: HashMap::new(), len: 0 }
        }

        pub fn insert(&mut self, value: T) {
            *self.inner.entry(value).or_default() += 1;
            self.len += 1;
        }

        pub fn remove(&mut self, value: T) {
            match self.inner.entry(value) {
                ::std::collections::hash_map::Entry::Occupied(mut entry) => {
                    match entry.get_mut() {
                        &mut 1 => {
                            entry.remove();
                        }
                        count => *count -= 1,
                    }
                    self.len -= 1;
                }
                ::std::collections::hash_map::Entry::Vacant(_) => {}
            }
        }

        pub fn len_types(&self) -> usize {
            self.inner.len()
        }

        pub fn len_all(&self) -> usize {
            self.len
        }
    }
}

#[cfg(test)]
mod tests;
