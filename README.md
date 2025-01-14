# Out-of-Bounds Vector Access in Rust

This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  This leads to a runtime panic.

## The Bug

The `bug.rs` file contains code that attempts to access the third element of a vector that only contains two elements. This will cause a panic because the index 2 is out of bounds.

## The Solution

The `bugSolution.rs` file shows how to correctly handle potential out-of-bounds access using `get()` method which returns an Option.