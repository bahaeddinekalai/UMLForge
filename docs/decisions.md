# Project decisions

- **Project name:** UMLForge is the current working product name. Trademark clearance has not yet been verified; complete that review before the first public release.
- **Implementation:** Rust workspace; Dioxus 0.7.1 UI for Windows desktop and WebAssembly web.
- **Rust:** stable toolchain, edition 2024, minimum Rust 1.85.
- **Project licence:** dual-licensed under MIT OR Apache-2.0. See `LICENSE`, `LICENSE-MIT`, and `LICENSE-APACHE-2.0`.
- **Unsafe code:** forbidden workspace-wide. Clippy denies panic-prone operations.
