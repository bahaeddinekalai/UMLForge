#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

//! Cross-platform Dioxus interface for UMLForge.

pub mod strings;
pub mod theme;

use dioxus::prelude::*;

/// Renders the initial UMLForge application screen.
#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        main {
            h1 { "{strings::EN.app_name}" }
            p { "{strings::EN.welcome}" }
        }
    }
}
