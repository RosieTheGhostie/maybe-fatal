This directory contains a handful of example programs that use the [maybe-fatal] crate. If you are
new to this crate, consider starting with [minimal-tao].

## Running an Example

From the root of the repository, run the following command (replacing `example-name` with the name
of the example):

```bash
cargo run --example example-name
# If the example needs additional features, add them like this:
# --feature feature0 --feature feature1 ...
```

## Available Examples

- **[minimal-tao]**[^1]

  The smallest possible recreation of [ariadne]'s main example (within reason).

- **[tao]**[^1]

  A more realistic variant of [minimal-tao] that demonstrates the general workflow this crate
  provides.

[ariadne]: https://docs.rs/ariadne/latest/ariadne/
[maybe-fatal]: https://docs.rs/maybe-fatal/latest/maybe_fatal/
[minimal-tao]: minimal-tao/README.md
[tao]: tao/README.md

[^1]: Requires the "derive" feature to be enabled.
