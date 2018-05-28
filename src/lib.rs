// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! A union-find data structure contains a partition of a
//! contiguous bounded set of integers. It supports two operations.
//! The *union* operation merges two partitions. The *find* operation
//! identifies the partition containing an element by returning a
//! "canonical" element of the partition as a label. Both operations
//! are fast.
//!
//! # Examples
//! ```
//! use union_find::UnionFind;
//! const N: usize = 20;
//! 
//! let mut parity = UnionFind::new(N);
//! for i in 0..N {
//!     if i & 1 == 0 {
//!         parity.union(0, i)
//!     } else {
//!         parity.union(1, i)
//!     }
//! };
//! for i in 0..N {
//!     let c = parity.find(i);
//!     assert_eq!(c, i & 1)
//! }
pub struct UnionFind {
    parts: Vec<usize>
}

impl UnionFind {
    /// Create a new partition table with `n` disjoint
    /// partitions numbered 0..`n`. Running time O(n).
    pub fn new(n: usize) -> UnionFind {
        let mut parts = Vec::with_capacity(n);
        for i in 0..n {
            parts.push(i)
        };
        UnionFind{ parts: parts }
    }

    /// Merge the partitions containing `i` and `j`.  This
    /// operation is structured such that the canonical
    /// element of the merged partition will be the
    /// canonical element of `i` in the old
    /// partition. Running time O(1).
    pub fn union(&mut self, i: usize, j: usize) {
        self.parts[j] = self.parts[i];
    }

    /// Return a "canonical element" for the partition
    /// containing `i`. Amortized worst-case running time
    /// O(alpha(n)), which is more-or-less O(1). This running
    /// time is achieved by mutating the data structure to
    /// make future `find()` operations faster.
    pub fn find(&mut self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        };
        let mut s = i;
        while s != p {
            let t = self.parts[s];
            self.parts[s] = p;
            s = t
        };
        p
    }

    /// Return a "canonical element" for the partition
    /// containing `i`. Worst-case running time O(n), found
    /// in cases involving a sequence of `union()` and
    /// `find_only()` operations with few `find()`
    /// operations. Prefer `find()` in any case where it is
    /// possible to make the table mutable.
    pub fn find_only(&self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        };
        p
    }

    /// Return `true` iff i and j are in the same partition.
    /// Time complexity is the same as `find()`.
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    /// Return `true` iff i and j are in the same partition.
    /// Time complexity is the same as `find_only()`. Prefer
    /// `same()` in any case where it is possible to make
    /// the table mutable.
    pub fn same_only(&self, i: usize, j: usize) -> bool {
        self.find_only(i) == self.find_only(j)
    }
}
