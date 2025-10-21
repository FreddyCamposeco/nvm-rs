use colored::{Color, Colorize};

/// Color scheme for NVM output (compatible with PowerShell version)
#[allow(dead_code)] // Will be used in Phase 5 (list) and Phase 7 (set-colors)
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub current: Color,       // Current version marker
    pub installed: Color,     // Installed checkmark
    pub not_installed: Color, // Not installed X mark
    pub system: Color,        // System version
    pub lts_label: Color,     // LTS labels
    pub latest: Color,        // Latest label
    pub global: Color,        // Global label
    pub nvmrc: Color,         // .nvmrc label
    pub gray: Color,          // Secondary text
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            current: Color::Green,
            installed: Color::Green,
            not_installed: Color::Red,
            system: Color::Yellow,
            lts_label: Color::Yellow,
            latest: Color::Cyan,
            global: Color::Cyan,
            nvmrc: Color::Magenta,
            gray: Color::BrightBlack,
        }
    }
}

impl ColorScheme {
    /// Parse color scheme from string (PowerShell compatibility)
    /// Format: 5 characters representing colors in order:
    /// current, installed, not_installed, system, lts_label
    ///
    /// Color codes:
    /// r/R = red, g/G = green, b/B = blue, y/Y = yellow
    /// c/C = cyan, m/M = magenta, k/K = black, w/W = white, e/E = gray
    #[allow(dead_code)] // Will be used in Phase 7 (set-colors command)
    pub fn from_string(scheme: &str) -> Option<Self> {
        if scheme.len() < 5 {
            return None;
        }

        let chars: Vec<char> = scheme.chars().collect();

        Some(ColorScheme {
            current: Self::parse_color_code(chars[0])?,
            installed: Self::parse_color_code(chars[1])?,
            not_installed: Self::parse_color_code(chars[2])?,
            system: Self::parse_color_code(chars[3])?,
            lts_label: Self::parse_color_code(chars[4])?,
            ..Default::default()
        })
    }

    fn parse_color_code(code: char) -> Option<Color> {
        match code {
            'r' => Some(Color::Red),
            'R' => Some(Color::BrightRed),
            'g' => Some(Color::Green),
            'G' => Some(Color::BrightGreen),
            'b' => Some(Color::Blue),
            'B' => Some(Color::BrightBlue),
            'y' => Some(Color::Yellow),
            'Y' => Some(Color::BrightYellow),
            'c' => Some(Color::Cyan),
            'C' => Some(Color::BrightCyan),
            'm' => Some(Color::Magenta),
            'M' => Some(Color::BrightMagenta),
            'k' => Some(Color::Black),
            'K' => Some(Color::BrightBlack),
            'w' => Some(Color::White),
            'W' => Some(Color::BrightWhite),
            'e' | 'E' => Some(Color::BrightBlack),
            _ => None,
        }
    }
}

/// Format a version line with colors
#[allow(dead_code)] // Will be used in Phase 5 (list command)
pub fn format_version_line(
    label: &str,
    version: &str,
    is_installed: bool,
    is_current: bool,
    scheme: &ColorScheme,
) -> String {
    let mut output = String::new();

    // Add checkmark/X
    if is_installed {
        output.push_str(&format!("{} ", "✓".color(scheme.installed)));
    } else {
        output.push_str(&format!("{} ", "✗".color(scheme.not_installed)));
    }

    // Add arrow if current
    if is_current {
        output.push_str(&format!("{} ", "→".color(scheme.current)));
    } else {
        output.push_str("  ");
    }

    // Add label if present
    if !label.is_empty() {
        let colored_label = match label {
            l if l.starts_with("lts/") => l.color(scheme.lts_label),
            "latest:" => label.color(scheme.latest),
            "system:" => label.color(scheme.system),
            ".nvmrc:" => label.color(scheme.nvmrc),
            _ => label.color(scheme.gray),
        };
        output.push_str(&format!("{:<15}", colored_label));
    }

    // Add version
    let version_colored = if is_current {
        version.color(scheme.current).bold()
    } else {
        version.normal()
    };
    output.push_str(&version_colored.to_string());

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_parsing() {
        let scheme = ColorScheme::from_string("ggrye");
        assert!(scheme.is_some());

        let invalid = ColorScheme::from_string("xyz");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_default_scheme() {
        let scheme = ColorScheme::default();
        assert_eq!(scheme.current, Color::Green);
    }
}
