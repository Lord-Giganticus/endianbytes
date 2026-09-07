# endianbytes
An implementation of upcoming rust feature read_le_be, and a write variant too.

![Crates.io Version](https://img.shields.io/crates/v/endianbytes)

Heavily inspired by nightly feature [read_le_be](https://github.com/rust-lang/rust/issues/156984) but adds a few more things (f16 and f128 support) and a
write_le_be variant too as I felt that was missing from the original rust PR.

It is recommended you import everything via glob import `use endianbytes::*;` to get the full set of features.

## NOTE
This crate follows an ***unconditional*** anti-LLM and anti-gen-AI policy for all contributions. Vibe coding sucks for everyone, don't do it.
