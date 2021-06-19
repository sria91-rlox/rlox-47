use std::io::Read;

use crate::errors::LoxError;

pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // Single or double character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new<T>(token_type: TokenType, lexeme: T, line: usize) -> Self
    where
        T: Into<String>,
    {
        Token {
            token_type,
            lexeme: lexeme.into(),
            line,
        }
    }

    pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<LoxError>> {
        let mut errors = Vec::new();

        let mut tokens = Vec::new();
        let mut start_index = 0;
        let mut current_line = 0;
        let mut chars = source.chars().enumerate().peekable();

        fn add_token(
            tokens: &mut Vec<Token>,
            source: &str,
            token_type: TokenType,
            start_index: &mut usize,
            index: usize,
            line: usize,
        ) {
            tokens.push(Token::new(
                token_type,
                source.get(*start_index..index).unwrap(),
                line,
            ));
        }

        while let Some((index, c)) = chars.next() {
            let start_index = index;

            match c {
                '(' => add_token(
                    &mut tokens,
                    source,
                    TokenType::LeftParen,
                    &mut start_index,
                    index,
                    current_line,
                ),
                ')' => add_token(
                    &mut tokens,
                    source,
                    TokenType::RightParen,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '{' => add_token(
                    &mut tokens,
                    source,
                    TokenType::LeftBrace,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '}' => add_token(
                    &mut tokens,
                    source,
                    TokenType::RightBrace,
                    &mut start_index,
                    index,
                    current_line,
                ),
                ',' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Comma,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '.' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Dot,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '-' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Minus,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '+' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Plus,
                    &mut start_index,
                    index,
                    current_line,
                ),
                ';' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Semicolon,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '*' => add_token(
                    &mut tokens,
                    source,
                    TokenType::Star,
                    &mut start_index,
                    index,
                    current_line,
                ),
                '!' => add_token(
                    &mut tokens,
                    source,
                    match chars.peek() {
                        Some((_, '=')) => TokenType::GreaterEqual,
                        _ => TokenType::Greater,
                    },
                    &mut start_index,
                    index,
                    current_line,
                ),
                '=' => add_token(
                    &mut tokens,
                    source,
                    match chars.peek() {
                        Some((_, '=')) => TokenType::GreaterEqual,
                        _ => TokenType::Greater,
                    },
                    &mut start_index,
                    index,
                    current_line,
                ),
                '<' => add_token(
                    &mut tokens,
                    source,
                    match chars.peek() {
                        Some((_, '=')) => TokenType::GreaterEqual,
                        _ => TokenType::Greater,
                    },
                    &mut start_index,
                    index,
                    current_line,
                ),
                '>' => add_token(
                    &mut tokens,
                    source,
                    match chars.peek() {
                        Some((_, '=')) => TokenType::GreaterEqual,
                        _ => TokenType::Greater,
                    },
                    &mut start_index,
                    index,
                    current_line,
                ),
                '/' => {
                    if let Some((_, '/')) = chars.peek() {
                        while let Some((_, '\n')) = chars.next() {}
                    } else {
                        add_token(
                            &mut tokens,
                            source,
                            TokenType::Slash,
                            &mut start_index,
                            index,
                            current_line,
                        );
                    }
                }
                '"' => {
                    let mut index = 0;
                    while match chars.peek(){
                        Some((i, cc)) => {
                            index = *i;
                            *cc != '"'
                        },
                        None => {
                            errors.push(LoxError::UnterminatedString);
                            tokens.push(Token::new(TokenType::EOF, "", tokens.len()));
                            return Err(errors);
                        }
                    }{
                        let c = chars.next();
                        if c.unwrap() == (_, '\n'){
                            current_line += 1;
                        }
                    }

                    chars.next();

                    tokens.push(Token::new(TokenType::String, source.get(start_index + 1..index + 1).unwrap(), current_line));
                }
                ' ' | '\r' | '\t' => break,
                '\n' => current_line += 1,

                _ => errors.push(LoxError::UnexpectedCharacter(current_line, c)),
            }
        }

        tokens.push(Token::new(TokenType::EOF, "", tokens.len()));

        Ok(tokens)
    }
}
