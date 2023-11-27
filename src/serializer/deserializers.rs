use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;
use crate::mapper::Value;
use crate::serializer::{DecodeError, Deserialize};

pub fn parse_token<T>(value: Option<&Value>) -> Result<T, DecodeError>
where
    T: FromStr,
{
    let value = match value {
        None => {return T::from_str("").map_err(|_| DecodeError::ParseError)}
        Some(v) => {v}
    };

    let token = match value {
        Value::Token(t) => {t}
        _ => {return Err(DecodeError::UnexpectedType)}
    };

    return token.to::<T>();
}

impl Deserialize for u8 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for u16 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for u32 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for u64 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for usize {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for i8 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for i16 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for i32 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for i64 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for isize {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for f32 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for f64 {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for bool {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for char {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl Deserialize for String {
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        parse_token(value)
    }
}

impl<T> Deserialize for Option<T>
where
    T: Deserialize,
{
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        match value {
            None => Ok(None),
            Some(v) => {
                let res = T::deserialize(Some(v))?;
                Ok(Some(res))
            }
        }
    }
}

impl<T> Deserialize for Box<T>
where
    T: Deserialize,
{
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        let res = T::deserialize(value)?;
        Ok(Box::new(res))
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize
{
    fn deserialize(value: Option<&Value>) -> Result<Self, DecodeError> {
        let value = match value {
            None => {return Ok(Vec::new())}
            Some(v) => {v}
        };
        match value {
            Value::Array(array) => {
                let mut vec = Vec::new();
                for item in array {
                    let res = T::deserialize(Some(item))?;
                    vec.push(res);
                }
                Ok(vec)
            },
            _ => Err(DecodeError::UnexpectedType),
        }
    }
}