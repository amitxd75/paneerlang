//! Lexical analysis for PaneerLang
//!
//! This module handles tokenization of PaneerLang source code using the logos crate.
//! It converts raw text into a stream of tokens that can be consumed by the parser.

use logos::Logos;

/// Token types for PaneerLang lexical analysis
///
/// Uses the logos crate for efficient tokenization with regex patterns.
/// Supports Hindi keywords, operators, literals, and identifiers.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Keywords - Hindi-inspired language constructs
    /// Built-in object for core functions
    #[token("paneer")]
    Paneer,

    #[token("bol")]
    Bol,

    #[token("ye")]
    Ye,

    #[token("agar")]
    Agar,

    #[token("toh")]
    Toh,

    #[token("varna")]
    Varna,

    #[token("func")]
    Func,

    #[token("return")]
    Return,

    #[token("wapas")]
    Wapas,

    #[token("kar")]
    Kar,

    #[token("jabtak")]
    Jabtak, // while

    #[token("har")]
    Har, // for

    #[token("mein")]
    Mein, // in

    #[token("se")]
    Se, // from

    #[token("tak")]
    Tak, // to

    // Types
    #[token("int")]
    IntType,

    #[token("float")]
    FloatType,

    #[token("string")]
    StringType,

    #[token("bool")]
    BoolType,

    #[token("array")]
    ArrayType,

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLiteral(i64),

    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(f64),

    #[regex(r#""([^"\\]|\\["\\nt])*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteral(String),

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("!")]
    Bang,

    #[token(">")]
    Greater,

    #[token("<")]
    Less,

    #[token(">=")]
    GreaterEqual,

    #[token("<=")]
    LessEqual,

    // Delimiters
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token("=")]
    Assign,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Comments
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,
}

/// Lexer state for tokenizing PaneerLang source code
///
/// Maintains a vector of tokens with their source positions and tracks
/// the current position during parsing.
pub struct Lexer {
    /// All tokens extracted from the source code with their positions
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    /// Current position in the token stream
    current: usize,
}

impl Lexer {
    /// Creates a new lexer and tokenizes the input string
    ///
    /// # Arguments
    /// * `input` - The PaneerLang source code to tokenize
    ///
    /// # Returns
    /// * `Ok(Lexer)` - Successfully tokenized lexer
    /// * `Err(String)` - Error message if tokenization fails
    pub fn new(input: &str) -> Result<Self, String> {
        let mut tokens = Vec::new();
        let mut lex = Token::lexer(input);

        while let Some(token) = lex.next() {
            match token {
                Ok(token) => tokens.push((token, lex.span())),
                Err(_) => {
                    return Err(format!(
                        "Unexpected character at position {}: '{}'",
                        lex.span().start,
                        &input[lex.span()]
                    ));
                }
            }
        }

        Ok(Lexer { tokens, current: 0 })
    }

    /// Peeks at the current token without consuming it
    ///
    /// # Returns
    /// * `Some(&Token)` - The current token if available
    /// * `None` - If at end of token stream
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current).map(|(token, _)| token)
    }

    /// Consumes and returns the current token, advancing to the next
    ///
    /// # Returns
    /// * `Some(&Token)` - The consumed token if available
    /// * `None` - If at end of token stream
    pub fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current].0;
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Checks if the lexer has reached the end of the token stream
    ///
    /// # Returns
    /// * `true` - If all tokens have been consumed
    /// * `false` - If more tokens are available
    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
