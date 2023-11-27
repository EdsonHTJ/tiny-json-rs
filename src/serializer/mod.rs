mod desserializers;

use alloc::string::String;
use core::str::FromStr;
use crate::lexer::{Lexer, LexerError, Token};
use crate::mapper::{Mapper, MapperError, Value};

pub trait Deserialize: Sized {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError>;
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
    where T: Deserialize
    {
        let option_val =match self {
            Value::Object(object) => object.get(key),
            _ => None
        };

        let res = T::deserialize(option_val)?;

        Ok(res)
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
    Ok(T::deserialize(Some(&value))?)
}

#[cfg(test)]
pub mod test {
    use alloc::string::{String, ToString};
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::mapper::Value;
    use crate::serializer::{DecodeError, Deserialize};

    pub struct A {
        pub a: i32,
        pub b: String,
    }

    pub struct B {
        pub a: i32,
        pub b: Vec<String>,
    }

    impl Deserialize for B {
        fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
            let value = match value {
                None => {return Ok(B {a: 0, b: vec![]})}
                Some(v) => {v}
            };

            let a = value.get_value::<i32>("a")?;
            let b = value.get_value::<Vec<String>>("b")?;
            Ok(B { a, b })
        }
    }

    impl Deserialize for A {
        fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
            let value = match value {
                None => {return Ok(A {a: 0, b: "".to_string()})}
                Some(v) => {v}
            };

            let a = value.get_value::<i32>("a")?;
            let b = value.get_value::<String>("b")?;
            Ok(A { a, b })
        }
    }

    #[test]
    pub fn test_deserialize() {

        const JSON: &str = r#"
        {
            "a": 1,
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

}