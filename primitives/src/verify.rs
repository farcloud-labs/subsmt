#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::collapsible_else_if)]

extern crate alloc;

// use core::fmt::Debug;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::prelude::fmt::Debug;
use scale_info::TypeInfo;
use serde;
use serde::{Deserialize, Serialize};
use sparse_merkle_tree::{
    merge::{hash_base_node, merge, MergeValue},
    traits::Hasher,
    H256,
};

cfg_if::cfg_if! {
    if #[cfg(feature="std")] {
        use utoipa::{ToSchema, IntoParams, __dev::ComposeSchema};

        #[derive(Debug, Serialize, Deserialize, Clone, Encode, Decode, TypeInfo, PartialEq, ToSchema)]
        pub struct Proof<K: Debug + Clone + TypeInfo, V: Default + Debug + Clone + TypeInfo> {
            #[serde(flatten)]
            pub key: K,
            #[serde(flatten)]
            pub value: V,
            pub path: H256,
            pub value_hash: H256,
            pub root: H256,
            pub leave_bitmap: H256,
            pub siblings: Vec<MergeValue>,
        }

    } else {
        #[derive(Debug, Serialize, Deserialize, Clone, Encode, Decode, TypeInfo, PartialEq)]
        pub struct Proof<K: Debug + Clone + TypeInfo, V: Default + Debug + Clone + TypeInfo> {
            #[serde(flatten)]
            pub key: K,
            #[serde(flatten)]
            pub value: V,
            pub path: H256,
            pub value_hash: H256,
            pub root: H256,
            pub leave_bitmap: H256,
            pub siblings: Vec<MergeValue>,
        }
    }

}

fn single_leaf_into_merge_value<H: Hasher + Default>(key: H256, value: H256) -> MergeValue {
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

pub fn verify<H: Hasher + Default>(
    path: H256,       // key的hash
    value_hash: H256, // value的hash
    leave_bitmap: H256,
    siblings: Vec<MergeValue>,
    root: H256,
) -> bool {
    if value_hash.is_zero() {
        return false;
    }
    if siblings.is_empty() {
        return single_leaf_into_merge_value::<H>(path, value_hash).hash::<H>() == root;
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
