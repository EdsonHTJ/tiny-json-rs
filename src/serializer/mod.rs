mod deserializers;
mod serializers;

use crate::lexer::{Lexer, LexerError, Token};
use crate::mapper::{Mapper, MapperError, Value};
use alloc::format;
use alloc::string::String;
use core::str::FromStr;

pub trait Deserialize: Sized {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError>;
}

pub trait Serialize: Sized {
    fn serialize(&self) -> Value;
}

#[derive(Debug)]
pub enum DecodeError {
    MapperError(MapperError),
    LexerError(LexerError),
    UnexpectedType,
    ParseError,
}

impl From<MapperError> for DecodeError {
    fn from(error: MapperError) -> Self {
        DecodeError::MapperError(error)
    }
}

impl From<LexerError> for DecodeError {
    fn from(error: LexerError) -> Self {
        DecodeError::LexerError(error)
    }
}

impl Token {
    pub fn to<T>(&self) -> Result<T, DecodeError>
    where
        T: FromStr,
    {
        T::from_str(&self.literal).map_err(|_| DecodeError::ParseError)
    }
}

impl Value {
    pub fn get_value<T>(&self, key: &str) -> Result<T, DecodeError>
    where
        T: Deserialize,
    {
        let option_val = match self {
            Value::Object(object) => object.get(key),
            _ => None,
        };

        let res = T::deserialize(option_val)?;

        Ok(res)
    }

    pub fn encode_json(&self) -> String {
        let mut output = String::new();
        match self {
            Value::Object(object) => {
                output += "{";
                let mut first = true;
                for (key, value) in object {
                    if !first {
                        output += ",";
                    }
                    first = false;
                    output += &format!("\"{}\":{}", key, value.encode_json());
                }
                output += "}";
            }
            Value::Token(t) => match t.token_type {
                crate::lexer::TokenType::String(_) => {
                    output += &format!("\"{}\"", t.literal);
                }
                _ => {
                    output += &format!("{}", t.literal);
                }
            },
            Value::Array(a) => {
                output += "[";
                let mut first = true;
                for value in a {
                    if !first {
                        output += ",";
                    }
                    first = false;
                    output += &format!("{}", value.encode_json());
                }
                output += "]";
            }
        }

        output
    }
}

pub fn decode<T>(input_str: String) -> Result<T, DecodeError>
where
    T: Deserialize,
{
    let mut lexer = Lexer::new(input_str);
    let tokens = lexer.tokenize()?;
    let mut mapper = Mapper::new(tokens);
    let object = mapper.parse_object()?;
    let value = Value::Object(object);
    Ok(T::deserialize(Some(&value))?)
}

pub fn encode<T>(input: T) -> String
where
    T: Serialize,
{
    input.serialize().encode_json()
}

#[cfg(test)]
pub mod test {
    use alloc::string::{String, ToString};
    use alloc::vec::Vec;
    use tiny_json_derive::{Deserialize, Serialize};

    use crate::alloc::borrow::ToOwned;
    use crate::mapper;
    use crate::serializer;

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct A {
        #[Rename = "aJson"]
        pub a: i32,
        pub b: String,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct B {
        pub a: i32,
        pub b: Vec<String>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct C {
        pub a: i32,
        pub b: Vec<A>,
    }

    #[test]
    pub fn test_deserialize() {
        const JSON: &str = r#"
        {
            "aJson": 1,
            "b": "Hello"
        }"#;

        let a: A = super::decode(JSON.to_string()).unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b, "Hello");
    }

    #[test]
    pub fn test_desserialize_vec() {
        const JSON: &str = r#"
        {
            "a": 1,
            "b": ["Hello","world"]
        }"#;

        let a: B = super::decode(JSON.to_string()).unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b.len(), 2);
        assert_eq!(a.b[0], "Hello");
        assert_eq!(a.b[1], "world");
    }

    #[test]
    pub fn test_encode_json() {
        let a = A {
            a: 1,
            b: "Hello".to_string(),
        };

        let json = super::encode(a);
        assert_eq!(json, r#"{"aJson":1,"b":"Hello"}"#);
    }

    #[test]
    pub fn test_nested() {
        const JSON: &str = r#"
        {
            "a": 1,
            "b": [
                {
                    "aJson": 1,
                    "b": "Hello"
                },
                {
                    "aJson": 2,
                    "b": "World"
                }
            ]
        }"#;

        let a: C = super::decode(JSON.to_string()).unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b.len(), 2);
        assert_eq!(a.b[0].a, 1);
        assert_eq!(a.b[0].b, "Hello");
        assert_eq!(a.b[1].a, 2);
        assert_eq!(a.b[1].b, "World");
    }
}
