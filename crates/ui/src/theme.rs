//! Shared foundations for application themes. Values follow Design.md's Paper & Ink tokens.

/// A semantic colour token shared by app chrome and diagram surfaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorTokens {
    /// App and diagram background.
    pub canvas: &'static str,
    /// Panels, dialogs, and cards.
    pub surface: &'static str,
    /// Primary text.
    pub ink: &'static str,
    /// Secondary text and hints.
    pub ink_muted: &'static str,
    /// Panel dividers.
    pub line: &'static str,
    /// Control and icon outlines.
    pub line_strong: &'static str,
    /// Primary actions, selection, links, and focus.
    pub accent: &'static str,
    /// Text drawn on the accent colour.
    pub on_accent: &'static str,
    /// Errors and destructive actions.
    pub danger: &'static str,
    /// Successful and valid states.
    pub success: &'static str,
    /// Attention and warning states.
    pub warning: &'static str,
}

/// Warm, paper-like default tokens.
pub const LIGHT: ColorTokens = ColorTokens {
    canvas: "#F7F5F1",
    surface: "#FFFFFF",
    ink: "#1B1F23",
    ink_muted: "#58616B",
    line: "#E3DFD7",
    line_strong: "#8A9199",
    accent: "#0E6B62",
    on_accent: "#FFFFFF",
    danger: "#B42318",
    success: "#177245",
    warning: "#8F5200",
};

/// Elevated-surface dark theme tokens.
pub const DARK: ColorTokens = ColorTokens {
    canvas: "#121518",
    surface: "#1A1E22",
    ink: "#ECEFF2",
    ink_muted: "#A5AEB8",
    line: "#2A3036",
    line_strong: "#7C858F",
    accent: "#5FCDBF",
    on_accent: "#0B1412",
    danger: "#FF8A7A",
    success: "#5FD39A",
    warning: "#F0B35A",
};

#[cfg(test)]
mod tests {
    use super::{DARK, LIGHT};

    #[test]
    fn light_theme_provides_every_semantic_token() {
        assert_eq!(LIGHT.canvas, "#F7F5F1");
        assert_eq!(LIGHT.surface, "#FFFFFF");
        assert_eq!(LIGHT.accent, "#0E6B62");
    }

    #[test]
    fn dark_theme_uses_dark_surfaces_and_light_ink() {
        assert_eq!(DARK.canvas, "#121518");
        assert_eq!(DARK.surface, "#1A1E22");
        assert_eq!(DARK.ink, "#ECEFF2");
    }
}
