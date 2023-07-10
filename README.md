# rebound-sys

This is a Rust crate which provides bindings for
[REBOUND](https://github.com/hannorein/rebound), an N-body simulator.

The generated bindings are currently only for a statically linked
version of librebound. This isn't for a deep reason; it's just that I
expect that dynamically linking to rebound is pretty unusual, and it's
easier to provide a statically linked sys crate.
