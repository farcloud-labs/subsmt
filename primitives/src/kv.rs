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

//! You can think of this backend as a KVDB, but it also provides Merkle proofs for the existence or non-existence of data.  
//! Here, define the data structures for your key and value, as they determine how data is stored in the database.  
//! Your key will ultimately be hashed, and this hash will determine the path of your value in the Merkle tree.

#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
// use ethers::utils::keccak256;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
// use sp_core::Hasher::keccak256;
use scale_info::prelude::{string::String, vec::Vec};
use sha3::Digest;
// use sp_crypto_hashing::keccak_256;
use scale_info::prelude::fmt::Debug;
use sha3::Keccak256;
use sparse_merkle_tree::{traits::Value, H256};

cfg_if::cfg_if! {
    if #[cfg(feature="std")] {
        /// The data structure of the value in the KVDB, which determines the type of data you store in the Merkle tree.
        use utoipa::{IntoParams, ToSchema};
        #[serde_as]
        #[derive(
            Encode,
            Decode,
            Debug,
            Serialize,
            Deserialize,
            Default,
            PartialEq,
            Eq,
            Clone,
            ToSchema,
            IntoParams,
            TypeInfo,
        )]
        pub struct SMTValue {
            /// The nonce value of the user account.
            // #[serde_as(as = "DisplayFromStr")]
            pub nonce: u64,
            /// The balance of the user account.
            #[serde_as(as = "DisplayFromStr")]
            pub balance: u128,
        }

        #[serde_as]
        #[derive(
            Encode,
            Decode,
            Debug,
            Serialize,
            Deserialize,
            Default,
            PartialEq,
            Eq,
            Clone,
            ToSchema,
            IntoParams,
            TypeInfo,
        )]
        /// The key in the KVDB, which determines for whom you are storing data.
        pub struct SMTKey {
            /// The on-chain user address.
            pub address: String,
        }
    } else {
        /// The data structure of the value in the KVDB, which determines the type of data you store in the Merkle tree.
        #[serde_as]
        #[derive(
            Encode,
            Decode,
            Debug,
            Serialize,
            Deserialize,
            Default,
            PartialEq,
            Eq,
            Clone,
            // ToSchema,
            // IntoParams,
            TypeInfo,
        )]
        pub struct SMTValue {
            /// The nonce value of the user account.
            // #[serde_as(as = "DisplayFromStr")]
            pub nonce: u64,
             /// The balance of the user account.
            #[serde_as(as = "DisplayFromStr")]
            pub balance: u128,
        }

        /// The key in the KVDB, which determines for whom you are storing data.
        #[serde_as]
        #[derive(
            Encode,
            Decode,
            Debug,
            Serialize,
            Deserialize,
            Default,
            PartialEq,
            Eq,
            Clone,
            TypeInfo,
        )]
        pub struct SMTKey {
            /// The on-chain user address.
            pub address: String,
        }

    }

}

/// How the key in the KVDB is computed into a hash value.
impl Value for SMTKey {
    fn zero() -> Self {
        SMTKey::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        let mut k = Keccak256::new();
        k.update(self.encode().as_slice());
        let r: [u8; 32] = k.finalize().into();
        r.into()
    }
}

/// How the value in the KVDB is computed into a hash value.
impl Value for SMTValue {
    fn zero() -> Self {
        Default::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        if self == &Default::default() {
            return H256::zero();
        }
        let mut k = Keccak256::new();
        k.update(self.encode().as_slice());
        let r: [u8; 32] = k.finalize().into();
        r.into()
    }
}

impl From<SMTValue> for Vec<u8> {
    fn from(value: SMTValue) -> Self {
        value.encode()
    }
}

impl From<Vec<u8>> for SMTValue {
    fn from(value: Vec<u8>) -> Self {
        let a: SMTValue = Decode::decode::<&[u8]>(&mut value.as_slice()).unwrap_or_default();
        a
    }
}

#[cfg(test)]
mod test {
    use super::SMTValue;
    use sparse_merkle_tree::{traits::Value, H256};

    #[test]
    fn test_value() {
        let v = SMTValue {
            nonce: 1,
            balance: 100000,
        };
        let v_vec: Vec<u8> = v.clone().into();
        assert_eq!(v, v_vec.into());

        let v1: SMTValue = Default::default();
        assert_eq!(v1.to_h256(), H256::default());
    }
}
