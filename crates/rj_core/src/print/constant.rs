use std::borrow::Cow;

use crate::class::Constant;

use super::error::PrintError;

fn get_constant_name(constant: &Constant) -> &'static str {
    match constant {
        Constant::Utf8 { .. } => "Utf8",
        Constant::Integer { .. } => "Integer",
        Constant::Float { .. } => "Float",
        Constant::Long { .. } => "Long",
        Constant::Double { .. } => "Double",
        Constant::Class { .. } => "Class",
        Constant::String { .. } => "String",
        Constant::Fieldref { .. } => "Fieldref",
        Constant::Methodref { .. } => "Methodref",
        Constant::InterfaceMethodref { .. } => "InterfaceMethodref",
        Constant::NameAndType { .. } => "NameAndType",
        Constant::MethodHandle { .. } => "MethodHandle",
        Constant::MethodType { .. } => "MethodType",
        Constant::Dynamic { .. } => "Dynamic",
        Constant::InvokeDynamic { .. } => "InvokeDynamic",
        Constant::Module { .. } => "Module",
        Constant::Package { .. } => "Package",
    }
}

fn get_value<'a>(constant: &'a Constant) -> Result<Cow<'a, str>, PrintError> {
    match constant {
        Constant::Utf8 { value } => Ok(core::str::from_utf8(value)?.into()),
        Constant::Class { name_index } => Ok(format!("#{}", name_index).into()),
        Constant::String { string_index } => Ok(format!("#{}", string_index).into()),
        Constant::Fieldref {
            class_index,
            name_and_type_index,
        } => Ok(format!("#{}.#{}", class_index, name_and_type_index).into()),
        Constant::Methodref {
            class_index,
            name_and_type_index,
        } => Ok(format!("#{}.#{}", class_index, name_and_type_index).into()),
        Constant::NameAndType {
            name_index,
            descriptor_index,
        } => Ok(format!("#{}:#{}", name_index, descriptor_index).into()),
        _ => unimplemented!("constant: {:?}", constant),
    }
}

fn validate_utf8<'a>(
    constant_pool: &'a [Constant],
    index: u16,
) -> Result<Cow<'a, str>, PrintError> {
    let reference = &constant_pool[index as usize - 1];
    match reference {
        Constant::Utf8 { .. } => Ok(get_value(reference)?),
        _ => Err(PrintError::InvalidConstant),
    }
}

fn validate_class<'a>(
    constant_pool: &'a [Constant],
    index: u16,
) -> Result<Cow<'a, str>, PrintError> {
    let reference = &constant_pool[index as usize - 1];
    match reference {
        Constant::Class { .. } => Ok(get_comment(reference, constant_pool)?),
        _ => Err(PrintError::InvalidConstant),
    }
}

fn validate_name_and_type<'a>(
    constant_pool: &'a [Constant],
    index: u16,
) -> Result<Cow<'a, str>, PrintError> {
    let reference = &constant_pool[index as usize - 1];
    match reference {
        Constant::NameAndType { .. } => Ok(get_comment(reference, constant_pool)?),
        _ => Err(PrintError::InvalidConstant),
    }
}

fn get_comment<'a>(
    constant: &'a Constant,
    constant_pool: &[Constant],
) -> Result<Cow<'a, str>, PrintError> {
    match constant {
        Constant::Utf8 { .. } => Ok("".into()),
        Constant::Class { name_index } => {
            let value = validate_utf8(constant_pool, *name_index)?;
            Ok(format!("{value}").into())
        }
        Constant::String { string_index } => {
            let value = validate_utf8(constant_pool, *string_index)?;
            Ok(format!("{value}").into())
        }
        Constant::Fieldref {
            class_index,
            name_and_type_index,
        } => {
            let class = validate_class(constant_pool, *class_index)?;
            let name_and_type = validate_name_and_type(constant_pool, *name_and_type_index)?;
            Ok(format!("{class}.{}", name_and_type).into())
        }
        Constant::Methodref {
            class_index,
            name_and_type_index,
        } => {
            let class = validate_class(constant_pool, *class_index)?;
            let name_and_type = validate_name_and_type(constant_pool, *name_and_type_index)?;
            Ok(format!("{class}.{}", name_and_type).into())
        }
        Constant::NameAndType {
            name_index,
            descriptor_index,
        } => {
            let name = validate_utf8(constant_pool, *name_index)?;
            let descriptor = validate_utf8(constant_pool, *descriptor_index)?;
            Ok(format!("{name}:{descriptor}").into())
        }
        _ => unimplemented!("constant: {:?}", constant),
    }
}

impl<'a> Constant<'a> {
    pub fn print(&self, constant_pool: &[Constant]) -> Result<String, PrintError> {
        let name = get_constant_name(self);
        let value = get_value(self)?;
        let comment = get_comment(self, constant_pool)?;
        let comment = if comment.is_empty() {
            "".to_string()
        } else {
            format!("// {}", comment)
        };
        Ok(format!("{name:<19}{value:<15}{comment}")
            .trim_end()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8() {
        let constant_pool = [Constant::Utf8 {
            value: b"Hello, World!",
        }];
        let constant = &constant_pool[0];
        assert_eq!("Hello, World!", get_value(constant).unwrap());
        assert_eq!("", get_comment(constant, &constant_pool).unwrap());
    }

    #[test]
    fn test_class() {
        let constant_pool = [
            Constant::Class { name_index: 2 },
            Constant::Utf8 {
                value: b"java/lang/Object",
            },
        ];
        let constant = &constant_pool[0];
        assert_eq!("#2", get_value(constant).unwrap());
        assert_eq!(
            "java/lang/Object",
            get_comment(constant, &constant_pool).unwrap()
        );
    }

    #[test]
    fn test_string() {
        let constant_pool = [
            Constant::String { string_index: 2 },
            Constant::Utf8 {
                value: b"Hello, World!",
            },
        ];
        let constant = &constant_pool[0];
        assert_eq!("#2", get_value(constant).unwrap());
        assert_eq!(
            "Hello, World!",
            get_comment(constant, &constant_pool).unwrap()
        );
    }

    #[test]
    fn test_fieldref() {
        let constant_pool = [
            Constant::Fieldref {
                class_index: 2,
                name_and_type_index: 3,
            },
            Constant::Class { name_index: 4 },
            Constant::NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            Constant::Utf8 { value: b"Main" },
            Constant::Utf8 { value: b"field" },
            Constant::Utf8 {
                value: b"Ljava/lang/String;",
            },
        ];
        let constant = &constant_pool[0];
        assert_eq!("#2.#3", get_value(constant).unwrap());
        assert_eq!(
            "Main.field:Ljava/lang/String;",
            get_comment(constant, &constant_pool).unwrap()
        );
    }

    #[test]
    fn test_methodref() {
        let constant_pool = [
            Constant::Methodref {
                class_index: 2,
                name_and_type_index: 3,
            },
            Constant::Class { name_index: 4 },
            Constant::NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            Constant::Utf8 { value: b"Main" },
            Constant::Utf8 { value: b"method" },
            Constant::Utf8 { value: b"()V" },
        ];
        let constant = &constant_pool[0];
        assert_eq!("#2.#3", get_value(constant).unwrap());
        assert_eq!(
            "Main.method:()V",
            get_comment(constant, &constant_pool).unwrap()
        );
    }

    #[test]
    fn test_name_and_type() {
        let constant_pool = [
            Constant::Utf8 { value: b"toString" },
            Constant::Utf8 {
                value: b"()Ljava/lang/String;",
            },
            Constant::NameAndType {
                name_index: 1,
                descriptor_index: 2,
            },
        ];
        assert_eq!("#1:#2", get_value(&constant_pool[2]).unwrap());
        assert_eq!(
            "toString:()Ljava/lang/String;",
            get_comment(&constant_pool[2], &constant_pool).unwrap()
        );
    }
}
