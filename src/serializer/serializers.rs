use crate::lexer::{StringType, Token, TokenType};
use crate::mapper::Value;
use crate::serializer::Serialize;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

impl Serialize for u8 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for usize {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for i8 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for i16 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Int,
            literal: self.to_string(),
        })
    }
}

impl Serialize for f32 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Float,
            literal: self.to_string(),
        })
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::Float,
            literal: self.to_string(),
        })
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::ReservedString,
            literal: self.to_string(),
        })
    }
}

impl Serialize for String {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::String(StringType::ComplexString),
            literal: self.to_string(),
        })
    }
}

impl Serialize for char {
    fn serialize(&self) -> Value {
        Value::Token(Token {
            token_type: TokenType::String(StringType::SimpleString),
            literal: self.to_string(),
        })
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Value {
        match self {
            Some(val) => val.serialize(),
            None => Value::Token(Token {
                token_type: TokenType::ReservedString,
                literal: String::from("null"),
            }),
        }
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Value {
        let mut array = Vec::new();
        for val in self {
            array.push(val.serialize());
        }
        Value::Array(array)
    }
}
