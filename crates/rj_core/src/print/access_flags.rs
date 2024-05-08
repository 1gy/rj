use crate::class::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

impl ClassAccessFlags {
    pub fn print(&self) -> String {
        let mut flags = vec![];
        if self.contains(ClassAccessFlags::PUBLIC) {
            flags.push("ACC_PUBLIC");
        }
        if self.contains(ClassAccessFlags::FINAL) {
            flags.push("ACC_FINAL");
        }
        if self.contains(ClassAccessFlags::SUPER) {
            flags.push("ACC_SUPER");
        }
        if self.contains(ClassAccessFlags::INTERFACE) {
            flags.push("ACC_INTERFACE");
        }
        if self.contains(ClassAccessFlags::ABSTRACT) {
            flags.push("ACC_ABSTRACT");
        }
        if self.contains(ClassAccessFlags::SYNTHETIC) {
            flags.push("ACC_SYNTHETIC");
        }
        if self.contains(ClassAccessFlags::ANNOTATION) {
            flags.push("ACC_ANNOTATION");
        }
        if self.contains(ClassAccessFlags::ENUM) {
            flags.push("ACC_ENUM");
        }
        if self.contains(ClassAccessFlags::MODULE) {
            flags.push("ACC_MODULE");
        }
        format!("flags: (0x{:04X}) {}", self.bits(), flags.join(", "))
    }

    pub fn print_program(&self) -> String {
        let mut flags = vec![];
        if self.contains(ClassAccessFlags::PUBLIC) {
            flags.push("public");
        }
        if self.contains(ClassAccessFlags::FINAL) {
            flags.push("final");
        }
        if self.contains(ClassAccessFlags::ABSTRACT) {
            flags.push("abstract");
        }
        {
            if self.contains(ClassAccessFlags::INTERFACE) {
                flags.push("interface");
            } else if self.contains(ClassAccessFlags::ENUM) {
                flags.push("enum");
            } else if self.contains(ClassAccessFlags::MODULE) {
                flags.push("module");
            } else {
                flags.push("class");
            }
        }
        flags.join(" ")
    }
}

impl FieldAccessFlags {
    pub fn print(&self) -> String {
        let mut flags = vec![];
        if self.contains(FieldAccessFlags::PUBLIC) {
            flags.push("ACC_PUBLIC");
        }
        if self.contains(FieldAccessFlags::PRIVATE) {
            flags.push("ACC_PRIVATE");
        }
        if self.contains(FieldAccessFlags::PROTECTED) {
            flags.push("ACC_PROTECTED");
        }
        if self.contains(FieldAccessFlags::STATIC) {
            flags.push("ACC_STATIC");
        }
        if self.contains(FieldAccessFlags::FINAL) {
            flags.push("ACC_FINAL");
        }
        if self.contains(FieldAccessFlags::VOLATILE) {
            flags.push("ACC_VOLATILE");
        }
        if self.contains(FieldAccessFlags::TRANSIENT) {
            flags.push("ACC_TRANSIENT");
        }
        if self.contains(FieldAccessFlags::SYNTHETIC) {
            flags.push("ACC_SYNTHETIC");
        }
        if self.contains(FieldAccessFlags::ENUM) {
            flags.push("ACC_ENUM");
        }
        format!("flags: (0x{:04X}) {}", self.bits(), flags.join(", "))
    }

    pub fn print_program(&self) -> String {
        let mut flags = vec![];
        if self.contains(FieldAccessFlags::PUBLIC) {
            flags.push("public");
        }
        if self.contains(FieldAccessFlags::PRIVATE) {
            flags.push("private");
        }
        if self.contains(FieldAccessFlags::PROTECTED) {
            flags.push("protected");
        }
        if self.contains(FieldAccessFlags::STATIC) {
            flags.push("static");
        }
        if self.contains(FieldAccessFlags::FINAL) {
            flags.push("final");
        }
        if self.contains(FieldAccessFlags::VOLATILE) {
            flags.push("volatile");
        }
        if self.contains(FieldAccessFlags::TRANSIENT) {
            flags.push("transient");
        }
        if self.contains(FieldAccessFlags::ENUM) {
            flags.push("enum");
        }
        flags.join(" ")
    }
}

impl MethodAccessFlags {
    pub fn print(&self) -> String {
        let mut flags = vec![];
        if self.contains(MethodAccessFlags::PUBLIC) {
            flags.push("ACC_PUBLIC");
        }
        if self.contains(MethodAccessFlags::PRIVATE) {
            flags.push("ACC_PRIVATE");
        }
        if self.contains(MethodAccessFlags::PROTECTED) {
            flags.push("ACC_PROTECTED");
        }
        if self.contains(MethodAccessFlags::STATIC) {
            flags.push("ACC_STATIC");
        }
        if self.contains(MethodAccessFlags::FINAL) {
            flags.push("ACC_FINAL");
        }
        if self.contains(MethodAccessFlags::SYNCHRONIZED) {
            flags.push("ACC_SYNCHRONIZED");
        }
        if self.contains(MethodAccessFlags::BRIDGE) {
            flags.push("ACC_BRIDGE");
        }
        if self.contains(MethodAccessFlags::VARARGS) {
            flags.push("ACC_VARARGS");
        }
        if self.contains(MethodAccessFlags::NATIVE) {
            flags.push("ACC_NATIVE");
        }
        if self.contains(MethodAccessFlags::ABSTRACT) {
            flags.push("ACC_ABSTRACT");
        }
        if self.contains(MethodAccessFlags::STRICT) {
            flags.push("ACC_STRICT");
        }
        if self.contains(MethodAccessFlags::SYNTHETIC) {
            flags.push("ACC_SYNTHETIC");
        }
        format!("flags: (0x{:04X}) {}", self.bits(), flags.join(", "))
    }

    pub fn print_program(&self) -> String {
        let mut flags = vec![];
        if self.contains(MethodAccessFlags::PUBLIC) {
            flags.push("public");
        }
        if self.contains(MethodAccessFlags::PRIVATE) {
            flags.push("private");
        }
        if self.contains(MethodAccessFlags::PROTECTED) {
            flags.push("protected");
        }
        if self.contains(MethodAccessFlags::STATIC) {
            flags.push("static");
        }
        if self.contains(MethodAccessFlags::FINAL) {
            flags.push("final");
        }
        if self.contains(MethodAccessFlags::SYNCHRONIZED) {
            flags.push("synchronized");
        }
        if self.contains(MethodAccessFlags::NATIVE) {
            flags.push("native");
        }
        if self.contains(MethodAccessFlags::ABSTRACT) {
            flags.push("abstract");
        }
        if self.contains(MethodAccessFlags::STRICT) {
            flags.push("strictfp");
        }
        flags.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_access_flags() {
        let flags = ClassAccessFlags::from_bits(0x0001);
        assert_eq!(flags.print(), "flags: (0x0001) ACC_PUBLIC");
        assert_eq!(flags.print_program(), "public class");
    }

    #[test]
    fn test_field_access_flags() {
        let flags = FieldAccessFlags::from_bits(0x0001);
        assert_eq!(flags.print(), "flags: (0x0001) ACC_PUBLIC");
        assert_eq!(flags.print_program(), "public");
    }

    #[test]
    fn test_method_access_flags() {
        let flags = MethodAccessFlags::from_bits(0x0001);
        assert_eq!(flags.print(), "flags: (0x0001) ACC_PUBLIC");
        assert_eq!(flags.print_program(), "public");
    }
}
