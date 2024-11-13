#![cfg_attr(not(feature = "std"), no_std)]

use sparse_merkle_tree::{traits::Hasher, H256};
use tiny_keccak::Hasher as KeccakHasher;
use tiny_keccak::Keccak;

pub struct Keccak256Hasher(Keccak);

impl Default for Keccak256Hasher {
    fn default() -> Self {
        Keccak256Hasher(Keccak::v256())
    }
}

impl Hasher for Keccak256Hasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    fn finish(self) -> H256 {
        let mut buf = [0u8; 32];
        self.0.finalize(&mut buf);
        buf.into()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use ethers::utils::keccak256;
    #[test]
    fn test_hasher() {
        let mut hasher = Keccak256Hasher::default();
        hasher.write_h256(&H256::default());
        let h = hasher.finish();

        let h1: H256 = keccak256(&H256::default()).into();
        assert_eq!(h, h1)


    }

}
