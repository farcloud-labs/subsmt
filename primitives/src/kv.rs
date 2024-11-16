use codec::{Decode, Encode};
// use ethers::utils::keccak256;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
// use sp_core::Hasher::keccak256;
use scale_info::prelude::{string::String, vec::Vec};
use sha3::Digest;
// use sp_crypto_hashing::keccak_256;
use sparse_merkle_tree::{traits::Value, H256};
use sha3::Keccak256;
use scale_info::prelude::fmt::Debug;

cfg_if::cfg_if! {
    if #[cfg(feature="std")] {
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
            // #[serde_as(as = "DisplayFromStr")]
            pub nonce: u64,
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
        pub struct SMTKey {
            pub address: String,
        }
    } else {
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
            // #[serde_as(as = "DisplayFromStr")]
            pub nonce: u64,
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
            TypeInfo,
        )]
        pub struct SMTKey {
            pub address: String,
        }

    }

}

impl Value for SMTKey {
    fn zero() -> Self {
        SMTKey::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        let mut k = Keccak256::new();
        k.update(&self.encode().as_slice());
        let r: [u8; 32] = k.finalize().into();
        r.into()
    }
}

impl Value for SMTValue {
    fn zero() -> Self {
        Default::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        if self == &Default::default() {
            return H256::zero();
        }
        let mut k = Keccak256::new();
        k.update(&self.encode().as_slice());
        let r: [u8; 32] = k.finalize().into();
        r.into()
    }
}

// impl Into<Vec<u8>> for SMTValue {
//     fn into(self) -> Vec<u8> {
//         self.encode()
//     }
// }

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
    use sparse_merkle_tree::traits::Value;
    use sparse_merkle_tree::H256;

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
