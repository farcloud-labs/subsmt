#![allow(dead_code)]
#![allow(unused_imports)]
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
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sparse_merkle_tree::error::Result;
use sparse_merkle_tree::error::{Error, Result as SMTResult};
use sparse_merkle_tree::CompiledMerkleProof;
use std::convert::AsRef;
use std::sync::Arc;
use utoipa::{ToSchema, __dev::ComposeSchema};

type MultiSMT<'a, V, H: Hasher> = SparseMerkleTree<H, V, SMTStore<'a>>;

pub struct MultiSMTStore<K, V, H> {
    store: Arc<Database>,
    v: PhantomData<(K, V, H)>,
}

impl<
        'a,
        K: Value + Clone + Serialize + ToSchema + Deserialize<'a> + ComposeSchema,
        V: Default +Value + Into<Vec<u8>> + From<Vec<u8>> + ToSchema + Serialize + Deserialize<'a> + ComposeSchema + PartialEq,
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
        tree.update(key.to_h256(), value).copied()
    }

    pub fn update_all(&'a self, prefix: &'a [u8], kvs: Vec<(K, V)>) -> SMTResult<H256> {
        let kvs = kvs
            .into_iter()
            .map(|(k, v)| Ok((k.to_h256(), v)))
            .collect::<Result<Vec<(H256, V)>>>()?;

        let mut tree = self.new_tree_with_store(prefix)?;
        let root = tree.update_all(kvs)?;
        Ok(root.clone())
    }

    // 获取根
    pub fn get_root(&'a self, prefix: &'a [u8]) -> Result<H256> {
        let tree = self.new_tree_with_store(prefix)?;
        Ok(tree.root().clone())
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
            key: key,
            value: value,
            root: tree.root().clone(),
            leave_bitmap: leave_bitmap,
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
                proof.key.to_h256(),
                proof.value.to_h256(),
                proof.leave_bitmap,
                proof.siblings,
                proof.root,
            )
        }
        return res;
        
    }
}

// todo 获取的值为空 不提供证明
// todo value为默认值 验证直接false

#[cfg(test)]
pub mod test {
    use crate::kvs::{SMTKey, SMTValue};

    // 创建multi_tree

    // 创建两个tree

    // 分别取两个tree的root

    // 插入一个tree数据

    // 查询两个树的root 证明互不干扰

    // 插入另外一个树的数据 证明两个树同样数据获得根相同

    // 获取数据

    // 获得证明

    // 验证

    // 清除数据

    // 获取根

    // 获取值

}