use crate::parser::{be_u16, be_u32, bytes};

use super::constant::Constant;
use super::error::ClassParseError;

#[derive(Debug)]
pub enum AttributeName {
    Code,
    // WIP
}

impl AttributeName {
    pub fn from_name(name: &[u8]) -> Option<Self> {
        match name {
            b"Code" => Some(Self::Code),
            // WIP
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    Unknown {
        data: &'a [u8],
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: &'a [u8],
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute<'a>>,
    },
}

fn parser_exception_table_entry(
    input: &[u8],
) -> Result<(&[u8], ExceptionTableEntry), ClassParseError> {
    let (input, start_pc) = be_u16(input)?;
    let (input, end_pc) = be_u16(input)?;
    let (input, handler_pc) = be_u16(input)?;
    let (input, catch_type) = be_u16(input)?;
    Ok((
        input,
        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    ))
}

fn parse_code<'a, F>(
    input: &'a [u8],
    constant_pool: &[Constant],
    parse_attribute: F,
) -> Result<(&'a [u8], Attribute<'a>), ClassParseError>
where
    F: Fn(&'a [u8], &[Constant]) -> Result<(&'a [u8], Attribute<'a>), ClassParseError>,
{
    let (input, max_stack) = be_u16(input)?;
    let (input, max_locals) = be_u16(input)?;
    let (input, code_length) = be_u32(input)?;
    let (input, code) = bytes(input, code_length as usize)?;
    let (input, exception_table) = {
        let (input, exception_table_length) = be_u16(input)?;
        let mut exception_table = Vec::new();
        let mut input = input;
        for _ in 0..exception_table_length {
            let (new_input, entry) = parser_exception_table_entry(input)?;
            input = new_input;
            exception_table.push(entry);
        }
        (input, exception_table)
    };
    let (input, attributes) = {
        let (input, attributes_count) = be_u16(input)?;
        let mut attributes = Vec::new();
        let mut input = input;
        for _ in 0..attributes_count {
            let (new_input, attribute) = parse_attribute(input, constant_pool)?;
            input = new_input;
            attributes.push(attribute);
        }
        (input, attributes)
    };
    let attribute = Attribute::Code {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    };
    Ok((input, attribute))
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
        _ => {
            let (input, data) = bytes(input, attribute_length as usize)?;
            (input, Attribute::Unknown { data })
        }
    };
    Ok((input, attribute))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_exception_table_entry() {
        let input = [
            0x00, 0x01, // start_pc
            0x00, 0x02, // end_pc
            0x00, 0x03, // handler_pc
            0x00, 0x04, // catch_type
            0x12, 0x34, // rest
        ];
        let (rest, entry) = parser_exception_table_entry(&input).unwrap();
        assert_eq!(rest, &[0x12, 0x34]);
        assert_eq!(
            entry,
            ExceptionTableEntry {
                start_pc: 1,
                end_pc: 2,
                handler_pc: 3,
                catch_type: 4,
            }
        );
    }

    #[test]
    fn test_parse_code() {
        let input = [
            0x00, 0x01, // max_stack
            0x00, 0x02, // max_locals
            0x00, 0x00, 0x00, 0x04, // code_length
            0x40, 0x41, 0x42, 0x43, // code
            0x00, 0x01, // exception_table_length
            0x10, 0x11, // start_pc
            0x12, 0x13, // end_pc
            0x14, 0x15, // handler_pc
            0x16, 0x17, // catch_type
            0x00, 0x00, // attributes_count
            0x12, 0x34, // rest
        ];
        let constant_pool = vec![];
        let (rest, attribute) = parse_code(&input, &constant_pool, parse_attribute).unwrap();
        assert_eq!(rest, &[0x12, 0x34]);
        assert_eq!(
            attribute,
            Attribute::Code {
                max_stack: 1,
                max_locals: 2,
                code: &[0x40, 0x41, 0x42, 0x43],
                exception_table: vec![ExceptionTableEntry {
                    start_pc: 0x1011,
                    end_pc: 0x1213,
                    handler_pc: 0x1415,
                    catch_type: 0x1617,
                }],
                attributes: vec![],
            }
        );
    }

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
                data: &[0x00, 0x01, 0x02, 0x03]
            }
        );
    }
}
