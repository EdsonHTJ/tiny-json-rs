use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const SIMPLE_ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
const DIGITS: &str = "0123456789";
//  Json tokens
pub enum Chars {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Quote,
    NewLine,
    MinusSign,
    Dot,
    Space,
    Char(u8),
}

impl Chars {
    pub fn from(ch: u8) -> Chars {
        match ch {
            b'{' => Chars::LBrace,
            b'}' => Chars::RBrace,
            b'[' => Chars::LBracket,
            b']' => Chars::RBracket,
            b':' => Chars::Colon,
            b',' => Chars::Comma,
            b'"' => Chars::Quote,
            b'\n' => Chars::NewLine,
            b'-' => Chars::MinusSign,
            b'.' => Chars::Dot,
            b' ' => Chars::Space,
            _ => Chars::Char(ch),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Chars::LBrace => "{".to_string(),
            Chars::RBrace => "}".to_string(),
            Chars::LBracket => "[".to_string(),
            Chars::RBracket => "]".to_string(),
            Chars::Colon => ":".to_string(),
            Chars::Comma => ",".to_string(),
            Chars::Quote => "\"".to_string(),
            Chars::NewLine => "\n".to_string(),
            Chars::MinusSign => "-".to_string(),
            Chars::Dot => ".".to_string(),
            Chars::Space => " ".to_string(),
            Chars::Char(ch) => format!("{}", *ch as char),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Chars::LBrace => '{',
            Chars::RBrace => '}',
            Chars::LBracket => '[',
            Chars::RBracket => ']',
            Chars::Colon => ':',
            Chars::Comma => ',',
            Chars::Quote => '"',
            Chars::NewLine => '\n',
            Chars::MinusSign => '-',
            Chars::Dot => '.',
            Chars::Space => ' ',
            Chars::Char(ch) => *ch as char,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringType {
    SimpleString,
    ComplexString,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    None,
    Int,
    String(StringType),
    ReservedString,
    Float,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn default() -> Token {
        return Token {
            token_type: TokenType::None,
            literal: "".to_string(),
        };
    }
}

#[derive(Clone, Debug)]
pub enum LexerError {
    InvalidTokenError(String),
    OutOfRangeError,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub token_list: Vec<Token>,
    pub current_token: Token,
    pub line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            input,
            position: 0,
            token_list: Vec::new(),
            line: 0,
            current_token: Token {
                token_type: TokenType::None,
                literal: "".to_string(),
            },
        };
    }

    fn read_char(&mut self) -> Result<Chars, LexerError> {
        if self.position >= self.input.len() {
            return Err(LexerError::OutOfRangeError);
        }

        let ch = self.input.as_bytes()[self.position];
        self.position += 1;
        return Ok(Chars::from(ch));
    }

    fn process_start_token(&mut self) -> Result<(), LexerError> {
        let ch = self.read_char()?;
        match ch {
            Chars::LBrace => {
                self.current_token.token_type = TokenType::LBrace;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::RBrace => {
                self.current_token.token_type = TokenType::RBrace;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::LBracket => {
                self.current_token.token_type = TokenType::LBracket;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::RBracket => {
                self.current_token.token_type = TokenType::RBracket;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::Colon => {
                self.current_token.token_type = TokenType::Colon;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::Comma => {
                self.current_token.token_type = TokenType::Comma;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::Quote => {
                self.current_token.token_type = TokenType::String(StringType::SimpleString);
                self.current_token.literal = "".to_string();
            }
            Chars::NewLine => {
                self.line += 1;
            }
            Chars::MinusSign => {
                self.current_token.token_type = TokenType::Int;
                self.current_token.literal = ch.to_string();
                self.token_list.push(self.current_token.clone());
            }
            Chars::Space => {}
            Chars::Char(c) => {
                if DIGITS.contains(c as char) {
                    self.current_token.token_type = TokenType::Int;
                    self.current_token.literal = ch.to_string();
                } else if SIMPLE_ALPHA.contains(c as char) {
                    self.current_token.token_type = TokenType::ReservedString;
                    self.current_token.literal = ch.to_string();
                } else {
                    return Err(LexerError::InvalidTokenError(ch.to_string()));
                }
            }
            _ => {
                return Err(LexerError::InvalidTokenError(ch.to_string()));
            }
        }
        Ok(())
    }

    fn process_int_token(&mut self) -> Result<(), LexerError> {
        let ch = self.read_char()?;
        match ch {
            Chars::Char(_) => {
                if DIGITS.contains(ch.to_char()) {
                    self.current_token.token_type = TokenType::Int;
                    self.current_token.literal.push(ch.to_char());
                } else {
                    return Err(LexerError::InvalidTokenError(ch.to_string()));
                }
            }
            Chars::Dot => {
                self.current_token.token_type = TokenType::Float;
                self.current_token.literal.push('.');
            }
            _ => {
                self.token_list.push(self.current_token.clone());
                self.current_token = Token::default();
                self.position -= 1;
            }
        }

        Ok(())
    }

    fn process_string_token(&mut self) -> Result<(), LexerError> {
        let ch = self.read_char()?;
        match ch {
            Chars::Char(c) => {
                if SIMPLE_ALPHA.contains(c as char) || DIGITS.contains(c as char) {
                    self.current_token.literal.push(ch.to_char());
                } else {
                    self.current_token.token_type = TokenType::String(StringType::ComplexString);
                    self.current_token.literal.push(ch.to_char());
                }
            }
            Chars::Quote => {
                self.token_list.push(self.current_token.clone());
                self.current_token = Token::default();
            }
            _ => {
                self.current_token.token_type = TokenType::String(StringType::ComplexString);
                self.current_token.literal.push(ch.to_char());
            }
        }

        Ok(())
    }

    fn process_reseved_string(&mut self) -> Result<(), LexerError> {
        let ch = self.read_char()?;
        match ch {
            Chars::Char(c) => {
                if SIMPLE_ALPHA.contains(c as char) || DIGITS.contains(c as char) {
                    self.current_token.literal.push(ch.to_char());
                } else {
                    return Err(LexerError::InvalidTokenError(ch.to_string()));
                }
            }
            _ => {
                self.token_list.push(self.current_token.clone());
                self.current_token = Token::default();
                self.position -= 1;
            }
        }

        Ok(())
    }

    fn process_float_token(&mut self) -> Result<(), LexerError> {
        let ch = self.read_char()?;
        match ch {
            Chars::Char(_) => {
                if DIGITS.contains(ch.to_char()) {
                    self.current_token.token_type = TokenType::Float;
                    self.current_token.literal.push(ch.to_char());
                } else {
                    return Err(LexerError::InvalidTokenError(ch.to_string()));
                }
            }
            _ => {
                self.token_list.push(self.current_token.clone());
                self.current_token = Token::default();
                self.position -= 1;
            }
        }

        Ok(())
    }

    fn process_next_token(&mut self) -> Result<(), LexerError> {
        match self.current_token.token_type {
            TokenType::None => {
                self.process_start_token()?;
            }
            TokenType::Int => {
                self.process_int_token()?;
            }
            TokenType::String(_) => {
                self.process_string_token()?;
            }
            TokenType::ReservedString => {
                self.process_reseved_string()?;
            }
            TokenType::Float => {
                self.process_float_token()?;
            }
            _ => {
                self.current_token = Token::default();
            }
        }

        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        self.token_list = Vec::new();
        self.current_token = Token::default();
        while self.position < self.input.len() {
            self.process_next_token()?
        }

        Ok(self.token_list.clone())
    }
}
