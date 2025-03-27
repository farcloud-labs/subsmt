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

//! Verify the Merkle proof.  
//! This provides a more user-friendly method of verifying the Merkle proof, rather than just a sequence of `Vec<u8>` that cannot be directly parsed back into the original data.  
//! It enables you to perform more actions on-chain.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::legacy_numeric_constants)]
#![allow(clippy::arithmetic_side_effects)]

extern crate alloc;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::{prelude::fmt::Debug, TypeInfo};
use serde::{self, Deserialize, Serialize};
use sparse_merkle_tree::{
    merge::{hash_base_node, merge, MergeValue},
    traits::Hasher,
    H256,
};

cfg_if::cfg_if! {
    if #[cfg(feature="std")] {
        use utoipa::{ToSchema, IntoParams, __dev::ComposeSchema};

        /// Merkle proof.
        #[derive(Debug, Serialize, Deserialize, Clone, Encode, Decode, TypeInfo, PartialEq, ToSchema)]
        pub struct Proof<K: Debug + Clone + TypeInfo, V: Default + Debug + Clone + TypeInfo> {
            /// The key in the KVDB.
            #[serde(flatten)]
            pub key: K,
            /// The value in the KVDB.
            #[serde(flatten)]
            pub value: V,
            /// The Merkle leaf's path (i.e., the hash value of the key).
            pub path: H256,
            /// The hash value of the Merkle leaf (i.e., the hash value of the value).
            pub value_hash: H256,
            /// Merkle root hash.
            pub root: H256,
            /// Path marker, indicating where hashing should be performed.
            pub leave_bitmap: H256,
            /// Branches encountered on the leave_bitmap that need to be hashed. They correspond one-to-one with the leave_bitmap.
            pub siblings: Vec<MergeValue>,
        }

    } else {
         /// Merkle proof.
        #[derive(Debug, Serialize, Deserialize, Clone, Encode, Decode, TypeInfo, PartialEq)]
        pub struct Proof<K: Debug + Clone + TypeInfo, V: Default + Debug + Clone + TypeInfo> {
            /// The key in the KVDB.
            #[serde(flatten)]
            pub key: K,
            /// The value in the KVDB.
            #[serde(flatten)]
            pub value: V,
            /// The Merkle leaf's path (i.e., the hash value of the key).
            pub path: H256,
            /// The hash value of the Merkle leaf (i.e., the hash value of the value).
            pub value_hash: H256,
            /// Merkle root hash.
            pub root: H256,
            /// Path marker, indicating where hashing should be performed.
            pub leave_bitmap: H256,
            /// Branches encountered on the leave_bitmap that need to be hashed. They correspond one-to-one with the leave_bitmap.
            pub siblings: Vec<MergeValue>,
        }
    }

}

/// When there is only one value in the database (i.e., only one leaf, and the other leaves are empty), how to compute the root.
fn single_leaf_verify<H: Hasher + Default>(key: H256, value: H256) -> MergeValue {
    if value.is_zero() {
        MergeValue::from_h256(value)
    } else {
        let base_key = key.parent_path(0);
        let base_node = hash_base_node::<H>(0, &base_key, &value);
        let zero_bits = key;
        MergeValue::MergeWithZero {
            base_node,
            zero_bits,
            zero_count: 0,
        }
    }
}

fn into_merge_value<H: Hasher + Default>(key: H256, value: H256, height: u8) -> MergeValue {
    // try keep hash same with MergeWithZero
    if value.is_zero() || height == 0 {
        MergeValue::from_h256(value)
    } else {
        let base_key = key.parent_path(0);
        let base_node = hash_base_node::<H>(0, &base_key, &value);
        let mut zero_bits = key;
        for i in height..=core::u8::MAX {
            if key.get_bit(i) {
                zero_bits.clear_bit(i);
            }
        }
        MergeValue::MergeWithZero {
            base_node,
            zero_bits,
            zero_count: height,
        }
    }
}

/// Verify the Merkle proof,  
/// including the verification when there is only one leaf (which differs slightly from multi-leaf cases).
pub fn verify<H: Hasher + Default>(
    path: H256,
    value_hash: H256,
    leave_bitmap: H256,
    siblings: Vec<MergeValue>,
    root: H256,
) -> bool {
    if value_hash.is_zero() {
        return false;
    }
    if siblings.is_empty() {
        return single_leaf_verify::<H>(path, value_hash).hash::<H>() == root;
    }

    let mut current_path = path;
    let mut n = 0;

    let mut current_v = MergeValue::zero();

    let mut left: MergeValue = MergeValue::zero();
    let mut right: MergeValue = MergeValue::zero();

    for i in 0..=u8::MAX {
        let parent_path = current_path.parent_path(i);
        if leave_bitmap.get_bit(i) {
            if n == 0 {
                current_v = into_merge_value::<H>(path, value_hash, i);
            }
            if current_path.is_right(i) {
                left = siblings[n].clone();
                right = current_v.clone();
            } else {
                left = current_v.clone();
                right = siblings[n].clone();
            }

            n += 1;
        } else {
            if n > 0 {
                if current_path.is_right(i) {
                    left = MergeValue::zero();
                    right = current_v.clone();
                } else {
                    left = current_v.clone();
                    right = MergeValue::zero();
                }
            }
        }

        current_v = merge::<H>(i, &parent_path, &left, &right);

        current_path = parent_path;
    }
    current_v.hash::<H>() == root
}
