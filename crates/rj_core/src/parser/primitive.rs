use super::error::ParserError;

pub fn bytes(input: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParserError> {
    if input.len() < length {
        return Err(ParserError::Eof);
    }
    let (value, rest) = input.split_at(length);
    Ok((rest, value))
}

pub fn be_u8(input: &[u8]) -> Result<(&[u8], u8), ParserError> {
    if input.is_empty() {
        return Err(ParserError::Eof);
    }
    let value = input[0];
    let rest = &input[1..];
    Ok((rest, value))
}

pub fn be_u16(input: &[u8]) -> Result<(&[u8], u16), ParserError> {
    if input.len() < 2 {
        return Err(ParserError::Eof);
    }
    let value = u16::from_be_bytes([input[0], input[1]]);
    let rest = &input[2..];
    Ok((rest, value))
}

pub fn be_u32(input: &[u8]) -> Result<(&[u8], u32), ParserError> {
    if input.len() < 4 {
        return Err(ParserError::Eof);
    }
    let value = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    let rest = &input[4..];
    Ok((rest, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes() {
        let input = [1, 2, 3, 4, 5];
        let (rest, value) = bytes(&input, 3).unwrap();
        assert_eq!(rest, [4, 5]);
        assert_eq!(value, [1, 2, 3]);

        let (rest, value) = bytes(&input, 5).unwrap();
        assert_eq!(rest, []);
        assert_eq!(value, [1, 2, 3, 4, 5]);

        let result = bytes(&input, 6);
        assert_eq!(result, Err(ParserError::Eof));
    }

    #[test]
    fn test_be_u8() {
        let input = [1, 2, 3, 4, 5];
        let (rest, value) = be_u8(&input).unwrap();
        assert_eq!(rest, [2, 3, 4, 5]);
        assert_eq!(value, 1);

        let result = be_u8(&[]);
        assert_eq!(result, Err(ParserError::Eof));
    }

    #[test]
    fn test_be_u16() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, value) = be_u16(&input).unwrap();
        assert_eq!(rest, [0x56, 0x78]);
        assert_eq!(value, 0x1234);

        let result = be_u16(&[0x12]);
        assert_eq!(result, Err(ParserError::Eof));
    }

    #[test]
    fn test_be_u32() {
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        let (rest, value) = be_u32(&input).unwrap();
        assert_eq!(rest, [0x9a, 0xbc]);
        assert_eq!(value, 0x12345678);

        let result = be_u32(&[0x12, 0x34, 0x56]);
        assert_eq!(result, Err(ParserError::Eof));
    }
}
