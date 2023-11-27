use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::any::Any;
use core::fmt::Display;
use crate::lexer::{StringType, Token, TokenType};

pub type Object = BTreeMap<String, Value>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Token(Token),
    Object(Object),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Token(token) => write!(f, "{}", token.literal),
            Value::Object(object) => write!(f, "{:?}", object),
            Value::Array(array) => write!(f, "{:?}", array),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MapperError {
    UnexpectedToken(Token)
}

pub struct Mapper {
    pub token_list: Vec<Token>,
    pub position: usize,
}

impl Mapper {
    pub fn new(token_list: Vec<Token>) -> Self {
        Mapper {
            token_list,
            position: 0,
        }
    }

    fn read_token(&mut self) -> Token {
        let token = self.token_list[self.position].clone();
        self.position += 1;
        token
    }

    fn peek_token(&mut self) -> Token {
        self.token_list[self.position].clone()
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Token,MapperError> {
        let token = self.read_token();
        if token.token_type != token_type {
            return Err(MapperError::UnexpectedToken(token));
        }
        Ok(token)
    }

    fn parse_array(&mut self) -> Result<Vec<Value>, MapperError> {
        let mut array = Vec::new();
        loop {
            let token = self.peek_token();
            if token.token_type == TokenType::RBracket {
                break;
            }

            if token.token_type == TokenType::LBrace {
                let object = self.parse_object()?;
                array.push(Value::Object(object));
            }else {
                let token = self.read_token();
                array.push(Value::Token(token));
            }

            let token = self.read_token();
            match token.token_type {
                TokenType::Comma => continue,
                TokenType::RBracket => break,
                _ => return Err(MapperError::UnexpectedToken(token))
            }
        }
        Ok(array)
    }

    fn parse_value(&mut self) -> Result<(String, Value), MapperError> {
        let key_token = self.expect(TokenType::String(StringType::SimpleString))?;
        self.expect(TokenType::Colon)?;
        let value_token = self.read_token();
        return match value_token.token_type {
            TokenType::String(_) => {
                //Trim " from value
                let mut value_token = value_token;
                value_token.literal = value_token.literal[1..value_token.literal.len()-1].to_string();
                let value = Value::Token(value_token);
                Ok((key_token.literal, value))
            },
            TokenType::Int | TokenType::Float | TokenType::ReservedString => {
                let value = Value::Token(value_token);
                Ok((key_token.literal, value))
            },
            TokenType::LBrace => {
                let value = Value::Object(self.parse_object()?);
                Ok((key_token.literal, value))
            },
            TokenType::LBracket => {
                let value = Value::Array(self.parse_array()?);
                Ok((key_token.literal, value))
            },
            _ => {
                Err(MapperError::UnexpectedToken(value_token))
            }
        }
    }

    pub fn parse_object(&mut self) -> Result<Object,MapperError> {
        let mut object = BTreeMap::new();
        self.expect(TokenType::LBrace)?;
        loop {
            let token = self.peek_token();
            if token.token_type == TokenType::RBrace {
                break;
            }

            let (key, value) = self.parse_value()?;
            //Trim " from key
            let key = key[1..key.len()-1].to_string();
            object.insert(key, value);
            let token = self.read_token();
            match token.token_type {
                TokenType::Comma => continue,
                TokenType::RBrace => break,
                _ => return Err(MapperError::UnexpectedToken(token))
            }
        }
        Ok(object)
    }
}

#[cfg(test)]
pub mod test {
    use alloc::string::{String, ToString};
    use alloc::vec::Vec;

    #[test]
    pub fn test_mapper() {
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

        let token_list = crate::lexer::Lexer::new(input).tokenize().unwrap();
        let mut mapper = crate::mapper::Mapper::new(token_list);
        let object = mapper.parse_object().unwrap();
        let keys = object.keys().collect::<Vec<&String>>();
        assert_eq!(object["name"].to_string(), "John");
        assert_eq!(object["age"].to_string(), "30");
        assert_eq!(object["isActive"].to_string(), "True");

        let cars = match object["cars"] {
            crate::mapper::Value::Array(ref cars) => cars,
            _ => panic!("Expected array")
        };

        let car1 = match cars[0] {
            crate::mapper::Value::Object(ref car) => car,
            _ => panic!("Expected object")
        };

        let car2 = match cars[1] {
            crate::mapper::Value::Object(ref car) => car,
            _ => panic!("Expected object")
        };

        assert_eq!(car1["name"].to_string(), "Ford");
        assert_eq!(car1["plate"].to_string(), "20-13f");
        assert_eq!(car2["name"].to_string(), "Fiat");
        assert_eq!(car2["plate"].to_string(), "20-13f");
    }
}