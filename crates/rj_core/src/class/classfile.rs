use crate::parser::{be_u16, be_u32};

use super::access_flags::ClassAccessFlags;
use super::attribute::{parse_attribute, Attribute};
use super::constant::{parse_constant, Constant};
use super::error::ClassParseError;
use super::field::{parse_field, Field};
use super::method::{parse_method, Method};

#[derive(Debug, PartialEq)]
pub struct ClassFile<'a> {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant<'a>>,
    access_flags: ClassAccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<Field<'a>>,
    methods: Vec<Method<'a>>,
    attributes: Vec<Attribute<'a>>,
}

pub fn parse_classfile(input: &[u8]) -> Result<(&[u8], ClassFile), ClassParseError> {
    let (input, magic) = be_u32(input)?;
    let (input, minor_version) = be_u16(input)?;
    let (input, major_version) = be_u16(input)?;
    let (input, constant_pool) = {
        let (input, constant_pool_count) = be_u16(input)?;
        let mut constant_pool = Vec::new();
        let mut input = input;
        for _ in 1..constant_pool_count {
            let (new_input, constant) = parse_constant(input)?;
            input = new_input;
            constant_pool.push(constant);
        }
        (input, constant_pool)
    };
    let (input, access_flags) = be_u16(input)?;
    let (input, this_class) = be_u16(input)?;
    let (input, super_class) = be_u16(input)?;
    let (input, interfaces) = {
        let (input, interfaces_count) = be_u16(input)?;
        let mut interfaces = Vec::new();
        let mut input = input;
        for _ in 0..interfaces_count {
            let (new_input, interface) = be_u16(input)?;
            input = new_input;
            interfaces.push(interface);
        }
        (input, interfaces)
    };
    let (input, fields) = {
        let (input, fields_count) = be_u16(input)?;
        let mut fields = Vec::new();
        let mut input = input;
        for _ in 0..fields_count {
            let (new_input, field) = parse_field(input, &constant_pool)?;
            input = new_input;
            fields.push(field);
        }
        (input, fields)
    };
    let (input, methods) = {
        let (input, methods_count) = be_u16(input)?;
        let mut methods = Vec::new();
        let mut input = input;
        for _ in 0..methods_count {
            let (new_input, method) = parse_method(input, &constant_pool)?;
            input = new_input;
            methods.push(method);
        }
        (input, methods)
    };
    let (input, attributes) = {
        let (input, attributes_count) = be_u16(input)?;
        let mut attributes = Vec::new();
        let mut input = input;
        for _ in 0..attributes_count {
            let (new_input, attribute) = parse_attribute(input, &constant_pool)?;
            input = new_input;
            attributes.push(attribute);
        }
        (input, attributes)
    };

    Ok((
        input,
        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags: ClassAccessFlags::from_bits(access_flags),
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_classfile() {
        let data = include_bytes!("../../../../java/HelloWorld.class");
        let (_, classfile) = parse_classfile(data).unwrap();
        assert_eq!(classfile.magic, 0xCAFEBABE);
        // TODO: Add more assertions
    }
}
