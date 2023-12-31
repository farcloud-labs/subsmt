use sparse_merkle_tree::{traits::Hasher, H256, merge::{merge, MergeValue, hash_base_node}};
use crate::keccak256_hasher::Keccak256Hasher;
use frame_support::dispatch::Vec;


fn single_leaf_into_merge_value<H: Hasher + Default>(key: H256, value: H256) -> MergeValue {
    if value.is_zero() {
        MergeValue::from_h256(value)
    } else {
        let base_key = key.parent_path(0);
        let base_node = hash_base_node::<H>(0, &base_key, &value);
        let mut zero_bits = key;
        let res = MergeValue::MergeWithZero {
            base_node,
            zero_bits,
            zero_count: 0,
        };
        res
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

pub fn verify(
    path: H256,
    value_hash: H256,
    leaves_bitmap: H256,
    siblings: Vec<MergeValue>,
    old_root: H256,
) -> bool {

    if siblings.len() == 0 {
        return single_leaf_into_merge_value::<Keccak256Hasher>(path, value_hash).hash::<Keccak256Hasher>() == old_root;
    }

    let mut current_path = path;
    let mut n = 0;

    let mut current_v = MergeValue::zero();

    let mut left: MergeValue = MergeValue::zero();
    let mut right: MergeValue = MergeValue::zero();

    for i in 0..=u8::MAX {
        let parent_path = current_path.parent_path(i);
        if leaves_bitmap.get_bit(i) {
            if n == 0 {
                current_v = into_merge_value::<Keccak256Hasher>(path, value_hash, i);
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

        current_v = merge::<Keccak256Hasher>(i, &parent_path, &left, &right);

        current_path = parent_path;
    }
    current_v.hash::<Keccak256Hasher>() == old_root
}