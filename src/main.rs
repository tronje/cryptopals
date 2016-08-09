extern crate cryptopals;
extern crate rustc_serialize as serialize;

use cryptopals::misc::*;
use serialize::hex::ToHex;

fn main() {
    let res = fixed_xor("1c0111001f010100061a024b53535009181c",
                        "686974207468652062756c6c277320657965");
    println!("{}", res.as_slice().to_hex());
}
