use std::borrow::Cow;

use crate::class::{
    parse_field_type, parse_method_descriptor, ClassFile, Constant, FieldType, MethodDescriptor,
};

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

fn get_utf8<'a>(index: u16, constant_pool: &'a [Constant<'a>]) -> Option<Cow<'a, str>> {
    if let Some(Constant::Utf8 { value }) = constant_pool.get(index as usize - 1) {
        return Some(Cow::Borrowed(core::str::from_utf8(value).unwrap()));
    }
    None
}

fn get_field_descriptor<'a>(
    index: u16,
    constant_pool: &'a [Constant<'a>],
) -> Option<FieldType<'a>> {
    if let Some(Constant::Utf8 { value }) = constant_pool.get(index as usize - 1) {
        let value = core::str::from_utf8(value).ok()?;
        let (_, field_type) = parse_field_type(value.as_bytes()).ok()?;
        return Some(field_type);
    }
    None
}

fn get_method_descriptor<'a>(
    index: u16,
    constant_pool: &'a [Constant<'a>],
) -> Option<MethodDescriptor<'a>> {
    if let Some(Constant::Utf8 { value }) = constant_pool.get(index as usize - 1) {
        let value = core::str::from_utf8(value).ok()?;
        let (_, method_descriptor) = parse_method_descriptor(value.as_bytes()).ok()?;
        return Some(method_descriptor);
    }
    None
}

impl<'a> FieldType<'a> {
    pub fn print(&self) -> String {
        match self {
            FieldType::Byte => "byte".to_string(),
            FieldType::Char => "char".to_string(),
            FieldType::Double => "double".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Int => "int".to_string(),
            FieldType::Long => "long".to_string(),
            FieldType::Short => "short".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Object(name) => core::str::from_utf8(name)
                .map(|s| s.replace('/', "."))
                .unwrap_or("".to_string())
                .to_string(),
            FieldType::Array(inner) => format!("{}[]", inner.print()),
            FieldType::Void => "void".to_string(),
        }
    }
}

impl<'a> MethodDescriptor<'a> {
    pub fn print_return(&self) -> String {
        self.return_type.print()
    }

    pub fn print_parameters(&self) -> String {
        self.parameters
            .iter()
            .map(|p| p.print())
            .collect::<Vec<_>>()
            .join(", ")
    }
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

        output.push_str("{\n");

        // fields
        {
            for field in &self.fields {
                let access_flags = field.access_flags.print_program();
                let name = get_utf8(field.name_index, &self.constant_pool)
                    .ok_or(PrintError::InvalidConstant)?;
                let descriptor = get_field_descriptor(field.descriptor_index, &self.constant_pool)
                    .ok_or(PrintError::InvalidConstant)?
                    .print();
                output.push_str(&format!("  {} {} {};\n", access_flags, descriptor, name));
            }
            output.push('\n');
        }

        // methods
        {
            for method in &self.methods {
                let access_flags = method.access_flags.print_program();
                let name = get_utf8(method.name_index, &self.constant_pool)
                    .ok_or(PrintError::InvalidConstant)?;
                let descriptor =
                    get_method_descriptor(method.descriptor_index, &self.constant_pool)
                        .ok_or(PrintError::InvalidConstant)?;
                output.push_str(&format!(
                    "  {} {} {}({});\n",
                    access_flags,
                    descriptor.print_return(),
                    name,
                    descriptor.print_parameters()
                ));
            }
        }

        output.push_str("}\n");

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
        let expected = r#"
public class HelloWorld
  minor version: 0
  major version: 65
  interfaces: 0, fields: 1, methods: 3, attributes: 1
Constant pool:
  #1 = Methodref          #2.#3          // java/lang/Object.<init>:()V
  #2 = Class              #4             // java/lang/Object
  #3 = NameAndType        #5:#6          // <init>:()V
  #4 = Utf8               java/lang/Object
  #5 = Utf8               <init>
  #6 = Utf8               ()V
  #7 = String             #8             // Hello, World!
  #8 = Utf8               Hello, World!
  #9 = Fieldref           #10.#11        // HelloWorld.message:Ljava/lang/String;
  #10 = Class              #12            // HelloWorld
  #11 = NameAndType        #13:#14        // message:Ljava/lang/String;
  #12 = Utf8               HelloWorld
  #13 = Utf8               message
  #14 = Utf8               Ljava/lang/String;
  #15 = Fieldref           #16.#17        // java/lang/System.out:Ljava/io/PrintStream;
  #16 = Class              #18            // java/lang/System
  #17 = NameAndType        #19:#20        // out:Ljava/io/PrintStream;
  #18 = Utf8               java/lang/System
  #19 = Utf8               out
  #20 = Utf8               Ljava/io/PrintStream;
  #21 = Methodref          #22.#23        // java/io/PrintStream.println:(Ljava/lang/String;)V
  #22 = Class              #24            // java/io/PrintStream
  #23 = NameAndType        #25:#26        // println:(Ljava/lang/String;)V
  #24 = Utf8               java/io/PrintStream
  #25 = Utf8               println
  #26 = Utf8               (Ljava/lang/String;)V
  #27 = Methodref          #10.#3         // HelloWorld.<init>:()V
  #28 = Methodref          #10.#29        // HelloWorld.sayHello:()V
  #29 = NameAndType        #30:#6         // sayHello:()V
  #30 = Utf8               sayHello
  #31 = Utf8               Code
  #32 = Utf8               LineNumberTable
  #33 = Utf8               main
  #34 = Utf8               ([Ljava/lang/String;)V
  #35 = Utf8               SourceFile
  #36 = Utf8               HelloWorld.java
{
  private java.lang.String message;

  public void <init>();
  private void sayHello();
  public static void main(java.lang.String[]);
}
"#;
        assert_eq!(output, expected[1..]);
    }
}
