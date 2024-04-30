#[derive(Debug, PartialEq)]
pub enum PrintError {
    Utf8Error(core::str::Utf8Error),
    InvalidConstant,
}

impl From<core::str::Utf8Error> for PrintError {
    fn from(e: core::str::Utf8Error) -> Self {
        PrintError::Utf8Error(e)
    }
}
