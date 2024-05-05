use super::super::error::ClassParseError;
use super::field_descriptor::{parse_field_type, FieldType};
use crate::parser::be_u8;

#[derive(Debug, PartialEq)]
pub struct MethodDescriptor<'a> {
    pub parameters: Vec<FieldType<'a>>,
    pub return_type: FieldType<'a>,
}

fn parse_return_type(input: &[u8]) -> Result<(&[u8], FieldType), ClassParseError> {
    let (rest, tag) = be_u8(input)?;
    match tag {
        b'V' => Ok((rest, FieldType::Void)),
        _ => {
            let (rest, field_type) = parse_field_type(rest)?;
            Ok((rest, field_type))
        }
    }
}

pub fn parse_method_descriptor(input: &[u8]) -> Result<(&[u8], MethodDescriptor), ClassParseError> {
    let (rest, _) = be_u8(input)?;
    let (rest, parameters) = parse_field_type(rest)?;
    let mut parameter_types = vec![parameters];
    let mut rest = rest;
    while let Ok((new_rest, field_type)) = parse_field_type(rest) {
        parameter_types.push(field_type);
        rest = new_rest;
    }
    let (rest, return_type) = parse_return_type(rest)?;
    Ok((
        rest,
        MethodDescriptor {
            parameters: parameter_types,
            return_type,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method_descriptor() {
        let input = b"(IDLjava/lang/Thread;)Ljava/lang/Object;";
        let expected = MethodDescriptor {
            parameters: vec![
                FieldType::Int,
                FieldType::Double,
                FieldType::Object(b"java/lang/Thread"),
            ],
            return_type: FieldType::Object(b"java/lang/Object"),
        };
        let (rest, result) = parse_method_descriptor(input).unwrap();
        assert_eq!(rest, b"");
        assert_eq!(result, expected);
    }
}
