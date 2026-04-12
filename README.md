[![](https://github.com/RosieTheGhostie/maybe-fatal/actions/workflows/tests.yml/badge.svg)][GitHub Actions]
[![](https://docs.rs/maybe-fatal/badge.svg)][docs.rs]
[![](https://img.shields.io/crates/v/maybe-fatal.svg)][crates.io]
[![](https://img.shields.io/crates/d/maybe-fatal.svg)][crates.io]
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://gnu.org/licenses/lgpl-3.0)

Potentially fatal diagnostics and diagnostic handling for compilers.

# Usage

```bash
cargo add maybe-fatal@0.1.0-beta.4
```

## Examples

See the [examples](./examples) directory for some examples of how to use this crate.

# Relationship to Other Crates

This crate can be thought of as a high-level wrapper around the [ariadne] crate, as its core
functionality is driven by it. It is also kind of a spiritual successor to the [rich-err] crate I
wrote, but to be honest, I totally forgot that existed until I went to publish this one.

There is also the [ariadnenum] crate, which fulfills a similar purpose to the [maybe-fatal-derive]
crate in this repository. The primary difference is that [maybe-fatal-derive] was designed
specifically for use in the context of this crate, whereas [ariadnenum] is meant for a workflow that
is already using [ariadne] directly.

One other crate worth mentioning is [wurm], which was similarly made for non-fatal error handling. I
considered using that crate here, but it made more sense with how I had designed the `Diagnostic`
type to just make my own stuff. Functionality relating to that can be found in the `sink` module.

# Prerelease Status

This is currently marked as being in beta, as I am still working on making sure the API is as
ergonomic and flexible as possible. I would also like to add some more detailed documentation, unit
tests, integration tests, and examples.

[ariadne]: https://docs.rs/ariadne/latest/ariadne/
[ariadnenum]: https://docs.rs/ariadnenum/latest/ariadnenum/
[crates.io]: https://crates.io/crates/maybe-fatal
[docs.rs]: https://docs.rs/maybe-fatal/latest/maybe_fatal/
[GitHub Actions]: https://github.com/RosieTheGhostie/maybe-fatal/actions
[maybe-fatal-derive]: https://docs.rs/maybe-fatal-derive/latest/maybe_fatal_derive/
[rich-err]: https://docs.rs/rich-err/latest/rich_err/
[wurm]: https://docs.rs/wurm/latest/wurm/
