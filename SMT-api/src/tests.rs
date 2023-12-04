use hex::encode;
use super::*;
use parity_scale_codec::Encode;

#[test]
fn main() {
    let a = Student {
        name: vec![0; 4],
        score: 64,
        id: 42,
    };

    let a_e = a.encode();
    // fixme 为什么是16在前
    println!("{:?}", a_e);
    let h_s = hex::encode(a_e);
    println!("{:?}", h_s);
    let a_n_e = a.name.encode();
    println!("{:?}", a_n_e);
    let a_n_s = a.score.encode();
    println!("{:?}", a_n_s);
    let a_n_id = a.id.encode();
    println!("{:?}", a_n_id);
    let h = hex::encode(a_n_id);
    println!("{:?}", h);

}