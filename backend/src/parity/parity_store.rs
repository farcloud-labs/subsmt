use codec::{Decode, Encode};
use sparse_merkle_tree::{
    error::Error,
    traits::{StoreReadOps, StoreWriteOps, Value},
    BranchKey, BranchNode, H256,
};
use std::sync::Arc;

use crate::parity_db::ParityDb;
use std::sync::Mutex;

pub struct SMTParityStore {
    inner: Arc<Mutex<ParityDb>>,
    col: u8,
}

impl SMTParityStore {
    pub fn new(db: Arc<Mutex<ParityDb>>, col: u8) -> Self {
        SMTParityStore { inner: db, col }
    }
}

impl<V> StoreWriteOps<V> for SMTParityStore
where
    V: Value + Into<Vec<u8>>,
{
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error> {
        self.inner.lock().unwrap()
            .insert(self.col, &node_key.encode(), &branch.encode())
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error> {
        self.inner.lock().unwrap()
            .insert(self.col, &leaf_key.encode(), &leaf.into())
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error> {
        self.inner.lock().unwrap()
            .delete(self.col, &node_key.encode())
            .map_err(|e| Error::Store(e.to_string()))
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error> {
        self.inner.lock().unwrap()
            .delete(self.col, &leaf_key.encode())
            .map_err(|e| Error::Store(e.to_string()))
    }
}

impl<V> StoreReadOps<V> for SMTParityStore
where
    V: Value + From<Vec<u8>>,
{
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error> {
        self.inner.lock().unwrap()
            .get(self.col, &branch_key.encode())
            .map_err(|e| Error::Store(e.to_string()))?
            .map(|v| BranchNode::decode(&mut v.as_slice()).unwrap())
            .map_or(Ok(None), |v| Ok(Some(v)))
    }

    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error> {
        self.inner.lock().unwrap()
            .get(self.col, &leaf_key.encode())
            .map_err(|e| Error::Store(e.to_string()))?
            .map(|v| v.into())
            .map_or(Ok(None), |v| Ok(Some(v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smt_primitives::kv::SMTValue;
    use sparse_merkle_tree::merge::MergeValue;
    use tempfile::tempdir;

    #[test]
    fn test_store() {
        // 创建数据库实例
        let temp_dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(ParityDb::new(temp_dir.path(), 2)));
        let mut store = SMTParityStore::new(db, 0);

        // 插入叶子
        let leaf1_key: H256 = [1u8; 32].to_vec().into();
        let leaf1 = SMTValue {
            nonce: 1,
            balance: 99,
        };
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), None::<SMTValue>);
        store.insert_leaf(leaf1_key, leaf1.clone()).unwrap();
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), Some(leaf1));
        <SMTParityStore as StoreWriteOps<SMTValue>>::remove_leaf(&mut store, &leaf1_key).unwrap();
        assert_eq!(store.get_leaf(&leaf1_key).unwrap(), None::<SMTValue>);

        // 插入分支节点
        let node1_key: BranchKey = BranchKey::new(100, [2u8; 32].into());
        let node1: BranchNode = BranchNode {
            left: MergeValue::from_h256([3u8; 32].into()),
            right: MergeValue::from_h256([4u8; 32].into()),
        };
        assert_eq!(
            <SMTParityStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            None::<BranchNode>
        );
        <SMTParityStore as StoreWriteOps<SMTValue>>::insert_branch(
            &mut store,
            node1_key.clone(),
            node1.clone(),
        )
        .unwrap();
        assert_eq!(
            <SMTParityStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            Some(node1.clone())
        );
        <SMTParityStore as StoreWriteOps<SMTValue>>::remove_branch(&mut store, &node1_key).unwrap();
        assert_eq!(
            <SMTParityStore as StoreReadOps<SMTValue>>::get_branch(&store, &node1_key).unwrap(),
            None::<BranchNode>
        );
    }
}
