mod code;
mod line_number_table;
mod source_file;

pub use self::code::{parse_code, Code};
pub use self::line_number_table::{parse_line_number_table, LineNumberTable};
pub use self::source_file::{parse_source_file, SourceFile};

use super::constant::Constant;
use super::error::ClassParseError;
use crate::parser::{be_u16, be_u32, bytes};

#[derive(Debug)]
pub enum AttributeName {
    Code,
    LineNumberTable,
    SourceFile,
    // WIP
}

impl AttributeName {
    pub fn from_name(name: &[u8]) -> Option<Self> {
        match name {
            b"Code" => Some(Self::Code),
            b"LineNumberTable" => Some(Self::LineNumberTable),
            b"SourceFile" => Some(Self::SourceFile),
            // WIP
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    Unknown {
        attribute_name_index: u16,
        data: &'a [u8],
    },
    Code(Code<'a, Attribute<'a>>),
    LineNumberTable(LineNumberTable),
    SourceFile(SourceFile),
}

impl<'a> From<Code<'a, Attribute<'a>>> for Attribute<'a> {
    fn from(code: Code<'a, Attribute<'a>>) -> Self {
        Attribute::Code(code)
    }
}

impl<'a> From<LineNumberTable> for Attribute<'a> {
    fn from(line_number_table: LineNumberTable) -> Self {
        Attribute::LineNumberTable(line_number_table)
    }
}

impl<'a> From<SourceFile> for Attribute<'a> {
    fn from(source_file: SourceFile) -> Self {
        Attribute::SourceFile(source_file)
    }
}

pub fn parse_attribute<'a>(
    input: &'a [u8],
    constant_pool: &[Constant],
) -> Result<(&'a [u8], Attribute<'a>), ClassParseError> {
    let (input, attribute_name_index) = be_u16(input)?;
    let name = match constant_pool.get(attribute_name_index as usize - 1) {
        Some(Constant::Utf8 { value }) => *value,
        _ => {
            return Err(ClassParseError::InvalidConstantPoolIndex(
                attribute_name_index,
            ))
        }
    };
    let (input, attribute_length) = be_u32(input)?;
    let (input, attribute) = match AttributeName::from_name(name) {
        Some(AttributeName::Code) => parse_code(input, constant_pool, parse_attribute)?,
        Some(AttributeName::LineNumberTable) => parse_line_number_table(input)?,
        Some(AttributeName::SourceFile) => parse_source_file(input)?,
        _ => {
            let (input, data) = bytes(input, attribute_length as usize)?;
            (
                input,
                Attribute::Unknown {
                    attribute_name_index,
                    data,
                },
            )
        }
    };
    Ok((input, attribute))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        let input = [
            0x00, 0x01, // attribute_name_index
            0x00, 0x00, 0x00, 0x04, // attribute_length
            0x00, 0x01, 0x02, 0x03, // data
            0x12, 0x34, // rest
        ];
        let constant_pool = vec![Constant::Utf8 {
            value: b"Unknown_Attribute_Name",
        }];
        let (rest, attribute) = parse_attribute(&input, &constant_pool).unwrap();
        assert_eq!(rest, &[0x12, 0x34]);
        assert_eq!(
            attribute,
            Attribute::Unknown {
                attribute_name_index: 0x0001,
                data: &[0x00, 0x01, 0x02, 0x03]
            }
        );
    }
}
