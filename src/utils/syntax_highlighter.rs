use crate::utils::colors::PaneerColors;

pub struct SyntaxHighlighter {
    keywords: Vec<&'static str>,
    types: Vec<&'static str>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        SyntaxHighlighter {
            keywords: vec![
                "ye", "agar", "varna", "func", "return", "wapas", "kar", "jabtak", "har", "mein",
                "se", "tak", "paneer", "bol", "true", "false",
            ],
            types: vec!["int", "float", "string", "bool", "array"],
        }
    }

    pub fn highlight(&self, code: &str) -> String {
        // Use a more sophisticated approach to avoid conflicts
        let mut result = String::new();
        let mut chars = code.chars().peekable();
        let mut current_token = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                '/' if chars.peek() == Some(&'/') => {
                    // Handle comments - consume rest of line
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                    }
                    let mut comment = String::from("//");
                    chars.next(); // consume second '/'
                    for ch in chars.by_ref() {
                        if ch == '\n' {
                            break;
                        }
                        comment.push(ch);
                    }
                    result.push_str(&format!("{}", PaneerColors::comment(&comment)));
                }
                '"' => {
                    // Handle strings
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                    }
                    let mut string_literal = String::from("\"");
                    while let Some(ch) = chars.next() {
                        string_literal.push(ch);
                        if ch == '"' {
                            break;
                        }
                        if ch == '\\'
                            && let Some(escaped) = chars.next()
                        {
                            string_literal.push(escaped);
                        }
                    }
                    result.push_str(&format!(
                        "{}",
                        PaneerColors::string_literal(&string_literal)
                    ));
                }
                ' ' | '\t' | '\n' | '\r' => {
                    // Whitespace - process current token and add whitespace
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                    }
                    result.push(ch);
                }
                '(' | ')' | '{' | '}' | '[' | ']' | ';' | ':' | ',' | '.' => {
                    // Punctuation
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                    }
                    result.push_str(&format!("{}", PaneerColors::punctuation(&ch.to_string())));
                }
                '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' => {
                    // Operators - check for multi-character operators
                    if !current_token.is_empty() {
                        result.push_str(&self.highlight_token(&current_token));
                        current_token.clear();
                    }
                    let mut operator = ch.to_string();
                    if let Some(&next_ch) = chars.peek()
                        && (ch == '>' || ch == '<' || ch == '!' || ch == '=')
                        && next_ch == '='
                    {
                        operator.push(chars.next().unwrap());
                    }
                    result.push_str(&format!("{}", PaneerColors::operator(&operator)));
                }
                _ => {
                    // Regular character - add to current token
                    current_token.push(ch);
                }
            }
        }

        // Process any remaining token
        if !current_token.is_empty() {
            result.push_str(&self.highlight_token(&current_token));
        }

        result
    }

    fn highlight_token(&self, token: &str) -> String {
        // Check if it's a number
        if token.chars().all(|c| c.is_ascii_digit() || c == '.')
            && token.chars().any(|c| c.is_ascii_digit())
        {
            return format!("{}", PaneerColors::number_literal(token));
        }

        // Check if it's a keyword
        if self.keywords.contains(&token) {
            if token == "paneer" || token == "bol" {
                return format!("{}", PaneerColors::special_keyword(token));
            } else if token == "true" || token == "false" {
                return format!("{}", PaneerColors::boolean(token));
            } else {
                return format!("{}", PaneerColors::keyword(token));
            }
        }

        // Check if it's a type
        if self.types.contains(&token) {
            return format!("{}", PaneerColors::type_name(token));
        }

        // Default - return as is
        token.to_string()
    }
}

pub fn print_code_block(title: &str, code: &str) {
    let highlighter = SyntaxHighlighter::new();
    println!("{} {}", PaneerColors::info("📝"), PaneerColors::info(title));
    println!(
        "{}{}{}",
        PaneerColors::border("┌"),
        PaneerColors::border(&"─".repeat(60)),
        PaneerColors::border("┐")
    );

    for line in code.lines() {
        let highlighted = highlighter.highlight(line);
        println!(
            "{} {} {}",
            PaneerColors::border("│"),
            highlighted,
            PaneerColors::border("│")
        );
    }

    println!(
        "{}{}{}",
        PaneerColors::border("└"),
        PaneerColors::border(&"─".repeat(60)),
        PaneerColors::border("┘")
    );
}
