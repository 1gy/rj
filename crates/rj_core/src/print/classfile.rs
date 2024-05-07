use std::borrow::Cow;

use crate::class::{ClassFile, Constant};

use super::error::PrintError;

fn get_classname<'a>(
    index: u16,
    constant_pool: &'a [crate::class::Constant<'a>],
) -> Option<Cow<'a, str>> {
    // let class_info = constant_pool.get
    if let Some(Constant::Class { name_index }) = constant_pool.get(index as usize - 1) {
        if let Some(Constant::Utf8 { value }) = constant_pool.get(*name_index as usize - 1) {
            return Some(Cow::Borrowed(core::str::from_utf8(value).unwrap()));
        }
    }
    None
}

impl<'a> ClassFile<'a> {
    pub fn print(&self) -> Result<String, PrintError> {
        let mut output = String::new();

        let access_flags = self.access_flags.print_program();
        let classname = get_classname(self.this_class, &self.constant_pool)
            .ok_or(PrintError::InvalidConstant)?;
        output.push_str(&format!("{access_flags} {classname}\n"));

        output.push_str(&format!("  minor version: {}\n", self.minor_version));
        output.push_str(&format!("  major version: {}\n", self.major_version));
        output.push_str(&format!(
            "  interfaces: {}, fields: {}, methods: {}, attributes: {}\n",
            self.interfaces.len(),
            self.fields.len(),
            self.methods.len(),
            self.attributes.len()
        ));

        output.push_str("Constant pool:\n");
        for (i, constant) in self.constant_pool.iter().enumerate() {
            output.push_str(&format!(
                "  #{} = {}\n",
                i + 1,
                constant.print(&self.constant_pool)?
            ));
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::class::parse_classfile;

    // use super::*;

    #[test]
    fn test_print() {
        let data = include_bytes!("../../../../java/HelloWorld.class");
        let (_, classfile) = parse_classfile(data).unwrap();

        let output = classfile.print().unwrap();
        println!("{}", output);
    }
}
