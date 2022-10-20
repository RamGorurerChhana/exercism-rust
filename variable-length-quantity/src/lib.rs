#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = vec![];
    for val in values {
        let bits = format!("{:b}", val);
        let bits = bits.chars().into_iter().rev().collect::<Vec<_>>();
        let bits = bits.chunks(7).collect::<Vec<_>>();
        let mut bits = bits
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let mut v = chunk.to_vec();
                while v.len() < 7 {
                    v.push('0');
                }
                if i == 0 {
                    v.push('0');
                } else {
                    v.push('1');
                }
                v.reverse();
                let v = v.iter().collect::<String>();
                u8::from_str_radix(&v, 2).unwrap()
            })
            .collect::<Vec<_>>();
        while let Some(val) = bits.pop() {
            bytes.push(val);
        }
    }

    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = vec![];
    let mut num_str = String::new();
    for byte in bytes {
        let byte = format!("{:<08b}", byte);
        let msb = byte.chars().nth(0).unwrap();
        num_str.push_str(&byte[1..]);
        if msb == '0' {
            let val = u32::from_str_radix(num_str.as_str(), 2).map_err(|_| Error::Overflow)?;
            result.push(val);
            num_str.clear();
        }
    }
    if num_str.is_empty() {
        Ok(result)
    } else {
        Err(Error::IncompleteNumber)
    }
}
