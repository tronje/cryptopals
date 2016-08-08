extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64, FromBase64};
use serialize::hex::{ToHex, FromHex};

pub fn hex_to_base64(hex_str: &str) -> String {
    //! Convert a string slice of hex values to a base64 String.
    //!
    //! # Examples
    //!
    //! ```
    //! let hex_str = "deadbeef";
    //! let base64_string = hex_to_base64(&hex_str);
    //!
    //! assert_eq!(base64_string, "3q2+7w==".to_string());
    //! ```

    hex_str.from_hex().unwrap().as_slice().to_base64(base64::STANDARD)
}

pub fn base64_to_hex(base64_str: &str) -> String {
    base64_str.from_base64().unwrap().as_slice().to_hex()
}

pub fn fixed_xor(left: &str, right: &str) -> Vec<u8> {
    assert_eq!(left.len(), right.len());

    let left_vec: Vec<u8> = left.from_hex().unwrap();
    let right_vec: Vec<u8> = right.from_hex().unwrap();
    let mut ret = vec![];

    for i in 0..left_vec.len() {
        ret.push(left_vec[i] ^ right_vec[i]);
    }

    ret
}

fn main() {
    let res = fixed_xor("1c0111001f010100061a024b53535009181c",
                        "686974207468652062756c6c277320657965");
        //.expect("Found invalid UTF-8");
    //let res_str = String::from_utf8(res).unwrap();
    //let res_slice: &str = &res_str[..];
    println!("{}", res.as_slice().to_hex());
    //println!("{}", res.len());
    //println!("{}", base64_to_hex(res_slice));
    //println!("{}", res);
    println!("{}", hex_to_base64("746865206b696420646f6e277420706c6179"));
}

#[test]
fn test_htob64() {
    assert_eq!(hex_to_base64("deadbeef"), "3q2+7w==".to_string());
}
