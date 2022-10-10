// Copyright 2018 Althea Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::error::Error;

/// Gives `F` the encoded length
pub fn encode_length<F: FnMut(&[u8]) -> Y, Y>(l: u64, offset: u8, mut f: F) -> Y {
    if l < 56 {
        let res: [u8; 1] = [l as u8 + offset];
        f(&res)
    } else if l < u64::max_value() {
        // this should be the max value of 8 if l == 0
        let lz_bytes = (l.leading_zeros() as usize) / 8;
        // room for 8 bytes plus a byte for the magic value
        let mut a = [0u8; 9];
        let magic = (8 - lz_bytes) as u8 + offset + 55;
        a[0] = magic;
        a[1..(9 - lz_bytes)].copy_from_slice(&l.to_be_bytes()[lz_bytes..]);
        f(&a[..(9 - lz_bytes)])
    } else {
        panic!("input too long");
    }
}

#[test]
fn test_encode_length_small() {
    encode_length(55u64, 0xc0, |b| assert_eq!(b, [55 + 0xc0]));
}

#[test]
fn test_encode_length_big() {
    encode_length(18446744073709551614u64, 0x80, |b| {
        assert_eq!(b, [191, 255, 255, 255, 255, 255, 255, 255, 254])
    });
}

#[test]
#[should_panic]
fn test_encode_length_of_wrong_size() {
    encode_length(18446744073709551615u64, 0x80, |_| {});
}

/// Gives `F` the encoded number
pub fn encode_number<T: Into<u64>, F: FnMut(&[u8]) -> Y, Y>(v: T, mut f: F) -> Y {
    let x: u64 = v.into();
    let mut lz_bytes = (x.leading_zeros() as usize) / 8;
    if x == 0 {
        // special case, there needs to be at least one byte
        lz_bytes -= 1;
    }
    let res: &[u8] = &x.to_be_bytes()[lz_bytes..];
    // avoid needing to allocate
    f(res)
}

#[test]
fn test_encode_number() {
    encode_number(0u8, |b| assert_eq!(b, [0]));
    encode_number(1u8, |b| assert_eq!(b, [1]));
    encode_number(255u8, |b| assert_eq!(b, [0xff]));
    encode_number(1024u16, |b| assert_eq!(b, [0x4, 0]));
    encode_number(1024u32, |b| assert_eq!(b, [0x4, 0]));
    encode_number(1024u64, |b| assert_eq!(b, [0x4, 0]));
}

fn to_integer(b: &[u8]) -> Option<u64> {
    if b.is_empty() {
        None
    } else {
        const LEN: usize = 8;
        let mut a = [0u8; LEN];
        a[(LEN - b.len())..].copy_from_slice(b);
        Some(u64::from_be_bytes(a))
    }
}

#[test]
fn to_integer_with_empty_buffer() {
    assert!(to_integer(&[]).is_none());
}

#[test]
fn to_integer_with_single_byte() {
    assert_eq!(to_integer(&[0xffu8]).unwrap(), 255u64);
}

#[test]
fn to_integer_with_multiple_bytes() {
    assert_eq!(to_integer(&[0x04u8, 0x00u8]).unwrap(), 1024u64);
}

#[test]
fn decode_u32_max() {
    assert_eq!(to_integer(&[0xffu8; 4]).unwrap(), 4294967295u64);
}

#[test]
fn decode_u64_max() {
    assert_eq!(to_integer(&[0xffu8; 8]).unwrap(), 18446744073709551615u64);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExpectedType {
    /// Expecting a string
    StringType,
    /// Expecting a list
    ListType,
}

#[derive(Debug)]
pub struct DecodeLengthResult {
    pub offset: usize,
    pub length: usize,
    pub expected_type: ExpectedType,
}

/// Decodes chunk of data and outputs offset, length of nested data and its expected type
pub fn decode_length(input: &[u8]) -> Result<DecodeLengthResult, Error> {
    if input.is_empty() {
        return Err(Error::EmptyBuffer);
    }
    let prefix = input[0];
    if prefix <= 0x7f {
        Ok(DecodeLengthResult {
            offset: 0,
            length: 1usize,
            expected_type: ExpectedType::StringType,
        })
    } else if prefix <= 0xb7 && input.len() > (prefix - 0x80) as usize {
        let str_len = prefix - 0x80;
        Ok(DecodeLengthResult {
            offset: 1,
            length: str_len as usize,
            expected_type: ExpectedType::StringType,
        })
    } else if prefix <= 0xbf
        && input.len() > prefix.checked_sub(0xb7).ok_or(Error::WrongPrefix)? as usize
        && input.len() as u64
            > prefix as u64 - 0xb7u64
                + to_integer(&input[1..prefix as usize - 0xb7 + 1])
                    .ok_or(Error::StringPrefixTooSmall)?
    {
        let len_of_str_len = prefix as usize - 0xb7;
        let str_len = to_integer(&input[1..len_of_str_len + 1]).unwrap();
        Ok(DecodeLengthResult {
            offset: 1 + len_of_str_len,
            length: str_len as usize,
            expected_type: ExpectedType::StringType,
        })
    } else if prefix <= 0xf7 && input.len() > prefix as usize - 0xc0 {
        let list_len = prefix as usize - 0xc0;
        Ok(DecodeLengthResult {
            offset: 1,
            length: list_len,
            expected_type: ExpectedType::ListType,
        })
    } else if
    /* prefix <= 0xff && */
    input.len() as u64 > prefix as u64 - 0xf7
        && input.len() as u64
            > prefix as u64 - 0xf7u64
                + to_integer(&input[1..prefix as usize - 0xf7 + 1])
                    .ok_or(Error::ListPrefixTooSmall)?
    {
        let len_of_list_len = prefix as usize - 0xf7;
        let list_len = to_integer(&input[1..len_of_list_len + 1]).unwrap();
        Ok(DecodeLengthResult {
            offset: 1 + len_of_list_len,
            length: list_len as usize,
            expected_type: ExpectedType::ListType,
        })
    } else {
        unreachable!();
    }
}

#[test]
fn decode_empty_byte_slice() {
    assert!(decode_length(&[]).is_err());
}

#[test]
fn decode_single_byte() {
    // "a"
    let res = decode_length(&[0x61u8]).unwrap();
    assert_eq!(res.offset, 0);
    assert_eq!(res.length, 1);
    assert_eq!(res.expected_type, ExpectedType::StringType);
}

#[test]
fn decode_short_string() {
    // "abc"
    let input = vec![0x83, 0x61, 0x62, 0x63, 0xff];
    let res = decode_length(&input[..]).unwrap();
    assert_eq!(res.offset, 1);
    assert_eq!(res.length, 3);
    assert_eq!(res.expected_type, ExpectedType::StringType);
}

#[test]
fn decode_short_array() {
    // 1024
    let res = decode_length(&[0xc4, 0x83, 0x61, 0x62, 0x63]).unwrap();
    assert_eq!(res.offset, 1);
    assert_eq!(res.length, 4);
    assert_eq!(res.expected_type, ExpectedType::ListType);
}
