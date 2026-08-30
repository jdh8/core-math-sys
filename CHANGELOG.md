# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->

## [Unreleased]

### Added

- Bind the new binary128 functions `cr_acosq`, `cr_asinq`, and `cr_atanq` behind
  the `f128` feature.

### Changed

- Advance the vendored CORE-MATH submodule to upstream `d350cca6`, bringing
  correctness fixes to existing binary16, binary32, binary64, and binary128
  functions with no further API changes.

## [1.2.0] - 2026-07-11

### Added

- Bind `cr_atan2q`, the correctly rounded binary128 (`__float128`) two-argument
  arctangent, newly implemented upstream. Requires the `f128` feature. The
  vendored CORE-MATH submodule is advanced to upstream `8c7c00f6`, which also
  brings accuracy, proof, and worst-case-table improvements to many existing
  functions (`exp`, `tanh`, `log`, `sin`, `cos`, ...) with no API changes.

### Removed

- Drop the `anyhow` and `glob` build-dependencies; `build.rs` now uses the
  standard library instead. `anyhow::Result`/`.context(...)` become
  `Box<dyn std::error::Error>`/`.ok_or(...)`, and the `glob("…/*/")` directory
  walks become `std::fs::read_dir`. This has no effect on the crate's public API
  or generated bindings — only the build-time dependency graph shrinks by two
  crates. This also supersedes the not-yet-released `anyhow` minimal-version
  floor bump (#1): with `anyhow` gone, the pin is moot.

## [1.1.1] - 2026-06-13

### Fixed

- Fix the `f128` build with Clang 16 and newer (including the Clang 18 used in
  CI): glibc declares the `*f128` math functions only for GCC, the binary128
  sources call them in non-`cr_` alias wrappers (`sqrtq`, `expq`, …), and
  Clang 16 turned the resulting implicit declarations into hard errors. The
  build script now force-includes a small header declaring those functions
  when compiling with Clang. This also gives the alias wrappers correct
  prototypes on Clang 15, which previously compiled them against an implicit
  `int` return type that made the aliases (never the bound `cr_*` functions)
  silently unusable.

## [1.1.0] - 2026-06-13

### Added

- Export four correctly-rounded functions that were already compiled into the
  static library but lacked bindings: `cr_lgamma`, `cr_tgamma`, and `cr_sincos`
  (`f64`), plus `cr_compoundf` (`f32`). This closes the asymmetry where only the
  single-precision `cr_lgammaf`, `cr_tgammaf`, and `cr_sincosf` were available.
  The change is purely additive — only the bindings are new, so existing code is
  unaffected.
- New opt-in Cargo features exposing CORE-MATH functions on more types:
  - `f16` — all 43 binary16 functions (`cr_expf16`, `cr_sqrtf16`, …) typed
    with the unstable [`f16`] primitive. Needs nightly Rust and a C compiler
    with `_Float16` and `__builtin_roundeven` support (GCC 12+ or Clang 17+).
  - `f128` — the 9 binary128 functions CORE-MATH currently provides
    (`cr_cbrtq`, `cr_expq`, `cr_exp10q`, `cr_exp2q`, `cr_expm1q`, `cr_hypotq`,
    `cr_logq`, `cr_rsqrtq`, `cr_sqrtq`) typed with the unstable [`f128`]
    primitive. Needs nightly Rust and Clang 15+ or GCC 14+ (the C sources use
    `__builtin_addcl`).

  Both features are off by default, so the default build is unchanged and
  stays stable-compatible. bfloat16 (`half::bf16`) support was evaluated and
  deferred: C `__bf16` is passed in floating-point registers while
  `half::bf16` has integer-register ABI, so bindings would need C shims, and
  `__bf16` arithmetic requires GCC 13+ or Clang 17+.

[`f16`]: https://doc.rust-lang.org/std/primitive.f16.html
[`f128`]: https://doc.rust-lang.org/std/primitive.f128.html

### Changed

- Adopt the Rust 2024 edition and declare `rust-version = "1.85"` (enforced by
  a new CI job). The practical impact is small: the effective MSRV was already
  ≥ 1.82 because bindgen 0.72 emits `unsafe extern` blocks in the generated
  bindings. Declaring the MSRV means toolchains older than 1.85 now get a
  clear Cargo error (and MSRV-aware dependency resolution) instead of a
  confusing syntax error.

## [1.0.3] - 2026-06-10

### Changed

- Add `links = "core_math"` to `[package]` in `Cargo.toml` so Cargo
  recognizes this crate as the native-link provider and lib.rs no longer
  reports the `*-sys crate without links property` warning.

## [1.0.2] - 2026-06-09

### Changed

- Track the official CORE-MATH repository again. The vendored submodule now
  points at `https://gitlab.inria.fr/core-math/core-math.git` (branch `master`)
  instead of a personal fork, and is advanced to the latest upstream commit.
  The fork existed only to carry an `issignaling` compatibility patch, which
  upstream has since fixed by renaming the symbol to `is_signaling`; the fork
  is therefore obsolete. No change to the public API.

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

[1.2.0]: https://github.com/jdh8/core-math-sys/releases/tag/1.2.0
[1.1.1]: https://github.com/jdh8/core-math-sys/releases/tag/1.1.1
[1.1.0]: https://github.com/jdh8/core-math-sys/releases/tag/1.1.0
[1.0.3]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.3
[1.0.2]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.2
[1.0.1]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.1
[1.0.0]: https://github.com/jdh8/core-math-sys/releases/tag/1.0.0
[0.2.0]: https://github.com/jdh8/core-math-sys/releases/tag/0.2.0
