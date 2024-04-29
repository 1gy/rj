use super::super::constant::Constant;
use super::super::error::ClassParseError;
use crate::parser::{be_u16, be_u32, bytes};

#[derive(Debug, PartialEq)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
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

#[derive(Debug, PartialEq)]
pub struct Code<'a, A> {
    max_stack: u16,
    max_locals: u16,
    code: &'a [u8],
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<A>,
}

pub fn parse_code<'a, A, F>(
    input: &'a [u8],
    constant_pool: &[Constant],
    parse_attribute: F,
) -> Result<(&'a [u8], A), ClassParseError>
where
    A: From<Code<'a, A>>,
    F: Fn(&'a [u8], &[Constant]) -> Result<(&'a [u8], A), ClassParseError>,
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
    let attribute = Code {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    };
    Ok((input, attribute.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestAttribute<'a>(Code<'a, TestAttribute<'a>>);

    impl<'a> From<Code<'a, TestAttribute<'a>>> for TestAttribute<'a> {
        fn from(code: Code<'a, TestAttribute<'a>>) -> Self {
            TestAttribute(code)
        }
    }

    fn dummy_parse_attribute<'a>(
        _input: &'a [u8],
        _constant_pool: &[Constant],
    ) -> Result<(&'a [u8], TestAttribute<'a>), ClassParseError> {
        unreachable!()
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
        let (rest, attribute) = parse_code(&input, &constant_pool, dummy_parse_attribute).unwrap();
        assert_eq!(rest, &[0x12, 0x34]);
        assert_eq!(
            attribute.0,
            Code {
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
}
