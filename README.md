# RISC0 provable reth-primitives


## Overview

This repo is based on RISC0 template with added [paradigm/reth-primitives](https://github.com/paradigmxyz/reth/tree/main/crates/primitives) as a dependency, 
to show case that it can operate in risc0 ZK VM.

in [`host/Cargo.toml`](./host/Cargo.toml) and [`methods/guest/Cargo.toml`](methods/guest/Cargo.toml) defined several versions of reth-primitives:

 

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