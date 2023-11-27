//No Std crate
#![no_std]
extern crate alloc;

use alloc::string::String;

pub mod lexer;
pub mod     mapper;
pub mod serializer;

pub fn encode<T>(value: T) -> String
where
    T: serializer::Serialize,
{
    serializer::encode(value)
}

pub fn decode<T>(input_str: String) -> Result<T, serializer::DecodeError>
where
    T: serializer::Deserialize,
{
    serializer::decode(input_str)
}
