// class file format
// https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html

use super::error::ClassParseError;
use crate::parser;

#[derive(Debug, PartialEq)]
pub enum Constant {
    Utf8 {
        value: String,
    },
    Integer {
        value: i32,
    },
    Float {
        value: f32,
    },
    Long {
        value: i64,
    },
    Double {
        value: f64,
    },
    Class {
        name_index: u16,
    },
    String {
        string_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

pub fn parse_utf8(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, length) = parser::be_u16(input)?;
    let (input, value) = parser::bytes(input, length as usize)?;
    let value = String::from_utf8(value.to_vec())?;
    Ok((input, Constant::Utf8 { value }))
}

pub fn parse_integer(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_i32(input)?;
    Ok((input, Constant::Integer { value }))
}

pub fn parse_float(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_f32(input)?;
    Ok((input, Constant::Float { value }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_utf8() {
        let input = [0x00, 0x03, 0x41, 0x42, 0x43, 0x44];
        let (rest, constant) = parse_utf8(&input).unwrap();
        assert_eq!(rest, &[0x44]);
        assert_eq!(
            constant,
            Constant::Utf8 {
                value: "ABC".to_string()
            }
        );

        let input = [0x00];
        let result = parse_utf8(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );

        let input = [0x00, 0x03, 0x41, 0x42];
        let result = parse_utf8(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );

        // invalid utf8
        let input = [0x00, 0x03, 0x41, 0x42, 0x80];
        let result = parse_utf8(&input);
        assert!(matches!(result, Err(ClassParseError::Utf8Error(_))));
    }

    #[test]
    fn test_parse_integer() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_integer(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Integer { value: 0x12345678 });

        let input = [0x12, 0x34, 0x56];
        let result = parse_integer(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );
    }

    #[test]
    fn test_parse_float() {
        let input = [0x3f, 0x9d, 0xf3, 0xb6];
        let (rest, constant) = parse_float(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Float { value: 1.234 });

        let input = [0x3f, 0x9d, 0xf3];
        let result = parse_float(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );
    }
}
