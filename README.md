[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](MIT)
[![License: APACHE2.0](https://img.shields.io/badge/License-APACHE2.0-blue?style=flat-square)](APACHE2.0)

## N64 Project Template for Rust (with Libdragon!)
This repo is provided as a proof-of-concept build linking Libdragon into a Rust program.

This project does not use nust64 or n64-pac like the original source code.

See the original documentation at https://github.com/rust-n64/n64-project-template

## Building
0. Install and build my fork of Libdragon: https://github.com/sarchar/libdragon
    0a. You need to build the toolchain yourself. The precompiled version *will not work*.
1. Install Rust: https://www.rust-lang.org/tools/install
2. Get the source: (e.g. using git, or downloading an archive manually)
```
git clone https://github.com/sarchar/n64-project-template
cd n64-project-template
```
3. Install just: `cargo install just`
4. Run `just build` to compile and build a ROM.

## License
The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can chose either the MIT licence or the
Apache-2.0 licence when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific licence.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.
