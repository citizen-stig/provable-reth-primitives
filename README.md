# RISC0 provable reth-primitives


## Overview

This repo is based on RISC0 template with added [paradigm/reth-primitives](https://github.com/paradigmxyz/reth/tree/main/crates/primitives) as a dependency, 
to show case that it can operate in risc0 ZK VM.

Files [`host/Cargo.toml`](./host/Cargo.toml) and [`methods/guest/Cargo.toml`](methods/guest/Cargo.toml) defined several versions of reth-primitives:

Main 2 are:

* Active: [`reth rev 43c72b0`](https://github.com/paradigmxyz/reth/commit/43c72b022cea3068edbce794fa94adefb029fd06), which is provable. Steps from [Quick start](#quick-start) should just work. Last line of the output should be:
```
2024-06-14T09:13:22.909775Z  INFO host: Done. output=10015
```
* One of the latest [`reth v0.2.0-beta.9`](https://github.com/paradigmxyz/reth/tree/v0.2.0-beta.9). To activate it, do the following steps:
   1. Comment whole section related `reth-primitives` in [`host/Cargo.toml`](./host/Cargo.toml) and [`methods/guest/Cargo.toml`](methods/guest/Cargo.toml)
   2. Uncomment `reth-primitives` with tag `reth v0.2.0-beta.9` in [`host/Cargo.toml`](./host/Cargo.toml) and [`methods/guest/Cargo.toml`](methods/guest/Cargo.toml)
   3. Uncomment `patch.crates-io` section **in root [`Cargo.toml`](./Cargo.toml)** and [`methods/guest/Cargo.toml`](methods/guest/Cargo.toml)
   4. Just to be sure run `cargo clean`
   5. Execute steps from [Quick start](#quick-start). Or just `RUST_LOG="info" RISC0_DEV_MODE=1 cargo run` if quick start has been done before.
 

`0.2.0-beta9` will probably fail, with something this:

```
reth_primitives_guest:    Compiling wait-timeout v0.2.0
reth_primitives_guest: error[E0433]: failed to resolve: use of undeclared crate or module `imp`
reth_primitives_guest:   --> /Users/nikolai/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wait-timeout-0.2.0/src/lib.rs:66:9
reth_primitives_guest:    |
reth_primitives_guest: 66 |         imp::wait_timeout(self, dur)
reth_primitives_guest:    |         ^^^ use of undeclared crate or module `imp`
reth_primitives_guest:
reth_primitives_guest: For more information about this error, try `rustc --explain E0433`.
```

Which is being addressed by https://github.com/paradigmxyz/reth/pull/8821 , but there might be other issues. 

## Quick Start


First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

```bash
make install-risc0-toolchain
```

To build all methods and execute the method within the zkVM, run the following
command:


```bash
RUST_LOG="info" RISC0_DEV_MODE=1 cargo run
```




# Other

## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```