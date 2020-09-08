# template-rust

[![Build Status nightly][github/self/actions/badge]][github/self/actions]

[![Tag][github/self/tags/badge]][github/self/tags]
[![Crates.io][crates.io/self/badge]][crates.io/self]
[![Docs][docs.rs/self/badge]][docs.rs/self]

[![Changelog][github/self/blob/changelog/badge]][github/self/blob/changelog]
[![Last commit][github/self/last-commit/badge]][github/self/last-commit]

[![License][github/self/license/badge]][github/self/license]

Welcome to this template-repo! `:)`
Goal of this repo is to provide a quick setup for rust-projects.

## Reason for `version < 1.0.0`

I don't like all those projects staying with version `< 1.0.0` forever.
This section helps to remind everyone including myself, why I am not on version `1.0.0` yet, or whether I should already be above.


## Setup and usage

### Long story short

Rust has a build-tool called `cargo`, which can be used to run everything except scripts in `scripts/`.

```zsh
cargo run --release --bin my_binary
```


## Credits

The project started in the early 2020.
This page honors the workers and helpers of this project, sorted by their last names.

__[Dominic Parga Cacheiro][github/dominicparga]__  
has written this template.


[crates.io/self]: https://crates.io/crates/template-rust
[crates.io/self/badge]: https://img.shields.io/crates/v/template-rust?style=for-the-badge
[docs.rs/self]: https://docs.rs/template-rust/
[docs.rs/self/badge]: https://img.shields.io/crates/v/template-rust?color=informational&label=docs&style=for-the-badge
[github/dominicparga]: https://github.com/dominicparga
[github/lesstat/cyclops/blob/README]: https://github.com/Lesstat/cyclops/blob/master/README.md#graph-data
[github/lesstat/multi-ch-constructor]: https://github.com/Lesstat/multi-ch-constructor
[github/lesstat/multi-ch-constructor/change-dim]: https://github.com/Lesstat/multi-ch-constructor/blob/bec548c1a1ebeae7ac19d3250d5473199336d6fe/src/multi_lib/graph.hpp#L49
[github/self/actions]: https://github.com/dominicparga/template-rust/actions
[github/self/actions/badge]: https://img.shields.io/github/workflow/status/dominicparga/template-rust/Rust?label=nightly-build&style=for-the-badge
[github/self/blob/changelog]: https://github.com/dominicparga/template-rust/blob/nightly/CHANGELOG.md
[github/self/blob/changelog/badge]: https://img.shields.io/badge/CHANGELOG-nightly-blueviolet?style=for-the-badge
[github/self/last-commit]: https://github.com/dominicparga/template-rust/commits
[github/self/last-commit/badge]: https://img.shields.io/github/last-commit/dominicparga/template-rust?style=for-the-badge
[github/self/license]: https://github.com/dominicparga/template-rust/blob/nightly/LICENSE
[github/self/license/badge]: https://img.shields.io/badge/license-Apache--2.0-green?style=for-the-badge
[github/self/tags]: https://github.com/dominicparga/template-rust/tags
[github/self/tags/badge]: https://img.shields.io/github/v/tag/dominicparga/template-rust?sort=semver&style=for-the-badge
[github/self/tree/examples]: https://github.com/dominicparga/template-rust/tree/nightly/examples
[github/self/wiki/usage]: https://github.com/dominicparga/template-rust/wiki/Usage
[github/servo/rust-smallvec]: https://github.com/servo/rust-smallvec
