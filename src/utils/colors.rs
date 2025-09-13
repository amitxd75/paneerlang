use colored::*;

// Color scheme for PaneerLang
pub struct PaneerColors;

impl PaneerColors {
    // Keywords
    pub fn keyword(text: &str) -> ColoredString {
        text.blue().bold()
    }

    pub fn special_keyword(text: &str) -> ColoredString {
        text.magenta().bold()
    }

    pub fn boolean(text: &str) -> ColoredString {
        text.yellow().bold()
    }

    // Types
    pub fn type_name(text: &str) -> ColoredString {
        text.red().bold()
    }

    // Literals
    pub fn string_literal(text: &str) -> ColoredString {
        text.green()
    }

    pub fn number_literal(text: &str) -> ColoredString {
        text.cyan()
    }

    // Operators and punctuation
    pub fn operator(text: &str) -> ColoredString {
        text.yellow()
    }

    pub fn punctuation(text: &str) -> ColoredString {
        text.white().bold()
    }

    // Comments
    pub fn comment(text: &str) -> ColoredString {
        text.bright_black().italic()
    }

    // UI Elements

    pub fn error(text: &str) -> ColoredString {
        text.red()
    }

    pub fn warning(text: &str) -> ColoredString {
        text.yellow()
    }

    pub fn info(text: &str) -> ColoredString {
        text.blue()
    }

    pub fn debug(text: &str) -> ColoredString {
        text.magenta()
    }

    pub fn highlight(text: &str) -> ColoredString {
        text.bright_green()
    }

    // Debug phases
    pub fn debug_phase(text: &str) -> ColoredString {
        text.cyan().bold()
    }

    pub fn debug_success(text: &str) -> ColoredString {
        text.green()
    }

    pub fn debug_info(text: &str) -> ColoredString {
        text.bright_blue()
    }

    // Borders and separators
    pub fn border(text: &str) -> ColoredString {
        text.bright_black()
    }

    pub fn separator(text: &str) -> ColoredString {
        text.bright_black()
    }
}
