use super::super::error::ClassParseError;
use crate::parser::{be_u8, take_until};

#[derive(Debug, PartialEq)]
pub enum FieldType<'a> {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Object(&'a [u8]),
    Array(Box<FieldType<'a>>),
}

fn parse_base_type(input: &[u8]) -> Result<(&[u8], FieldType), ClassParseError> {
    let (rest, tag) = be_u8(input)?;
    match tag {
        b'B' => Ok((rest, FieldType::Byte)),
        b'C' => Ok((rest, FieldType::Char)),
        b'D' => Ok((rest, FieldType::Double)),
        b'F' => Ok((rest, FieldType::Float)),
        b'I' => Ok((rest, FieldType::Int)),
        b'J' => Ok((rest, FieldType::Long)),
        b'S' => Ok((rest, FieldType::Short)),
        b'Z' => Ok((rest, FieldType::Boolean)),
        _ => Err(ClassParseError::InvalidFieldDescriptor),
    }
}

fn parse_object_type(input: &[u8]) -> Result<(&[u8], FieldType), ClassParseError> {
    let (rest, tag) = be_u8(input)?;
    if tag != b'L' {
        return Err(ClassParseError::InvalidFieldDescriptor);
    }
    let (rest, class_name) = take_until(rest, b";")?;
    Ok((rest, FieldType::Object(class_name)))
}

fn parse_array_type(input: &[u8]) -> Result<(&[u8], FieldType), ClassParseError> {
    let (rest, tag) = be_u8(input)?;
    if tag != b'[' {
        return Err(ClassParseError::InvalidFieldDescriptor);
    }
    let (rest, field_type) = parse_field_type(rest)?;
    Ok((rest, FieldType::Array(Box::new(field_type))))
}

pub fn parse_field_type(input: &[u8]) -> Result<(&[u8], FieldType), ClassParseError> {
    parse_base_type(input)
        .or_else(|_| parse_object_type(input))
        .or_else(|_| parse_array_type(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field_type() {
        let data = b"Bxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Byte);

        let data = b"Cxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Char);

        let data = b"Dxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Double);

        let data = b"Fxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Float);

        let data = b"Ixxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Int);

        let data = b"Jxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Long);

        let data = b"Sxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Short);

        let data = b"Zxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Boolean);

        let data = b"Lcom/example/Example;xxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Object(b"com/example/Example"));

        let data = b"[Ixxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(field_type, FieldType::Array(Box::new(FieldType::Int)));

        let data = b"[[[Dxxx";
        let (rest, field_type) = parse_field_type(data).unwrap();
        assert_eq!(rest, b"xxx");
        assert_eq!(
            field_type,
            FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Array(
                Box::new(FieldType::Double)
            )))))
        );

        let data = b"Xxxx";
        assert_eq!(
            parse_field_type(data),
            Err(ClassParseError::InvalidFieldDescriptor)
        );
    }
}
