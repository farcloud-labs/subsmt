// 字节和hex字符串之间的转转
// use hex::encode;
//
use super::*;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::{self, serde_as};
// use parity_scale_codec::alloc::vec::Vec;
// #[cfg(not(feature="derive"))]
// use parity_scale_codec_derive::{Encode, Decode};
// --features hex
use serde_with::hex::Hex;
// use serde_with

#[serde_as]
#[derive(Deserialize, Serialize, Debug, PartialEq, Encode, Decode)]
pub struct Student {
    #[serde_as(as = "Hex")]
    name: Vec<u8>,
    score: u16,
    id: u32,
}

#[test]
fn main() {
    let a = Student {
        name: b"hell".to_vec(),
        score: 42,
        id: 16777215,
    };

    let s = serde_json::to_string(&a).unwrap();
    println!("s: {:#?}", s);
    let ss: Student = serde_json::from_str(&s).unwrap();
    println!("ss: {:?}", ss);
    println!("a_json: {:#?}", serde_json::to_string(&a).unwrap());

    let a_e = a.encode();
    // fixme 为什么是16在前
    println!("a: {:?}", a_e);
    // let h_s = hex::encode(a_e);
    // hex::decode(data)
    println!("a_hex: {:?}", hex::encode(a_e));
    let a_n_e = a.name.encode();
    println!("{:?}", a_n_e);
    println!("name_hex: {:?}", hex::encode(a_n_e));
    let a_n_s = a.score.encode();
    println!("{:?}", a_n_s);
    println!("score_hex: {:?}", hex::encode(a_n_s));
    let a_n_id = a.id.encode();
    println!("{:?}", a_n_id);
    println!("id_hex: {:?}", hex::encode(a_n_id));
    // let h = hex::encode(a_n_id);
    // println!("{:?}", h);
}
