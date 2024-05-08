use std::string::FromUtf8Error;

use crate::parser;

#[derive(Debug, PartialEq)]
pub enum ClassParseError {
    ParseError(parser::ParseError),
    Utf8Error(FromUtf8Error),
    InvalidConstantTag(u8),
    InvalidConstantPoolIndex(u16),
    InvalidFieldDescriptor,
}

impl From<parser::ParseError> for ClassParseError {
    fn from(error: parser::ParseError) -> Self {
        ClassParseError::ParseError(error)
    }
}

impl From<FromUtf8Error> for ClassParseError {
    fn from(error: FromUtf8Error) -> Self {
        ClassParseError::Utf8Error(error)
    }
}
