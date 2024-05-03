use crate::parser;

#[derive(Debug, PartialEq)]
pub enum InstructionParseError {
    ParseError(parser::ParseError),
    UnknownInstruction(u8),
}

impl From<parser::ParseError> for InstructionParseError {
    fn from(error: parser::ParseError) -> Self {
        InstructionParseError::ParseError(error)
    }
}
