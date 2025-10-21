pub mod colors;
pub mod http;

use colored::Colorize;

/// Print informational message
#[allow(dead_code)] // Will be used in multiple phases
pub fn print_info(message: &str) {
    println!("{}", message);
}

/// Print success message in green
pub fn print_success(message: &str) {
    println!("{}", message.green());
}

/// Print warning message in yellow
pub fn print_warning(message: &str) {
    println!("{}", message.yellow());
}

/// Print error message in red
#[allow(dead_code)] // Will be used in error handling
pub fn print_error(message: &str) {
    eprintln!("{}", message.red());
}

/// Print a checkmark (✓) in green
pub fn print_check() {
    print!("{} ", "✓".green());
}

/// Print an X mark (✗) in red
pub fn print_x() {
    print!("{} ", "✗".red());
}

/// Print arrow (→) in cyan
#[allow(dead_code)] // Will be used in Phase 5 (list)
pub fn print_arrow() {
    print!("{} ", "→".cyan());
}

/// Check if terminal supports colors
pub fn supports_color() -> bool {
    // Check NO_COLOR environment variable (standard)
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Check if running in a TTY
    atty::is(atty::Stream::Stdout)
}

/// Disable colors if not supported
pub fn init_colors() {
    if !supports_color() {
        colored::control::set_override(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_support() {
        // This test just ensures the function doesn't panic
        let _ = supports_color();
    }
}
