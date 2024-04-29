use super::super::error::ClassParseError;
use crate::parser::be_u16;

#[derive(Debug, PartialEq)]
pub struct SourceFile {
    sourcefile_index: u16,
}

pub fn parse_source_file<A>(input: &[u8]) -> Result<(&[u8], A), ClassParseError>
where
    A: From<SourceFile>,
{
    let (input, sourcefile_index) = be_u16(input)?;
    let attribute = SourceFile { sourcefile_index };
    Ok((input, attribute.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_source_file() {
        let input = [0x00, 0x01];
        let expected = SourceFile {
            sourcefile_index: 1,
        };
        let (input, result) = parse_source_file::<SourceFile>(&input).unwrap();
        assert_eq!(input, &[]);
        assert_eq!(result, expected);
    }
}
