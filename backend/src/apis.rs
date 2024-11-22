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

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(elided_named_lifetimes)]
use crate::store::SMTStore;
use ethers::core::k256::sha2::digest::Key;
use kvdb_rocksdb::Database;
use smt_primitives::{
    keccak_hasher::Keccak256Hasher,
    verify::{verify as smt_verify, Proof},
};
use sparse_merkle_tree::{
    merge::MergeValue,
    traits::{Hasher, Value},
    SparseMerkleTree, H256,
};
use std::{fmt::Debug, io, marker::PhantomData, path::Path};

use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sparse_merkle_tree::{
    error::{Error, Result, Result as SMTResult},
    CompiledMerkleProof,
};
use std::{convert::AsRef, sync::Arc};
use utoipa::{ToSchema, __dev::ComposeSchema};

type MultiSMT<'a, V, H> = SparseMerkleTree<H, V, SMTStore<'a>>;

pub struct MultiSMTStore<K, V, H> {
    store: Arc<Database>,
    v: PhantomData<(K, V, H)>,
}

impl<
        'a,
        K: Value + Clone + Serialize + ToSchema + Deserialize<'a> + ComposeSchema + Debug + TypeInfo,
        V: Default
            + Value
            + Into<Vec<u8>>
            + From<Vec<u8>>
            + ToSchema
            + Serialize
            + Deserialize<'a>
            + ComposeSchema
            + PartialEq
            + Clone
            + Debug
            + TypeInfo,
        H: Hasher + Default,
    > MultiSMTStore<K, V, H>
{
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let db = Database::open(&Default::default(), path)?;
        Ok(Self {
            store: db.into(),
            v: Default::default(),
        })
    }

    pub fn new_tree_with_store(&'a self, prefix: &'a [u8]) -> Result<MultiSMT<V, H>> {
        let db = SMTStore::new(self.store.clone(), Default::default(), prefix);
        MultiSMT::new_with_store(db)
    }

    // 给某个默克尔树插入值
    pub fn update(&'a self, prefix: &'a [u8], key: K, value: V) -> SMTResult<H256> {
        let mut tree = self.new_tree_with_store(prefix)?;
        let h = tree.update(key.to_h256(), value)?;
        Ok(*h)
    }

    pub fn update_all(&'a self, prefix: &'a [u8], kvs: Vec<(K, V)>) -> SMTResult<H256> {
        let kvs = kvs
            .into_iter()
            .map(|(k, v)| Ok((k.to_h256(), v)))
            .collect::<Result<Vec<(H256, V)>>>()?;

        let mut tree = self.new_tree_with_store(prefix)?;
        let root = tree.update_all(kvs)?;
        Ok(*root)
    }

    // 获取根
    pub fn get_root(&'a self, prefix: &'a [u8]) -> Result<H256> {
        let tree = self.new_tree_with_store(prefix)?;
        Ok(*tree.root())
    }

    // 获取值
    pub fn get_value(&'a self, prefix: &'a [u8], key: K) -> Result<V> {
        let tree = self.new_tree_with_store(prefix)?;
        let value = tree.get(&key.to_h256())?;
        Ok(value)
    }

    // 获取证明
    pub fn get_merkle_proof(&'a self, prefix: &'a [u8], key: K) -> Result<Proof<K, V>> {
        let tree = self.new_tree_with_store(prefix)?;
        let proof = tree.merkle_proof(vec![key.to_h256()])?;
        let leaves_bitmap = proof.leaves_bitmap();
        let siblings = proof.merkle_path();
        let leave_bitmap = leaves_bitmap[0];
        let value = self.get_value(prefix, key.clone())?;

        Ok(Proof {
            key: key.clone(),
            value: value.clone(),
            path: key.to_h256(),
            value_hash: value.to_h256(),
            root: *tree.root(),
            leave_bitmap,
            siblings: siblings.clone(),
        })
    }

    pub fn get_merkle_proof_old(&'a self, prefix: &'a [u8], keys: Vec<K>) -> SMTResult<Vec<u8>> {
        let tree = self.new_tree_with_store(prefix)?;
        let keys = keys
            .into_iter()
            .map(|k| Ok(k.to_h256()))
            .collect::<Result<Vec<H256>>>()?;

        let proof = tree.merkle_proof(keys.clone()).unwrap();
        let proof = proof.compile(keys)?;
        Ok(proof.0)
    }

    pub fn get_next_root(&'a self, old_proof: Vec<u8>, next_kvs: Vec<(K, V)>) -> Result<H256> {
        let p = CompiledMerkleProof(old_proof);
        let kvs = next_kvs
            .into_iter()
            .map(|(k, v)| Ok((k.to_h256(), v.to_h256())))
            .collect::<Result<Vec<(H256, H256)>>>()?;

        let next_root = p.compute_root::<H>(kvs)?;
        Ok(next_root)
    }

    // 删除某个默克尔树
    pub fn clear(&'a self, prefix: &'a [u8]) {
        let mut tx = self.store.transaction();
        tx.delete_prefix(Default::default(), prefix);
        self.store.write(tx).unwrap();
    }

    pub fn verify(&'a self, proof: Proof<K, V>) -> bool {
        let mut res = false;
        if proof.value != V::default() {
            res = smt_verify::<H>(
                proof.path,
                proof.value_hash,
                proof.leave_bitmap,
                proof.siblings,
                proof.root,
            )
        }
        res
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use actix_web::web;
    use smt_primitives::kv::{SMTKey, SMTValue};
    use std::sync::Mutex;
    #[test]
    fn test_apis() {
        // 创建multi_tree
        let base_path = "./apis_test_db";
        let multi_tree =
            MultiSMTStore::<SMTKey, SMTValue, Keccak256Hasher>::open(Path::new(base_path)).unwrap();

        let tree1: &[u8] = "tree1".as_ref();
        let tree2: &[u8] = "tree2".as_ref();
        multi_tree.clear(tree1);
        multi_tree.clear(tree2);
        multi_tree.new_tree_with_store(tree1).unwrap();
        multi_tree.new_tree_with_store(tree2).unwrap();

        // 分别取两个tree的root
        assert_eq!(multi_tree.get_root(tree1).unwrap(), H256::zero());
        assert_eq!(multi_tree.get_root(tree2).unwrap(), H256::zero());

        // 插入一个tree数据
        let tree1_key1 = SMTKey {
            address: "1".to_string(),
        };
        let tree1_value1: SMTValue = SMTValue {
            nonce: 1,
            balance: 99,
        };
        let tree1_key2 = SMTKey {
            address: "2".to_string(),
        };
        let tree1_value2: SMTValue = SMTValue {
            nonce: 2,
            balance: 97,
        };

        assert_eq!(
            multi_tree.get_value(tree1, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(multi_tree.get_root(tree1).unwrap(), H256::zero());
        multi_tree
            .update(tree1, tree1_key1.clone(), tree1_value1.clone())
            .unwrap();
        assert_eq!(
            multi_tree.get_value(tree1, tree1_key1.clone()).unwrap(),
            tree1_value1.clone()
        );
        let proof = multi_tree
            .get_merkle_proof(tree1, tree1_key1.clone())
            .unwrap();
        assert_eq!(multi_tree.verify(proof), true);
        // remove
        multi_tree
            .update(tree1, tree1_key1.clone(), SMTValue::default())
            .unwrap();
        assert_eq!(
            multi_tree.get_value(tree1, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(multi_tree.get_root(tree1).unwrap(), H256::zero());
        multi_tree
            .update(tree1, tree1_key1.clone(), tree1_value1.clone())
            .unwrap();
        let tree1_root1 = multi_tree.get_root(tree1).unwrap();
        let old_proof = multi_tree
            .get_merkle_proof_old(tree1, vec![tree1_key2.clone()])
            .unwrap();
        let _next_root = multi_tree
            .get_next_root(
                old_proof.clone(),
                vec![(tree1_key2.clone(), tree1_value2.clone())],
            )
            .unwrap();
        multi_tree
            .update(tree1, tree1_key2.clone(), tree1_value2.clone())
            .unwrap();
        let tree1_root2 = multi_tree.get_root(tree2).unwrap();
        assert_ne!(tree1_root1, tree1_root2);
        assert_eq!(multi_tree.get_root(tree2).unwrap(), H256::zero());

        let tree2_root1 = multi_tree
            .update(tree2, tree1_key1.clone(), tree1_value1.clone())
            .unwrap();
        multi_tree
            .update(tree2, tree1_key2.clone(), tree1_value2.clone())
            .unwrap();
        assert_eq!(tree1_root1, tree2_root1);
        assert_eq!(
            multi_tree.get_root(tree1).unwrap(),
            multi_tree.get_root(tree2).unwrap()
        );
        let tree2_proof1 = multi_tree
            .get_merkle_proof(tree2, tree1_key2.clone())
            .unwrap();
        assert_eq!(multi_tree.verify(tree2_proof1), true);

        // clear
        multi_tree.clear(tree1);
        assert_eq!(
            multi_tree.get_value(tree1, tree1_key2.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(
            multi_tree.get_value(tree2, tree1_key2.clone()).unwrap(),
            tree1_value2.clone()
        );
        multi_tree.clear(tree2);
        assert_eq!(
            multi_tree.get_value(tree2, tree1_key2.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(
            multi_tree.get_value(tree2, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(multi_tree.get_root(tree2).unwrap(), H256::zero());
        let mut kvs: Vec<(SMTKey, SMTValue)> = vec![];

        for i in 1..2 {
            kvs.push((
                SMTKey {
                    address: i.to_string(),
                },
                SMTValue {
                    nonce: i as u64,
                    balance: i as u128,
                },
            ));
        }

        for kv in kvs.clone() {
            multi_tree
                .update(tree2, kv.0.clone(), kv.1.clone())
                .unwrap();
            let p = multi_tree.get_merkle_proof(tree2, kv.0.clone()).unwrap();
            assert_eq!(multi_tree.verify(p), true);
        }

        multi_tree.clear(tree1);
        for kv in kvs.clone() {
            multi_tree
                .update_all(tree1, vec![(kv.0.clone(), kv.1.clone())])
                .unwrap();
            let p = multi_tree.get_merkle_proof(tree1, kv.0.clone()).unwrap();
            assert_eq!(multi_tree.verify(p), true);
        }
        assert_eq!(
            multi_tree.get_root(tree1).unwrap(),
            multi_tree.get_root(tree2).unwrap()
        );
        multi_tree.update_all(tree1, kvs.clone()).unwrap();
    }
}
