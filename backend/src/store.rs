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

//! Store the Merkle tree data in a KVDB (Key-Value Database).

#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]
use sparse_merkle_tree::{
    error::Error,
    traits::{StoreReadOps, StoreWriteOps, Value},
    BranchKey, BranchNode, H256,
};
use std::{marker::PhantomData, sync::Arc};

use codec::{Decode, Encode};
// use rocksdb::{DBCommon, DB, DBWithThreadMode, OptimisticTransactionDB, ThreadMode};
use kvdb_rocksdb::Database;

pub struct SMTStore {
    inner: Arc<Database>,
    // col: u32, // 这个也是用不到 默认值是0
    prefix: String,
}

impl SMTStore {
    pub fn new(db: Arc<Database>, prefix: impl Into<String>) -> Self {
        SMTStore {
            inner: db,
            // col, 
            prefix: prefix.into(),
        }
    }
}

impl<V> StoreWriteOps<V> for SMTStore
where
    V: Value + Into<Vec<u8>>,
{
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.put(
            Default::default(),
            &[self.prefix.as_bytes(), &node_key.encode()].concat(),
            &branch.encode(),
        );
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }

    // 叶子就是数据
    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.put(
            Default::default(),
            &[self.prefix.as_bytes(), &leaf_key.encode()].concat(),
            &leaf.into(),
        );
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.delete(Default::default(), &[self.prefix.as_bytes(), &node_key.encode()].concat());
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.delete(Default::default(), &[self.prefix.as_bytes(), &leaf_key.encode()].concat());
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }
}

impl<V> StoreReadOps<V> for SMTStore
where
    V: Value + From<Vec<u8>>,
{
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        self.inner
            .get(Default::default(), &[self.prefix.as_bytes(), &branch_key.encode()].concat())
            .map(|s| s.map(|v| BranchNode::decode(&mut v.as_slice()).unwrap()))
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error> {
        self.inner
            .get(Default::default(), &[self.prefix.as_bytes(), leaf_key.as_slice()].concat())
            .map(|s| s.map(|v| v.into()))
            .map_err(|e| Error::Store(e.to_string()))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use smt_primitives::kv::{SMTKey, SMTValue};
    use sparse_merkle_tree::{merge::MergeValue, traits::Value};
    use std::path::Path;

    #[test]
    fn test_store() {
        // 打开数据库
        let base_path = "./test_store_db";
        let db = Database::open(&Default::default(), Path::new(base_path)).unwrap();
        let mut store = SMTStore::new(Arc::new(db), "test");

        //插入叶子
        let leaf1_key: H256 = [1u8; 32].to_vec().into();
        let leaf1 = SMTValue {
            nonce: 1,
            balance: 99,
        };
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), None::<SMTValue>);
        store.insert_leaf(leaf1_key, leaf1.clone()).unwrap();
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), Some(leaf1));
        <SMTStore as StoreWriteOps<SMTValue>>::remove_leaf(&mut store, &leaf1_key).unwrap();
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), None::<SMTValue>);

        // 插入
        let node1_key: BranchKey = BranchKey::new(100, [2u8; 32].into());
        let node1: BranchNode = BranchNode {
            left: MergeValue::from_h256([3u8; 32].into()),
            right: MergeValue::from_h256([4u8; 32].into()),
        };
        assert_eq!(
            <SMTStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            None::<BranchNode>
        );
        <SMTStore as StoreWriteOps<SMTValue>>::insert_branch(
            &mut store,
            node1_key.clone(),
            node1.clone(),
        )
        .unwrap();
        assert_eq!(
            <SMTStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            Some(node1.clone())
        );
        <SMTStore as StoreWriteOps<SMTValue>>::remove_branch(&mut store, &node1_key.clone())
            .unwrap();
        assert_eq!(
            <SMTStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            None::<BranchNode>
        );
    }
}
