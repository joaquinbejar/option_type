# Contributing to `option_type`

Thanks for your interest in improving `option_type`. This crate provides
enum-based classification of financial option contracts (standard and exotic)
with a compact, allocation-free public surface. Its core design goal is to
**make invalid states unrepresentable at the type level**, so contributions are
held to a strict, semver-aware bar.

All code, comments, docs, commit messages, and PR descriptions must be in
**English**.

The crate enforces its coding rules mechanically: the `[lints.rust]` and
`[lints.clippy]` blocks in `Cargo.toml` (deny `unsafe_code`, `unwrap_used`,
`expect_used`, `panic`) and the CI matrix are the source of truth. If a change
needs a lint weakened to compile, the change is wrong, not the lint.

## Branching model

- `main` is the only long-lived branch. All work happens on short-lived
  feature branches cut from `main`.
- Name branches `issue-<n>-<slug>`, e.g. `issue-13-contributing-security`.
  Every branch maps to exactly one tracking issue.
- **One issue per pull request.** Keep PRs focused; do not bundle unrelated
  changes.
- The PR body must end with `Closes #<n>` so the issue closes on merge.
- PRs are integrated with **merge commits** (no squash, no rebase-merge), which
  preserves the `Merge pull request #<n> from <branch>` history you see in
  `git log --merges`.

## Design discussion happens in issues

Discuss the design, scope, and trade-offs of a change **in the issue**, before
or alongside the PR — not in PR review comments. Open (or claim) an issue first;
the PR implements a decision the issue already reached. This keeps the rationale
discoverable and out of ephemeral review threads.

## Commit conventions

- Subject line in the **imperative mood**: "Add barrier rebate field", not
  "Added" or "Adds".
- The body explains **why** the change is made, not just what changed — the
  diff already shows what.
- English only. One logical change per commit where practical.

## Pre-submission gate

Run the full check gate before pushing:

```bash
make pre-push
```

`make pre-push` is a **pure check gate** (it never auto-fixes) and runs, in
order:

1. `fmt-check` — `cargo +stable fmt --all --check`
2. `lint` — `cargo clippy --all-targets --all-features -- -D warnings`
3. `test` — `cargo test --all-features`
4. `test-default` — `cargo test --no-default-features`
5. `release` — `cargo build --release`
6. `doc-check` — `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
7. `readme-check` — verifies `README.md` is in sync with the generated output

The two-configuration test matrix (`--all-features` and
`--no-default-features`) is mandatory whenever the public surface is touched.

Discover every available target with:

```bash
make help
```

The auto-fixers are **not** part of `pre-push` — run them yourself as needed:

- `make fmt` — format the tree with `rustfmt`.
- `make lint-fix` — apply `clippy --fix` suggestions.

## README is generated — never hand-edit it

`README.md` is **generated** from the `src/lib.rs` crate docs plus the static
sections in `README.tpl`, via `cargo-readme`. Do not edit `README.md` by hand.

To change README content:

- For the API narrative / body: edit the crate docs in `src/lib.rs`.
- For the static sections (badges, Examples, MSRV, Releasing, Contribution and
  Contact, License): edit `README.tpl`.

Then regenerate and commit the result:

```bash
make readme
```

CI (`.github/workflows/format_check.yml`) runs `make readme-check` and fails the
build if `README.md` has drifted from the generated output.

## CHANGELOG

Every PR adds an entry under the `## [Unreleased]` section of
[`CHANGELOG.md`](./CHANGELOG.md), following
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (`Added`, `Changed`,
`Fixed`, etc.). The `[Unreleased]` section is promoted to a dated version at
release time.

## Semver discipline

This crate is consumed by downstream pricing and analytics code, and its public
enums are the whole product. Treat the following as the semver contract:

- Every `pub` enum variant, struct field, and method signature.
- `#[repr(u8)]` on the leaf sub-enums (`AsianAveragingType`, `BarrierType`,
  `BinaryType`, `LookbackType`, `RainbowType`) — it is a 1-byte-layout guarantee
  callers rely on. Removing it is a major break.
- The tag and payload shape of every serialized variant (on-disk / on-wire
  compatibility).

Adding a variant to a non-`#[non_exhaustive]` enum, or renaming/removing any
variant, field, or method, is a **breaking change** requiring a major version
bump (or a pre-1.0 minor bump). `cargo semver-checks` runs in CI
(`.github/workflows/semver.yml`, `--all-features`) and will flag violations.
Never introduce a catch-all `Unknown` / `Other` variant.

## MSRV policy

The Minimum Supported Rust Version is currently **1.85**, declared as
`rust-version` in `Cargo.toml` and enforced by
`.github/workflows/msrv.yml`.

Bumping the MSRV is a **breaking change** (major version bump, or pre-1.0 minor
bump). When you bump it, update all three of:

1. `rust-version` in `Cargo.toml`,
2. the pinned toolchain in `.github/workflows/msrv.yml`,
3. the MSRV section in the README (edit `README.tpl` and run `make readme`).

## Pre-submission checklist

Before you open a PR:

- [ ] `make pre-push` is clean (includes both feature configurations and
      `readme-check`).
- [ ] `CHANGELOG.md` has an entry under `[Unreleased]`.
- [ ] The PR maps to one issue and its body ends with `Closes #<n>`.
- [ ] No `.unwrap()` / `.expect()` / `panic!` / unchecked indexing in `src/`
      outside `#[cfg(test)]`; no `println!` / `eprintln!` / `dbg!` / `log` in
      `src/`; no `unsafe`.
- [ ] Semver impact is understood and reflected in the version and changelog.

## Contact

Questions that are not design discussion for a specific issue can go to the
maintainer: Joaquín Béjar García — <jb@taunais.com>.
