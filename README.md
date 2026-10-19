# UMLForge

UMLForge is an open-source modelling tool built in Rust with Dioxus for Windows desktop and the web. The workspace is at its foundation stage; most diagram and modelling features are not implemented yet.

## Prerequisites

- Rust stable via [rustup](https://rustup.rs/). The repository toolchain configuration installs `rustfmt`, Clippy, LLVM coverage tools, and the `wasm32-unknown-unknown` target.
- Git.
- The Dioxus CLI **0.7.1**, matching the `dioxus` crate. Install with `cargo binstall dioxus-cli@0.7.1` (or `cargo install dioxus-cli --version 0.7.1 --locked`).
- For native Windows app builds: Visual Studio C++ Build Tools, Windows SDK, WebView2, and the NASM/CMake dependencies required by native crates.

## Build and test

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --exclude umlforge-desktop --all-targets -- -D warnings
cargo test --workspace --exclude umlforge-desktop
cargo xtask layers
cargo xtask parity
cargo xtask wasm
```

`cargo xtask wasm` checks the WebAssembly-compatible workspace members. The Windows-only desktop application and platform-neutral UI root are excluded; the web application checks the UI through its web-specific Dioxus feature.

## Run

From the repository root:

```powershell
dx serve --platform web --package umlforge-web
dx serve --platform desktop --package umlforge-desktop
cargo run --package umlforge-cli
```

## Licence

UMLForge is available under your choice of the MIT License or Apache License, Version 2.0. See [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT), and [LICENSE-APACHE-2.0](LICENSE-APACHE-2.0).
