// class file format
// https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html

use super::error::ClassParseError;
use crate::parser;

pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl ConstantTag {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Utf8),
            3 => Some(Self::Integer),
            4 => Some(Self::Float),
            5 => Some(Self::Long),
            6 => Some(Self::Double),
            7 => Some(Self::Class),
            8 => Some(Self::String),
            9 => Some(Self::Fieldref),
            10 => Some(Self::Methodref),
            11 => Some(Self::InterfaceMethodref),
            12 => Some(Self::NameAndType),
            15 => Some(Self::MethodHandle),
            16 => Some(Self::MethodType),
            17 => Some(Self::Dynamic),
            18 => Some(Self::InvokeDynamic),
            19 => Some(Self::Module),
            20 => Some(Self::Package),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Constant<'a> {
    Utf8 {
        value: &'a [u8],
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

fn parse_utf8(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, length) = parser::be_u16(input)?;
    let (input, value) = parser::bytes(input, length as usize)?;
    Ok((input, Constant::Utf8 { value }))
}

fn parse_integer(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_i32(input)?;
    Ok((input, Constant::Integer { value }))
}

fn parse_float(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_f32(input)?;
    Ok((input, Constant::Float { value }))
}

fn parse_long(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_i64(input)?;
    Ok((input, Constant::Long { value }))
}

fn parse_double(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, value) = parser::be_f64(input)?;
    Ok((input, Constant::Double { value }))
}

fn parse_class(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, name_index) = parser::be_u16(input)?;
    Ok((input, Constant::Class { name_index }))
}

fn parse_string(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, string_index) = parser::be_u16(input)?;
    Ok((input, Constant::String { string_index }))
}

fn parse_fieldref(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, class_index) = parser::be_u16(input)?;
    let (input, name_and_type_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::Fieldref {
            class_index,
            name_and_type_index,
        },
    ))
}

fn parse_methodref(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, class_index) = parser::be_u16(input)?;
    let (input, name_and_type_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::Methodref {
            class_index,
            name_and_type_index,
        },
    ))
}

fn parse_interface_methodref(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, class_index) = parser::be_u16(input)?;
    let (input, name_and_type_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::InterfaceMethodref {
            class_index,
            name_and_type_index,
        },
    ))
}

fn parse_name_and_type(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, name_index) = parser::be_u16(input)?;
    let (input, descriptor_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::NameAndType {
            name_index,
            descriptor_index,
        },
    ))
}

fn parse_method_handle(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, reference_kind) = parser::be_u8(input)?;
    let (input, reference_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::MethodHandle {
            reference_kind,
            reference_index,
        },
    ))
}

fn parse_method_type(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, descriptor_index) = parser::be_u16(input)?;
    Ok((input, Constant::MethodType { descriptor_index }))
}

fn parse_dynamic(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, bootstrap_method_attr_index) = parser::be_u16(input)?;
    let (input, name_and_type_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::Dynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        },
    ))
}

fn parse_invoke_dynamic(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, bootstrap_method_attr_index) = parser::be_u16(input)?;
    let (input, name_and_type_index) = parser::be_u16(input)?;
    Ok((
        input,
        Constant::InvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        },
    ))
}

fn parse_module(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, name_index) = parser::be_u16(input)?;
    Ok((input, Constant::Module { name_index }))
}

fn parse_package(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, name_index) = parser::be_u16(input)?;
    Ok((input, Constant::Package { name_index }))
}

pub fn parse_constant(input: &[u8]) -> Result<(&[u8], Constant), ClassParseError> {
    let (input, tag) = parser::be_u8(input)?;

    match ConstantTag::from_u8(tag).ok_or(ClassParseError::InvalidConstantTag(tag))? {
        ConstantTag::Utf8 => parse_utf8(input),
        ConstantTag::Integer => parse_integer(input),
        ConstantTag::Float => parse_float(input),
        ConstantTag::Long => parse_long(input),
        ConstantTag::Double => parse_double(input),
        ConstantTag::Class => parse_class(input),
        ConstantTag::String => parse_string(input),
        ConstantTag::Fieldref => parse_fieldref(input),
        ConstantTag::Methodref => parse_methodref(input),
        ConstantTag::InterfaceMethodref => parse_interface_methodref(input),
        ConstantTag::NameAndType => parse_name_and_type(input),
        ConstantTag::MethodHandle => parse_method_handle(input),
        ConstantTag::MethodType => parse_method_type(input),
        ConstantTag::Dynamic => parse_dynamic(input),
        ConstantTag::InvokeDynamic => parse_invoke_dynamic(input),
        ConstantTag::Module => parse_module(input),
        ConstantTag::Package => parse_package(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_utf8() {
        let input = [0x00, 0x03, 0x41, 0x42, 0x43, 0x44];
        let (rest, constant) = parse_utf8(&input).unwrap();
        assert_eq!(rest, &[0x44]);
        assert_eq!(constant, Constant::Utf8 { value: b"ABC" });

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

    #[test]
    fn test_parse_long() {
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let (rest, constant) = parse_long(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::Long {
                value: 0x123456789abcdef0
            }
        );

        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
        let result = parse_long(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );
    }

    #[test]
    fn test_parse_double() {
        let input = [0x3f, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88, 0x87];
        let (rest, constant) = parse_double(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Double { value: 1.234_567 });

        let input = [0x3f, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88];
        let result = parse_double(&input);
        assert_eq!(
            result,
            Err(ClassParseError::ParseError(parser::ParseError::Eof))
        );
    }

    #[test]
    fn test_parse_class() {
        let input = [0x12, 0x34];
        let (rest, constant) = parse_class(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Class { name_index: 0x1234 });
    }

    #[test]
    fn test_parse_string() {
        let input = [0x12, 0x34];
        let (rest, constant) = parse_string(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::String {
                string_index: 0x1234
            }
        );
    }

    #[test]
    fn test_parse_fieldref() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_fieldref(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::Fieldref {
                class_index: 0x1234,
                name_and_type_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_methodref() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_methodref(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::Methodref {
                class_index: 0x1234,
                name_and_type_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_interface_methodref() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_interface_methodref(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::InterfaceMethodref {
                class_index: 0x1234,
                name_and_type_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_name_and_type() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_name_and_type(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::NameAndType {
                name_index: 0x1234,
                descriptor_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_method_handle() {
        let input = [0x01, 0x23, 0x45];
        let (rest, constant) = parse_method_handle(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::MethodHandle {
                reference_kind: 0x01,
                reference_index: 0x2345
            }
        );
    }

    #[test]
    fn test_parse_method_type() {
        let input = [0x12, 0x34];
        let (rest, constant) = parse_method_type(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::MethodType {
                descriptor_index: 0x1234
            }
        );
    }

    #[test]
    fn test_parse_dynamic() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_dynamic(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::Dynamic {
                bootstrap_method_attr_index: 0x1234,
                name_and_type_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_invoke_dynamic() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, constant) = parse_invoke_dynamic(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(
            constant,
            Constant::InvokeDynamic {
                bootstrap_method_attr_index: 0x1234,
                name_and_type_index: 0x5678
            }
        );
    }

    #[test]
    fn test_parse_module() {
        let input = [0x12, 0x34];
        let (rest, constant) = parse_module(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Module { name_index: 0x1234 });
    }

    #[test]
    fn test_parse_package() {
        let input = [0x12, 0x34];
        let (rest, constant) = parse_package(&input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(constant, Constant::Package { name_index: 0x1234 });
    }

    #[test]
    fn test_parse_constant() {
        let input = [0x01, 0x00, 0x03, 0x41, 0x42, 0x43];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Utf8 { .. }));

        let input = [0x03, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Integer { .. }));

        let input = [0x04, 0x3f, 0x9d, 0xf3, 0xb6];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Float { .. }));

        let input = [0x05, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Long { .. }));

        let input = [0x06, 0x3f, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88, 0x87];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Double { .. }));

        let input = [0x07, 0x12, 0x34];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Class { .. }));

        let input = [0x08, 0x12, 0x34];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::String { .. }));

        let input = [0x09, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Fieldref { .. }));

        let input = [0x0a, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Methodref { .. }));

        let input = [0x0b, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::InterfaceMethodref { .. }));

        let input = [0x0c, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::NameAndType { .. }));

        let input = [0x0f, 0x01, 0x23, 0x45];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::MethodHandle { .. }));

        let input = [0x10, 0x12, 0x34];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::MethodType { .. }));

        let input = [0x11, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Dynamic { .. }));

        let input = [0x12, 0x12, 0x34, 0x56, 0x78];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::InvokeDynamic { .. }));

        let input = [0x13, 0x12, 0x34];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Module { .. }));

        let input = [0x14, 0x12, 0x34];
        let (_, constant) = parse_constant(&input).unwrap();
        assert!(matches!(constant, Constant::Package { .. }));

        let input = [99];
        let result = parse_constant(&input);
        assert_eq!(result, Err(ClassParseError::InvalidConstantTag(99)));
    }
}
