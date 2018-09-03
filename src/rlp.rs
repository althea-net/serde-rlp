use byteorder::{BigEndian, WriteBytesExt};
use num::Num;
use num::Unsigned;
use std::mem::size_of;

fn to_binary(x: u64) -> Vec<u8> {
    if x == 0 {
        Vec::new()
    } else {
        let mut result = to_binary(x / 256);
        result.push((x % 256) as u8);
        result
    }
}

#[test]
fn test_to_binary_null() {
    assert_eq!(to_binary(0u64), []);
}

#[test]
fn test_to_binary_non_null() {
    assert_eq!(to_binary(1024u64), [0x04, 0x00]);
    assert_eq!(
        to_binary(18446744073709551615u64),
        [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
}

pub fn encode_length(l: u64, offset: u8) -> Vec<u8> {
    if l < 56 {
        vec![l as u8 + offset]
    } else if l < u64::max_value() {
        let mut bl = to_binary(l);
        let magic = bl.len() as u8 + offset + 55;
        bl.insert(0, magic);
        bl
    } else {
        panic!("input too long");
    }
}

#[test]
fn test_encode_length_small() {
    assert_eq!(encode_length(55u64, 0xc0), [55 + 0xc0]);
}

#[test]
fn test_encode_length_big() {
    assert_eq!(
        encode_length(18446744073709551614u64, 0x80),
        [191, 255, 255, 255, 255, 255, 255, 255, 254]
    );
}

#[test]
#[should_panic]
fn test_encode_length_of_wrong_size() {
    encode_length(18446744073709551615u64, 0x80);
}

pub fn encode_number<T: Num + Unsigned>(v: T) -> Vec<u8>
where
    T: Into<u64>,
{
    let mut wtr = vec![];
    wtr.write_uint::<BigEndian>(v.into(), size_of::<T>())
        .unwrap();
    let index = wtr.iter().position(|&r| r > 0u8).unwrap_or(0);
    wtr.split_off(index)
}

#[test]
fn test_encode_number() {
    assert_eq!(encode_number(255u8), [0xff]);
    assert_eq!(encode_number(1024u16), [0x04, 0x00]);
    assert_eq!(encode_number(1024u32), [0x04, 0x00]);
    assert_eq!(encode_number(1024u64), [0x04, 0x00]);
}
