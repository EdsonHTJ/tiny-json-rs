use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const SIMPLE_ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
const DIGITS: &str = "0123456789";
//  Json tokens
pub enum Chars{
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
        }
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
    pub line : usize,
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
        }
    }

    fn read_char(&mut self) -> Result<Chars, LexerError> {
        if self.position >= self.input.len() {
            return Err(LexerError::OutOfRangeError);
        }

        let ch = self.input.as_bytes()[self.position];
        self.position += 1;
        return Ok(Chars::from(ch));
    }

    fn process_start_token(&mut self) -> Result<(), LexerError>{
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
                self.current_token.literal = ch.to_string();
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

    fn process_int_token(&mut self) -> Result<(), LexerError>{
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

    fn process_string_token(&mut self) -> Result<(), LexerError>{
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
                self.current_token.literal.push(ch.to_char());
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

    fn process_reseved_string(&mut self) -> Result<(), LexerError>{
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

    fn process_float_token(&mut self) -> Result<(), LexerError>{
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


    fn process_next_token(&mut self) ->Result<(), LexerError> {
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>,LexerError> {
        self.token_list = Vec::new();
        self.current_token = Token::default();
        while self.position < self.input.len() {
            self.process_next_token()?
        }

        Ok(self.token_list.clone())
    }
}

#[cfg(test)]
mod test {
    use alloc::string::ToString;
    use super::*;

    #[test]
    fn test_lexer() {
        let input = r#"
        {
            "name": "John",
            "age": 30,
            "isActive": True,
            "cars": [
                {
                    "name": "Ford",
                    "plate": "20-13f"
                },
                {
                    "name": "Fiat",
                    "plate": "20-13f"
                }
            ]
        }
        "#.to_string();

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 37);
        assert_eq!(tokens[0].token_type, TokenType::LBrace);
        assert_eq!(tokens[0].literal, "{");
        assert_eq!(tokens[1].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[1].literal, "\"name\"");
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[2].literal, ":");
        assert_eq!(tokens[3].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[3].literal, "\"John\"");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[4].literal, ",");
        assert_eq!(tokens[5].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[5].literal, "\"age\"");
        assert_eq!(tokens[6].token_type, TokenType::Colon);
        assert_eq!(tokens[6].literal, ":");
        assert_eq!(tokens[7].token_type, TokenType::Int);
        assert_eq!(tokens[7].literal, "30");
        assert_eq!(tokens[8].token_type, TokenType::Comma);
        assert_eq!(tokens[8].literal, ",");
        assert_eq!(tokens[9].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[9].literal, "\"isActive\"");
        assert_eq!(tokens[10].token_type, TokenType::Colon);
        assert_eq!(tokens[10].literal, ":");
        assert_eq!(tokens[11].token_type, TokenType::ReservedString);
        assert_eq!(tokens[11].literal, "True");
        assert_eq!(tokens[12].token_type, TokenType::Comma);
        assert_eq!(tokens[12].literal, ",");
        assert_eq!(tokens[13].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[13].literal, "\"cars\"");
        assert_eq!(tokens[14].token_type, TokenType::Colon);
        assert_eq!(tokens[14].literal, ":");
        assert_eq!(tokens[15].token_type, TokenType::LBracket);
        assert_eq!(tokens[15].literal, "[");
        assert_eq!(tokens[16].token_type, TokenType::LBrace);
        assert_eq!(tokens[16].literal, "{");
        assert_eq!(tokens[17].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[17].literal, "\"name\"");
        assert_eq!(tokens[18].token_type, TokenType::Colon);
        assert_eq!(tokens[18].literal, ":");
        assert_eq!(tokens[19].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[19].literal, "\"Ford\"");
        assert_eq!(tokens[20].token_type, TokenType::Comma);
        assert_eq!(tokens[20].literal, ",");
        assert_eq!(tokens[21].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[21].literal, "\"plate\"");
        assert_eq!(tokens[22].token_type, TokenType::Colon);
        assert_eq!(tokens[22].literal, ":");
        assert_eq!(tokens[23].token_type, TokenType::String(StringType::ComplexString));
        assert_eq!(tokens[23].literal, "\"20-13f\"");
        assert_eq!(tokens[24].token_type, TokenType::RBrace);
        assert_eq!(tokens[24].literal, "}");
        assert_eq!(tokens[25].token_type, TokenType::Comma);
        assert_eq!(tokens[25].literal, ",");
        assert_eq!(tokens[26].token_type, TokenType::LBrace);
        assert_eq!(tokens[26].literal, "{");
        assert_eq!(tokens[27].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[27].literal, "\"name\"");
        assert_eq!(tokens[28].token_type, TokenType::Colon);
        assert_eq!(tokens[28].literal, ":");
        assert_eq!(tokens[29].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[29].literal, "\"Fiat\"");
        assert_eq!(tokens[30].token_type, TokenType::Comma);
        assert_eq!(tokens[30].literal, ",");
        assert_eq!(tokens[31].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[31].literal, "\"plate\"");
        assert_eq!(tokens[32].token_type, TokenType::Colon);
        assert_eq!(tokens[32].literal, ":");
        assert_eq!(tokens[33].token_type, TokenType::String(StringType::ComplexString));
        assert_eq!(tokens[33].literal, "\"20-13f\"");
        assert_eq!(tokens[34].token_type, TokenType::RBrace);
        assert_eq!(tokens[34].literal, "}");
        assert_eq!(tokens[35].token_type, TokenType::RBracket);
        assert_eq!(tokens[35].literal, "]");
        assert_eq!(tokens[36].token_type, TokenType::RBrace);
        assert_eq!(tokens[36].literal, "}");
    }

    #[test]
    fn test_simple() {
        const JSON: &str = r#"
        {
            "a": 1,
            "b": "Hello",
        }"#;

        let mut lexer = Lexer::new(JSON.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::LBrace);
        assert_eq!(tokens[0].literal, "{");
        assert_eq!(tokens[1].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[1].literal, "\"a\"");
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[2].literal, ":");
        assert_eq!(tokens[3].token_type, TokenType::Int);
        assert_eq!(tokens[3].literal, "1");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[4].literal, ",");
        assert_eq!(tokens[5].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[5].literal, "\"b\"");
        assert_eq!(tokens[6].token_type, TokenType::Colon);
        assert_eq!(tokens[6].literal, ":");
        assert_eq!(tokens[7].token_type, TokenType::String(StringType::SimpleString));
        assert_eq!(tokens[7].literal, "\"Hello\"");
        assert_eq!(tokens[8].token_type, TokenType::Comma);
        assert_eq!(tokens[8].literal, ",");
        assert_eq!(tokens[9].token_type, TokenType::RBrace);
        assert_eq!(tokens[9].literal, "}");
    }
}

