// This file is part of farcloud-labs/subsmt.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use sha3::{Digest, Keccak256};
use sparse_merkle_tree::{traits::Hasher, H256};

pub struct Keccak256Hasher(Keccak256);

impl Default for Keccak256Hasher {
    fn default() -> Self {
        Keccak256Hasher(Keccak256::new())
    }
}

impl Hasher for Keccak256Hasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_ref());
    }
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..])
    }

    fn finish(self) -> H256 {
        let a: [u8; 32] = self.0.finalize().into();
        a.into()
    }
}

// 需要测一遍看看是否对得上

#[cfg(test)]
pub mod test {
    use super::*;
    use ethers::utils::keccak256;
    #[test]
    fn test_hasher() {
        let mut hasher = Keccak256Hasher::default();
        hasher.write_h256(&H256::default());
        let h1 = hasher.finish();

        let h11: H256 = keccak256(&H256::default()).into();
        assert_eq!(h1, h11);

        let b: H256 = [1u8; 32].to_vec().into();
        let mut hasher1 = Keccak256Hasher::default();
        hasher1.write_h256(&b);
        let h2 = hasher1.finish();
        let h22: H256 = keccak256(&b).into();
        assert_eq!(h2, h22);

        let mut hasher2 = Keccak256Hasher::default();
        hasher2.write_byte(25u8);
        let h2: H256 = hasher2.finish();
        assert_eq!(h2, keccak256(25u8.to_be_bytes()).into());
    }
}
