//! English-first UI strings. Feature components should use these table entries, not literals.

/// English labels used by the initial application shell.
pub const EN: Strings = Strings {
    app_name: "UMLForge",
    welcome: "Your modelling workspace is ready.",
};

/// The initial application string table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Strings {
    /// Product name shown in the app.
    pub app_name: &'static str,
    /// Introductory message on the empty application screen.
    pub welcome: &'static str,
}
