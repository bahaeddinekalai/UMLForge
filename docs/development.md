# Development setup

## Prerequisites

- Install stable Rust via rustup. `rust-toolchain.toml` selects Rust and requests the `rustfmt`, `clippy`, and `llvm-tools-preview` components plus `wasm32-unknown-unknown`.
- Install the Dioxus CLI matching the exact dependency version, currently `0.7.1`: `cargo binstall dioxus-cli --version 0.7.1 --no-confirm`.
- Windows desktop builds require a supported Visual Studio C++ toolchain, a Windows SDK, and WebView2.

## Build, test, lint

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --exclude umlforge-desktop --all-targets -- -D warnings
cargo test --workspace --exclude umlforge-desktop
cargo xtask layers
cargo xtask parity
cargo xtask wasm
```

The desktop app is Windows-only. `cargo xtask wasm` excludes the native desktop app and platform-neutral UI root, then compiles the web app with the shared UI's web-specific Dioxus backend.

## Run the apps

```powershell
dx serve --platform web --package umlforge-web
dx serve --platform desktop --package umlforge-desktop
cargo run --package umlforge-cli
```

This workspace is an initial scaffold; feature implementation proceeds according to `Build-plan.MD`.
