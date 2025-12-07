pub fn print_success(message: &str) {
    println!("✓ {}", message);
}

pub fn print_info(message: &str) {
    println!("{}", message);
}

pub fn print_warning(message: &str) {
    println!("Warning: {}", message);
}

// ANSI color codes
pub const COLOR_RESET: &str = "\x1b[0m";
pub const COLOR_RED: &str = "\x1b[31m";
pub const COLOR_GREEN: &str = "\x1b[32m";
pub const COLOR_YELLOW: &str = "\x1b[33m";
pub const COLOR_BLUE: &str = "\x1b[34m";
pub const COLOR_MAGENTA: &str = "\x1b[35m";
pub const COLOR_CYAN: &str = "\x1b[36m";
pub const COLOR_GRAY: &str = "\x1b[37m";
pub const COLOR_DARK_GRAY: &str = "\x1b[90m";

pub fn colorize(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, COLOR_RESET)
}

pub fn print_colored(text: &str, color: &str) {
    print!("{}", colorize(text, color));
}

pub fn print_colored_no_newline(text: &str, color: &str) {
    print!("{}", colorize(text, color));
}
