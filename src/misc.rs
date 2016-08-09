//! Miscellaneous utility functions that don't do much by themselves.

extern crate rustc_serialize as serialize;

use self::serialize::base64::{self, ToBase64, FromBase64};
use self::serialize::hex::{ToHex, FromHex};

pub fn hex_to_base64(hex_str: &str) -> String {
    //! Convert a string slice of hex values to a base64 String.
    //!
    //! # Examples
    //!
    //! ```
    //! assert_eq!(hex_to_base64("deadbeef"), "3q2+7w==".to_string());
    //! ```

    hex_str.from_hex().unwrap().as_slice().to_base64(base64::STANDARD)
}

pub fn base64_to_hex(base64_str: &str) -> String {
    //! Convert a string slice of base64 values to a hex String.
    //!
    //! # Examples
    //!
    //! ```
    //! assert_eq!("deadbeef".to_string(), base64_to_hex("3q2+7w=="));
    //! ```

    base64_str.from_base64().unwrap().as_slice().to_hex()
}

pub fn fixed_xor(left: &str, right: &str) -> Vec<u8> {
    //! Given two string slices 'left' and 'right',
    //! xor them byte-wise and return a Vec<u8> of the results.

    assert_eq!(left.len(), right.len());

    let left_vec: Vec<u8> = left.from_hex().unwrap();
    let right_vec: Vec<u8> = right.from_hex().unwrap();
    let mut ret = vec![];

    for i in 0..left_vec.len() {
        ret.push(left_vec[i] ^ right_vec[i]);
    }

    ret
}

#[test]
fn test_htob64() {
    assert_eq!(hex_to_base64("deadbeef"), "3q2+7w==".to_string());
}

#[test]
fn test_b64toh() {
    assert_eq!(base64_to_hex("3q2+7w=="), "deadbeef".to_string());
}
