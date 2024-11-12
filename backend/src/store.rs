use sparse_merkle_tree::{
    error::Error,
    traits::{StoreReadOps, StoreWriteOps, Value},
    BranchKey, BranchNode, H256,
};
use std::marker::PhantomData;
use std::sync::Arc;

use codec::{Decode, Encode};
// use rocksdb::{DBCommon, DB, DBWithThreadMode, OptimisticTransactionDB, ThreadMode};
use kvdb_rocksdb::Database;

pub struct SMTStore<'a> {
    inner: Arc<Database>,
    col: u32,
    prefix: &'a [u8],
}

impl<'a> SMTStore<'a> {
    pub fn new(db: Arc<Database>, col: u32, prefix: &'a [u8]) -> Self {
        SMTStore {
            inner: db,
            col,
            prefix,
        }
    }
}

impl<'a, V> StoreWriteOps<V> for SMTStore<'a>
where
    V: Value + Into<Vec<u8>>,
{
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.put(
            self.col,
            &[self.prefix, &node_key.encode()].concat(),
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
            self.col,
            &[self.prefix, &leaf_key.encode()].concat(),
            &leaf.into(),
        );
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.delete(self.col, &[self.prefix, &node_key.encode()].concat());
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error> {
        let mut tx = self.inner.transaction();
        tx.delete(self.col, &[self.prefix, &leaf_key.encode()].concat());
        self.inner
            .write(tx)
            .map_err(|e| Error::Store(e.to_string()))
    }
}

impl<'a, V> StoreReadOps<V> for SMTStore<'a>
where
    V: Value + From<Vec<u8>>,
{
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        self.inner
            .get(self.col, &[self.prefix, &branch_key.encode()].concat())
            .map(|s| s.map(|v| BranchNode::decode(&mut v.as_slice()).unwrap()))
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error> {
        self.inner
            .get(self.col, &[self.prefix, &leaf_key.as_slice()].concat())
            .map(|s| s.map(|v| v.into()))
            .map_err(|e| Error::Store(e.to_string()))
    }
}
