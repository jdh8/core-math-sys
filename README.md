# core-math-sys

[![Crates.io](https://img.shields.io/crates/v/core-math-sys.svg)](https://crates.io/crates/core-math-sys)
[![Documentation](https://docs.rs/core-math-sys/badge.svg)](https://docs.rs/core-math-sys)
[![Build status](https://github.com/jdh8/core-math-sys/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/core-math-sys)

Generated bindings to [CORE-MATH](https://core-math.gitlabpages.inria.fr/)

CORE-MATH is a correctly rounded mathematical library in C.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.
Meanwhile, its speed is competitive with the most popular C libraries, even
faster most of the time.

This crate provides the raw bindings to the CORE-MATH library.  It is
recommended to use the [core-math](https://crates.io/crates/core-math) crate
directly instead, which provides a safe Rusty interface.

## Cargo features

The default build exposes the `f32` and `f64` functions and works on stable
Rust.  Functions on more exotic types are opt-in because their Rust primitives
are unstable and their C types need a recent compiler:

- `f16` — binary16 functions (`cr_*f16`, 43 functions).  Requires nightly Rust
  for the unstable [`f16`](https://doc.rust-lang.org/std/primitive.f16.html)
  primitive, and GCC&nbsp;12+ or Clang&nbsp;17+ (the C sources need `_Float16`
  and `__builtin_roundeven`).
- `f128` — binary128 functions (`cr_*q`, 9 functions).  Requires nightly Rust
  for the unstable [`f128`](https://doc.rust-lang.org/std/primitive.f128.html)
  primitive, and Clang&nbsp;15+ or GCC&nbsp;14+ (the C sources use
  `__builtin_addcl`).
