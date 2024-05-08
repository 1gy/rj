use super::error::ParseError;

pub fn bytes(input: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if input.len() < length {
        return Err(ParseError::Eof);
    }
    let (value, rest) = input.split_at(length);
    Ok((rest, value))
}

pub fn be_u8(input: &[u8]) -> Result<(&[u8], u8), ParseError> {
    if input.is_empty() {
        return Err(ParseError::Eof);
    }
    let value = input[0];
    let rest = &input[1..];
    Ok((rest, value))
}

pub fn be_u16(input: &[u8]) -> Result<(&[u8], u16), ParseError> {
    if input.len() < 2 {
        return Err(ParseError::Eof);
    }
    let value = u16::from_be_bytes([input[0], input[1]]);
    let rest = &input[2..];
    Ok((rest, value))
}

pub fn be_u32(input: &[u8]) -> Result<(&[u8], u32), ParseError> {
    if input.len() < 4 {
        return Err(ParseError::Eof);
    }
    let value = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    let rest = &input[4..];
    Ok((rest, value))
}

pub fn be_u64(input: &[u8]) -> Result<(&[u8], u64), ParseError> {
    if input.len() < 8 {
        return Err(ParseError::Eof);
    }
    let value = u64::from_be_bytes([
        input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
    ]);
    let rest = &input[8..];
    Ok((rest, value))
}

pub fn be_i8(input: &[u8]) -> Result<(&[u8], i8), ParseError> {
    if input.is_empty() {
        return Err(ParseError::Eof);
    }
    let value = input[0] as i8;
    let rest = &input[1..];
    Ok((rest, value))
}

pub fn be_i16(input: &[u8]) -> Result<(&[u8], i16), ParseError> {
    if input.len() < 2 {
        return Err(ParseError::Eof);
    }
    let value = i16::from_be_bytes([input[0], input[1]]);
    let rest = &input[2..];
    Ok((rest, value))
}

pub fn be_i32(input: &[u8]) -> Result<(&[u8], i32), ParseError> {
    if input.len() < 4 {
        return Err(ParseError::Eof);
    }
    let value = i32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    let rest = &input[4..];
    Ok((rest, value))
}

pub fn be_i64(input: &[u8]) -> Result<(&[u8], i64), ParseError> {
    if input.len() < 8 {
        return Err(ParseError::Eof);
    }
    let value = i64::from_be_bytes([
        input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
    ]);
    let rest = &input[8..];
    Ok((rest, value))
}

pub fn be_f32(input: &[u8]) -> Result<(&[u8], f32), ParseError> {
    if input.len() < 4 {
        return Err(ParseError::Eof);
    }
    let value = f32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    let rest = &input[4..];
    Ok((rest, value))
}

pub fn be_f64(input: &[u8]) -> Result<(&[u8], f64), ParseError> {
    if input.len() < 8 {
        return Err(ParseError::Eof);
    }
    let value = f64::from_be_bytes([
        input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
    ]);
    let rest = &input[8..];
    Ok((rest, value))
}

pub fn take_until<'a>(input: &'a [u8], bytes: &[u8]) -> Result<(&'a [u8], &'a [u8]), ParseError> {
    let position = input
        .windows(bytes.len())
        .position(|window| window == bytes)
        .ok_or(ParseError::Eof)?;
    let (value, rest) = input.split_at(position);
    let rest = &rest[bytes.len()..];
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
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_u8() {
        let input = [1, 2, 3, 4, 5];
        let (rest, value) = be_u8(&input).unwrap();
        assert_eq!(rest, [2, 3, 4, 5]);
        assert_eq!(value, 1);

        let result = be_u8(&[]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_u16() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, value) = be_u16(&input).unwrap();
        assert_eq!(rest, [0x56, 0x78]);
        assert_eq!(value, 0x1234);

        let result = be_u16(&[0x12]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_u32() {
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        let (rest, value) = be_u32(&input).unwrap();
        assert_eq!(rest, [0x9a, 0xbc]);
        assert_eq!(value, 0x12345678);

        let result = be_u32(&[0x12, 0x34, 0x56]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_u64() {
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22];
        let (rest, value) = be_u64(&input).unwrap();
        assert_eq!(rest, [0x11, 0x22]);
        assert_eq!(value, 0x123456789abcdef0);

        let result = be_u64(&[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_i8() {
        // positive
        let input = [0x12, 0x34];
        let (rest, value) = be_i8(&input).unwrap();
        assert_eq!(rest, [0x34]);
        assert_eq!(value, 0x12);

        // negative
        let input = [0xff, 0x12];
        let (rest, value) = be_i8(&input).unwrap();
        assert_eq!(rest, [0x12]);
        assert_eq!(value, -1);

        let result = be_i8(&[]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_i16() {
        // positive
        let input = [0x12, 0x34, 0x56, 0x78];
        let (rest, value) = be_i16(&input).unwrap();
        assert_eq!(rest, [0x56, 0x78]);
        assert_eq!(value, 0x1234);

        // negative
        let input = [0xff, 0xff, 0xff, 0xff];
        let (rest, value) = be_i16(&input).unwrap();
        assert_eq!(rest, [0xff, 0xff]);
        assert_eq!(value, -1);

        let result = be_i16(&[0x12]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_i32() {
        // positive
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        let (rest, value) = be_i32(&input).unwrap();
        assert_eq!(rest, [0x9a, 0xbc]);
        assert_eq!(value, 0x12345678);

        // negative
        let input = [0xff, 0xff, 0xff, 0xff, 0x12, 0x34];
        let (rest, value) = be_i32(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, -1);

        let result = be_i32(&[0x12, 0x34, 0x56]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_i64() {
        // positive
        let input = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22];
        let (rest, value) = be_i64(&input).unwrap();
        assert_eq!(rest, [0x11, 0x22]);
        assert_eq!(value, 0x123456789abcdef0);

        // negative
        let input = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x12, 0x34];
        let (rest, value) = be_i64(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, -1);

        let result = be_i64(&[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_f32() {
        // positive
        let input = [0x3f, 0x9d, 0xf3, 0xb6, 0x12, 0x34];
        let (rest, value) = be_f32(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, 1.234);

        // negative
        let input = [0xbf, 0x9d, 0xf3, 0xb6, 0x12, 0x34];
        let (rest, value) = be_f32(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, -1.234);

        let result = be_f32(&[0x40, 0x49, 0x0f]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_be_f64() {
        // positive
        let input = [0x3f, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88, 0x87, 0x12, 0x34];
        let (rest, value) = be_f64(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, 1.234_567);

        // negative
        let input = [0xbf, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88, 0x87, 0x12, 0x34];
        let (rest, value) = be_f64(&input).unwrap();
        assert_eq!(rest, [0x12, 0x34]);
        assert_eq!(value, -1.234_567);

        let result = be_f64(&[0xbf, 0xf3, 0xc0, 0xc9, 0x53, 0x9b, 0x88]);
        assert_eq!(result, Err(ParseError::Eof));
    }

    #[test]
    fn test_take_until() {
        let input = [1, 2, 3, 4, 5];
        let (rest, value) = take_until(&input, &[3, 4]).unwrap();
        assert_eq!(rest, [5]);
        assert_eq!(value, [1, 2]);

        let (rest, value) = take_until(&input, &[1, 2]).unwrap();
        assert_eq!(rest, [3, 4, 5]);
        assert_eq!(value, []);

        let result = take_until(&input, &[6, 7]);
        assert_eq!(result, Err(ParseError::Eof));
    }
}
