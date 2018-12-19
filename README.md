# UnionFind
Copyright &copy; 2017 Bart Massey

This Rust crate implements "union-find". A union-find
structure contains a collection of disjoint partitions.  It
supports two fundamental operations: *union* to merge two
partitions, and *find* to find a "canonical" partition
label. Both are fast.

Full Rustdoc documentation is provided, but you'll have to
build it yourself.

This is a work in progress:

* Currently limited to partitions of contiguous bounded
  integers.
* Minimal testing has been done.
* No benchmarking has been done.

This program is licensed under the "MIT License".  Please
see the file LICENSE in the source distribution of this
software for license terms.
