use std::marker::PhantomData;
use sparse_merkle_tree::{
    error::Error,
    traits::{StoreReadOps, StoreWriteOps, Value},
    BranchKey, BranchNode, H256,
};
use codec::{Encode, Decode};
use rocksdb::{DBCommon, DB, DBWithThreadMode, OptimisticTransactionDB, ThreadMode};

pub struct DefaultStore<'a, T: ThreadMode> {
    inner: &'a OptimisticTransactionDB<T>,
}

impl<'a, T: ThreadMode> DefaultStore<'a, T> {
    pub fn new(db: &'a OptimisticTransactionDB<T>) -> Self {
        DefaultStore {
            inner: db,
        }
    }
}


impl<'a, V, T> StoreWriteOps<V> for DefaultStore<'a, T>
where
    V: Value + AsRef<[u8]>,
    T: ThreadMode,
{
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        self.inner.put(node_key.encode(), branch.encode()).map_err(|e| Error::Store(e.to_string()))
    }

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error> {
        self.inner.put(leaf_key.encode(), leaf.as_ref()).map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error> {
        self.inner.delete(node_key.encode()).map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error> {
        self.inner.delete(leaf_key.encode()).map_err(|e| Error::Store(e.to_string()))
    }
}

impl<'a, V, T> StoreReadOps<V> for DefaultStore<'a, T>
where
    V: Value + AsRef<[u8]> + From<Vec<u8>>,
    T: ThreadMode, {
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        self.inner.get(branch_key.encode()).map(|s| s.map(|v| BranchNode::decode(&mut v.as_slice()).unwrap())).map_err(|e| Error::Store(e.to_string()))
    }

    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error> {
        self.inner
            .get(leaf_key.as_slice())
            .map(|s| s.map(|v| v.into()))
            .map_err(|e| Error::Store(e.to_string()))
    }
}

pub struct DefaultStoreMultiTree<'a, T:ThreadMode> {
    prefix: &'a [u8],
    inner: &'a OptimisticTransactionDB<T>,
}

impl<'a, V, T> StoreWriteOps<V> for DefaultStoreMultiTree<'a, T>
where
    V: Value + AsRef<[u8]>,
    T: ThreadMode,
{
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        self.inner.put([self.prefix, &node_key.encode()].concat(), branch.encode()).map_err(|e| Error::Store(e.to_string()))
    }

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error> {
        self.inner.put([self.prefix, &leaf_key.encode()].concat(), leaf.as_ref()).map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error> {
        self.inner.delete([self.prefix, &node_key.encode()].concat()).map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error> {
        self.inner.delete([self.prefix, &leaf_key.encode()].concat()).map_err(|e| Error::Store(e.to_string()))
    }
}

impl<'a, V, T> StoreReadOps<V> for DefaultStoreMultiTree<'a, T>
where
    V: Value + AsRef<[u8]> + From<Vec<u8>>,
    T: ThreadMode, {
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        self.inner.get([self.prefix, &branch_key.encode()].concat()).map(|s| s.map(|v| BranchNode::decode(&mut v.as_slice()).unwrap())).map_err(|e| Error::Store(e.to_string()))
    }

    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error> {
        self.inner
            .get([self.prefix, leaf_key.as_slice()].concat())
            .map(|s| s.map(|v| v.into()))
            .map_err(|e| Error::Store(e.to_string()))
    }
}




