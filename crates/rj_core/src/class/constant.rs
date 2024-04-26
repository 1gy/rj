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
}
