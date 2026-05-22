# core-math-sys

This crate generates Rust FFI bindings to [CORE-MATH](https://core-math.gitlabpages.inria.fr/).
CORE-MATH is a correctly rounded mathematical library in C.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.

After updating the codebase, please

- Format the code with `cargo fmt`.
- Run the tests with `cargo test --all-features`.
- Update [CHANGELOG.md](CHANGELOG.md) with a summary of the changes and their impact on users.
- Propose a clear and descriptive commit message.
