# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->

## [1.0.1] - 2026-05-23

### Documentation

- Reformat `README.md`.
- Add `CLAUDE.md` with a contributor checklist (format, test, update
  CHANGELOG, write a descriptive commit message).

### Dependencies

- Update `bindgen`, `cc`, `glob`, and `anyhow` build-dependencies.

## [1.0.0] - 2025-08-03

### Changed

- Switch the vendored CORE-MATH submodule to a personal fork to guarantee
  long-term API compatibility regardless of upstream churn.

### Fixed

- Rename the internal use of `issignaling` to avoid collision with the
  OS-provided macro on platforms where libm already defines it.

### Dependencies

- Bump build-dependencies (`bindgen`, `cc`, `glob`).

## [0.2.0] - 2024-08-27

### Added

- Expose additional CORE-MATH functions, and re-export functions that had
  been hidden during earlier iterations of the build.
- `no-std` category in `Cargo.toml`.
- Linkage sanity test.

### Changed

- Optional native-code build when the `TARGET_CPU` environment variable
  is provided; otherwise build for a portable baseline.

### Fixed

- Prototype of `cr_sincosf`.
- Linkage of `signgam`.
- Build on macOS.
- Fall back to the system `libm` for math functions that CORE-MATH does
  not implement, instead of failing to link.

### Documentation

- Update Rust crate docs.

### Tooling

- GitHub Actions workflow `rust.yml`, with follow-up fixes for
  cross-platform CI.

### Internal

- Refactor `build.rs`.

(Earlier scaffolding commits — initial commit, "Try building CORE-MATH",
"Try minimalizing everything", "Build the library with plain C" — are
rolled up into this entry as pre-public iterations.)

[1.0.1]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.1
[1.0.0]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.0
[0.2.0]: https://github.com/jdh8/core-math-sys/releases/tag/0.2.0
