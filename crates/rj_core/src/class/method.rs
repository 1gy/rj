use crate::parser::be_u16;

use super::access_flags::MethodAccessFlags;
use super::attribute::{parse_attribute, Attribute};
use super::constant::Constant;
use super::error::ClassParseError;

#[derive(Debug, PartialEq)]
pub struct Method<'a> {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute<'a>>,
}

pub fn parse_method<'a>(
    input: &'a [u8],
    constant_pool: &[Constant],
) -> Result<(&'a [u8], Method<'a>), ClassParseError> {
    let (input, access_flags) = be_u16(input)?;
    let (input, name_index) = be_u16(input)?;
    let (input, descriptor_index) = be_u16(input)?;
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

    Ok((
        input,
        Method {
            access_flags: MethodAccessFlags::from_bits(access_flags),
            name_index,
            descriptor_index,
            attributes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() {
        let data = [
            0x00, 0x09, // access_flags
            0x00, 0x01, // name_index
            0x00, 0x02, // descriptor_index
            0x00, 0x01, // attributes_count
            0x00, 0x03, // attribute_name_index
            0x00, 0x00, 0x00, 0x04, // attribute_length
            0x00, 0x01, 0x02, 0x03, // data
            0x12, 0x34, // rest
        ];
        let constant_pool = vec![
            Constant::Utf8 { value: b"name" },
            Constant::Utf8 {
                value: b"descriptor",
            },
            Constant::Utf8 {
                value: b"Unknown_Attribute_Name",
            },
        ];
        let (rest, method) = parse_method(&data, &constant_pool).unwrap();
        assert_eq!(rest, &[0x12, 0x34]);
        assert_eq!(
            method,
            Method {
                access_flags: MethodAccessFlags::from_bits(0x0009),
                name_index: 1,
                descriptor_index: 2,
                attributes: vec![Attribute::Unknown {
                    attribute_name_index: 0x0003,
                    data: &[0x00, 0x01, 0x02, 0x03]
                }]
            }
        );
    }
}
