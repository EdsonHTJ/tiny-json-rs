use alloc::string::String;
use core::str::FromStr;
use crate::lexer::{Lexer, LexerError, Token};
use crate::mapper::{Mapper, MapperError, Value};

pub trait Deserialize: Sized {
    fn deserialize(value: Value) -> Result<Self, DecodeError>;
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

pub fn decode<T>(input_str: String) -> Result<T,DecodeError>
where
    T: Deserialize,
{
    let mut lexer = Lexer::new(input_str);
    let tokens = lexer.tokenize()?;
    let mut mapper = Mapper::new(tokens);
    let object = mapper.parse_object()?;
    let value = Value::Object(object);
    Ok(T::deserialize(value)?)
}

impl Token {
    fn to<T>(&self) -> Result<T, DecodeError>
        where
            T: FromStr,
    {
        T::from_str(&self.literal).map_err(|_| DecodeError::ParseError)
    }
}


#[cfg(test)]
pub mod test {
    use alloc::string::{String, ToString};
    use crate::mapper::Value;
    use crate::serializer::{DecodeError, Deserialize};

    pub struct A {
        pub a: i32,
        pub b: String,
    }

    impl Deserialize for A {
        fn deserialize(value: Value) -> Result<Self, DecodeError> {
            match value {
                Value::Object(obj) => {
                    // Extract the value for field 'a', ensure it's a Token, and parse it as i32
                    let a = match obj.get("a") {
                        Some(Value::Token(token)) => token.to::<i32>()?,
                        _ => return Err(DecodeError::UnexpectedType),
                    };

                    // Extract the value for field 'b', ensure it's a Token, and parse it as String
                    let b = match obj.get("b") {
                        Some(Value::Token(token)) => token.to::<String>()?,
                        _ => return Err(DecodeError::UnexpectedType),
                    };

                    Ok(A { a, b })
                },
                _ => Err(DecodeError::UnexpectedType),
            }
        }
    }

    const JSON: &str = r#"
    {
        "a": 1,
        "b": "Hello"
    }"#;

    #[test]
    pub fn test_deserialize() {
        let a: A = super::decode(JSON.to_string()).unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b, "Hello");
    }

}