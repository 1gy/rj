use super::super::error::ClassParseError;
use crate::parser::be_u16;

#[derive(Debug, PartialEq)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

pub fn parse_line_number_table_entry(
    input: &[u8],
) -> Result<(&[u8], LineNumberTableEntry), ClassParseError> {
    let (input, start_pc) = be_u16(input)?;
    let (input, line_number) = be_u16(input)?;
    Ok((
        input,
        LineNumberTableEntry {
            start_pc,
            line_number,
        },
    ))
}

#[derive(Debug, PartialEq)]
pub struct LineNumberTable {
    line_number_table: Vec<LineNumberTableEntry>,
}

pub fn parse_line_number_table<A>(input: &[u8]) -> Result<(&[u8], A), ClassParseError>
where
    A: From<LineNumberTable>,
{
    let (input, line_number_table_length) = be_u16(input)?;
    let mut line_number_table = Vec::new();
    let mut input = input;
    for _ in 0..line_number_table_length {
        let (new_input, entry) = parse_line_number_table_entry(input)?;
        input = new_input;
        line_number_table.push(entry);
    }
    let attribute = LineNumberTable { line_number_table };
    Ok((input, attribute.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_number_table_entry() {
        let input = [0x00, 0x01, 0x00, 0x02];
        let expected = LineNumberTableEntry {
            start_pc: 1,
            line_number: 2,
        };
        let (input, result) = parse_line_number_table_entry(&input).unwrap();
        assert_eq!(input, &[]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line_number_table() {
        let input = [
            0x00, 0x02, // line_number_table_length
            0x00, 0x12, 0x00, 0x34, // line_number_table[0]
            0x00, 0x56, 0x00, 0x78, // line_number_table[1]
            0x99, 0x99, // rest
        ];
        let expected = LineNumberTable {
            line_number_table: vec![
                LineNumberTableEntry {
                    start_pc: 0x12,
                    line_number: 0x34,
                },
                LineNumberTableEntry {
                    start_pc: 0x56,
                    line_number: 0x78,
                },
            ],
        };
        let (input, result) = parse_line_number_table::<LineNumberTable>(&input).unwrap();
        assert_eq!(input, &[0x99, 0x99]);
        assert_eq!(result, expected);
    }
}
