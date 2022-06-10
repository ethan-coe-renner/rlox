use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl TokenType {
    pub fn string_to_keyword_or_id(input: &str) -> Self {
	match input {
	    "and" => Self::AND,
	    "class" => Self::CLASS,
	    "else"=> Self::ELSE,
	    "false" => Self::FALSE,
	    "fun" => Self::FUN,
	    "for" => Self::FOR,
	    "if" => Self::IF,
	    "nil" => Self::NIL,
	    "or" => Self::OR,
	    "print" => Self::PRINT,
	    "return" => Self::RETURN,
	    "super" => Self::SUPER,
	    "this" => Self::THIS,
	    "true" => Self::TRUE,
	    "var" => Self::VAR,
	    "while" => Self::WHILE,
	    identifier => Self::IDENTIFIER(identifier.to_string()),
	}
	}
}

pub struct Token {
    token_type: TokenType,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::IDENTIFIER(id) => write!(f, "{:?} {id}", self.token_type),
            TokenType::NUMBER(num) => write!(f, "{:?} {num}", self.token_type),
            TokenType::STRING(string) => write!(f, "{:?} {string}", self.token_type),
            other_type => write!(f, "{:?}", other_type),
        }
    }
}
