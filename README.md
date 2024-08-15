core-math-sys
=============
[![Build status](https://github.com/jdh8/core-math-sys/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/core-math-sys)
[![Crates.io](https://img.shields.io/crates/v/core-math-sys.svg)](https://crates.io/crates/core-math-sys)
[![Documentation](https://docs.rs/core-math-sys/badge.svg)](https://docs.rs/core-math-sys)

Generated bindings to [CORE-MATH](https://core-math.gitlabpages.inria.fr/)

CORE-MATH is a correctly rounded mathematical library in C.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.
Meanwhile, its speed is competitive with the most popular C libraries, even
faster most of the time.

This crate provides the raw bindings to the CORE-MATH library.  It is
recommended to use the [core-math](https://crates.io/crates/core-math) crate
directly instead, which provides a safe Rusty interface.