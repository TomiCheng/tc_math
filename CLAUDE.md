# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

- Comments and documentation are written in English only — doc comments,
  README, changelog, and inline comments alike.
- Never wire a README into rustdoc (`#![doc = include_str!("../README.md")]`).
  The README's badges and relative links do not survive rustdoc rendering.
  Crate documentation lives in `//!` and `///` comments; the README repeats what
  a reader on crates.io needs.
- No breaking changes. Public API work is additive: adding items, trait
  implementations, or default methods. If a change cannot be made additively,
  stop and raise it rather than altering an existing signature or behavior.
- Every crate README opens with the badge block: crates.io, docs.rs, CI,
  license, and rustc.
- Crate READMEs are short and written for crates.io: the badge block, a
  sentence or two on what the crate is, the "Types", "Traits" and "Features"
  lists, a usage example of ten lines or fewer, a short security note and the
  license. Behaviour belongs in the rustdoc, and validation and release
  commands in the root README.
- Crate READMEs use no Markdown tables; crates.io renders them badly. Traits,
  types and features are flat one-line bullets (`` `Item` — what it does. ``). Benchmark results
  and how to reproduce them live in the crate's `BENCHES.md`, which is in the
  `include` list, and the README links to it. `BENCHES.md` is read on GitHub,
  so its results may use tables.

The crate list and workspace-wide checks live in the root
[README.md](README.md); read it rather than restating it here. Every crate is
`no_std` and needs no allocator by default.

`tc_bigint` holds big integers in three kinds, each unsigned and signed:
`FixedBigUint<N>` and `FixedBigInt<N>` of `N` limbs on the stack,
`PaddedBigUint` and `PaddedBigInt` at a width fixed when built, and `BigUint`
and `BigInt` at arbitrary precision, over `Limb`, `LimbArray<N>` and `Word`,
which is `u64` on 64-bit targets and `u32` otherwise. `N` counts limbs, not
bits. The signed types are two's complement. A padded binary operation
extends the narrower operand to the wider width and judges overflow against
it. The `alloc` feature adds the padded and big types, and `rand_core` adds
`RandomBits` and `RandomRange`; both are default-off. It depends on
`num-traits` without default features, `tc_constant_time`, `tc_zeroize` and,
with `rand_core`, `rand_core`, on every target, and CI enforces that set with
`cargo tree` on the `wasm32-unknown-unknown`, `aarch64-unknown-none` and x86
targets.

The operators panic on overflow in every build, as the primitive integers do
in debug builds; the `num-traits` checked, wrapping, saturating and
overflowing forms are the alternatives. The fixed-width and padded types are
constant time in their values; widths, shift amounts and exponents are public.
The big types are variable time, only for public values, and each names the
padded type that takes secret ones. `tests/constant_time.rs` scans every file
under `src` and requires each public function and trait implementation to
say "Constant time" or "Variable time" in its doc, each variable-time one to
say it is only for public values, and constant-time code to call nothing that
only variable-time items define. Keep the timing contract of each item stated
in its doc comment. The constant-time types wipe through volatile writes the
working values an operation gives up; the values they return are the
caller's to wipe, and none of them has a `Drop` of its own. `Debug` on `Limb`
never prints the word.

`unsafe` code is forbidden at the crate root.

Rust 1.85 is guaranteed for every build. Besides `tc_*` crates, the library
depends on `num-traits` and, behind its feature, `rand_core`, and both build
on 1.85; dev-dependencies such as `criterion` are exempt. The MSRV job
therefore runs `cargo check` on 1.85 with and without features; tests run on
stable. `.cargo/config.toml` sets `incompatible-rust-versions = "allow"` so
`Cargo.lock` tracks the latest releases and stable CI tests what current
toolchains resolve. Adding another third-party dependency to a default build
or a first-party feature hands the 1.85 guarantee to that crate; raise it
before doing so.

A crate depends on a workspace sibling through `path` plus `version`, so the
workspace builds and tests against the local crate while the published package
requires the release. When a change needs a sibling API that is not released
yet, raise the `version` requirement to the release that adds it; that release
has to be published first.

## Conventions

Each crate ships its own `README.md`, `CHANGELOG.md`, `LICENSE-MIT`,
`LICENSE-APACHE`, and an explicit `include` list in `Cargo.toml`. Changelog
entries are written as `## <version> - Unreleased` and dated in a separate
commit at release, with `### Added` and `### Compatibility` sections.

Adding a crate to the workspace means five edits beyond the crate itself: the
`members` list, a `-p <crate>` on the single `cargo package --locked` step in
the CI `quality` job (the only place package archives are verified; packaging
the crates in one invocation checks each against its siblings' local sources
rather than their releases), a `cargo tree` check of its dependency set in the
CI `portable` job, a row in the root `README.md`, and
workspace inheritance for `edition`, `rust-version`, `license`, and
`repository`. A missing `rust-version` also leaves clippy suggesting APIs newer
than 1.85.

Documentation is part of the contract: crates use `#![deny(missing_docs)]`,
doctests carry the executable examples, and CI runs `cargo doc` with
`RUSTDOCFLAGS: -D warnings`, with and without `--all-features`. Doc links to
feature-gated items break the build without that feature, so name them in plain
code spans. An additive public API change belongs in the crate README's
contract lists — "Types", "Traits" and "Features" in
`tc_bigint/README.md` — and in the changelog, not only in the code.

Work happens on `feat/*` branches off `develop`; pull requests target `develop`,
which merges to `main`. Commit messages use an imperative subject and a wrapped
body that explains the reasoning, not a bullet list of the diff.

Note: the root `Cargo.toml` uses CRLF line endings while the rest of the tree
uses LF. Tools that rewrite whole files will flip it and produce a noisy diff.
