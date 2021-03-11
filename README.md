# twasm-std

Tetsy WASM contracts standard library for Rust

[![Build Status](https://travis-ci.org/paritytech/pwasm-std.svg?branch=master)](https://travis-ci.org/paritytech/pwasm-std)

[Documentation](https://paritytech.github.io/pwasm-std/pwasm_std/)

`pwasm-std` is a limited subset of the Rust standard library, along with a custom allocator which delegates the allocation to the runtime-defined externs.

## Use

Just add a dependency
```toml
[dependencies]
pwasm-std = "0.13"
```

Test `pwasm-std` with

```
cargo test --features=test
```

The crate is supposed to be used on nightly Rust only, until the custom allocator api stablizes in Rust.

## no_std

`pwasm-std` is itself compiled with no_std and expected to be used within no_std-crates/binaries, since it defines `lang_item`-s on it's own, which will conflict with standard library.

But for test scenarios it can be compiled with feature "std" and auxiliary crate "pwasm-test" to support testing of contracts' internal logic.

# License

`pwasm-std` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in pwasm-std by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
