use crate::parity_db::ParityDb;
use crate::parity_store::SMTParityStore;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use smt_primitives::{
    // keccak_hasher::Keccak256Hasher,
    verify::{verify as smt_verify, Proof},
};
use sparse_merkle_tree::{
    error::{Error, Result as SMTResult},
    // merge::MergeValue,
    traits::{Hasher, Value},
    CompiledMerkleProof,
    SparseMerkleTree,
    H256,
};
use std::{fmt::Debug, marker::PhantomData, path::Path, sync::Arc};
use utoipa::{ToSchema, __dev::ComposeSchema};

type MultiSMT<V, H> = SparseMerkleTree<H, V, SMTParityStore>;

/// Multiple Merkle trees are stored in a ParityDb database
pub struct MultiSMTParityStore<K, V, H> {
    store: Arc<ParityDb>,
    v: PhantomData<(K, V, H)>,
}

impl<
        K: Value
            + Clone
            + Serialize
            + ToSchema
            + Deserialize<'static>
            + ComposeSchema
            + Debug
            + TypeInfo,
        V: Default
            + Value
            + Into<Vec<u8>>
            + From<Vec<u8>>
            + ToSchema
            + Serialize
            + Deserialize<'static>
            + ComposeSchema
            + PartialEq
            + Clone
            + Debug
            + TypeInfo,
        H: Hasher + Default,
    > MultiSMTParityStore<K, V, H>
{
    /// Open the ParityDb database, create it if it does not exist.
    pub fn open<P: AsRef<Path>>(path: P, num_columns: u8) -> std::io::Result<Self> {
        let db = ParityDb::new(path.as_ref(), num_columns);
        Ok(Self {
            store: Arc::new(db),
            v: Default::default(),
        })
    }

    /// Create or open a new tree with specified column
    pub fn new_tree_with_store(&self, col: u8) -> Result<MultiSMT<V, H>, Error> {
        let db = SMTParityStore::new(self.store.clone(), col);
        MultiSMT::new_with_store(db)
    }

    /// Insert a value into a specific Merkle tree
    pub fn update(&self, col: u8, key: K, value: V) -> SMTResult<H256> {
        let mut tree = self.new_tree_with_store(col)?;
        let h = tree.update(key.to_h256(), value)?;
        Ok(*h)
    }

    /// Insert multiple values into a Merkle tree at once
    pub fn update_all(&self, col: u8, kvs: Vec<(K, V)>) -> SMTResult<H256> {
        let kvs = kvs
            .into_iter()
            .map(|(k, v)| Ok((k.to_h256(), v)))
            .collect::<Result<Vec<(H256, V)>, Error>>()?;

        let mut tree = self.new_tree_with_store(col)?;
        let root = tree.update_all(kvs)?;
        Ok(*root)
    }

    /// Get the root hash
    pub fn get_root(&self, col: u8) -> Result<H256, Error> {
        let tree = self.new_tree_with_store(col)?;
        Ok(*tree.root())
    }

    /// Get the value of a specific key in a particular tree
    pub fn get_value(&self, col: u8, key: K) -> Result<V, Error> {
        let tree = self.new_tree_with_store(col)?;
        let value = tree.get(&key.to_h256())?;
        Ok(value)
    }

    /// Get the Merkle proof
    pub fn get_merkle_proof(&self, col: u8, key: K) -> Result<Proof<K, V>, Error> {
        let tree = self.new_tree_with_store(col)?;
        let proof = tree.merkle_proof(vec![key.to_h256()])?;
        let leaves_bitmap = proof.leaves_bitmap();
        let siblings = proof.merkle_path();
        let leave_bitmap = leaves_bitmap[0];
        let value = self.get_value(col, key.clone())?;

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

    /// Get the Merkle proof, the return value is `Vec<u8>`
    pub fn get_merkle_proof_old(&self, col: u8, keys: Vec<K>) -> SMTResult<Vec<u8>> {
        let tree = self.new_tree_with_store(col)?;
        let keys = keys
            .into_iter()
            .map(|k| Ok(k.to_h256()))
            .collect::<Result<Vec<H256>, Error>>()?;

        let proof = tree.merkle_proof(keys.clone())?;
        let proof = proof.compile(keys)?;
        Ok(proof.0)
    }

    /// Before data is updated, the future value of the root hash can be calculated in advance
    pub fn get_next_root(&self, old_proof: Vec<u8>, next_kvs: Vec<(K, V)>) -> Result<H256, Error> {
        let p = CompiledMerkleProof(old_proof);
        let kvs = next_kvs
            .into_iter()
            .map(|(k, v)| Ok((k.to_h256(), v.to_h256())))
            .collect::<Result<Vec<(H256, H256)>, Error>>()?;

        let next_root = p.compute_root::<H>(kvs)?;
        Ok(next_root)
    }

    /// Delete a specific Merkle tree by clearing its column
    pub fn clear(&self, col: u8) -> Result<(), Error> {
        self.store
            .clear_column(col)
            .map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }

    /// Verify the Merkle proof
    pub fn verify(&self, proof: Proof<K, V>) -> bool {
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
mod tests {
    use super::*;
    use smt_primitives::{kv::{SMTKey, SMTValue}, keccak_hasher::Keccak256Hasher};
    use tempfile::tempdir;

    #[test]
    fn test_apis() {
        // Create multi_tree
        let temp_dir = tempdir().unwrap();
        let multi_tree =
            MultiSMTParityStore::<SMTKey, SMTValue, Keccak256Hasher>::open(temp_dir.path(), 2)
                .unwrap();

        let tree1_col: u8 = 0;
        let tree2_col: u8 = 1;
        multi_tree.clear(tree1_col).unwrap();
        multi_tree.clear(tree2_col).unwrap();
        multi_tree.new_tree_with_store(tree1_col).unwrap();
        multi_tree.new_tree_with_store(tree2_col).unwrap();

        // Get roots from both trees
        assert_eq!(multi_tree.get_root(tree1_col).unwrap(), H256::zero());
        assert_eq!(multi_tree.get_root(tree2_col).unwrap(), H256::zero());

        // Insert data into tree1
        let tree1_key1 = SMTKey {
            address: "1".to_string(),
        };
        let tree1_value1 = SMTValue {
            nonce: 1,
            balance: 99,
        };
        let tree1_key2 = SMTKey {
            address: "2".to_string(),
        };
        let tree1_value2 = SMTValue {
            nonce: 2,
            balance: 97,
        };

        assert_eq!(
            multi_tree.get_value(tree1_col, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );
        assert_eq!(multi_tree.get_root(tree1_col).unwrap(), H256::zero());

        // Update and verify
        multi_tree
            .update(tree1_col, tree1_key1.clone(), tree1_value1.clone())
            .unwrap();
        assert_eq!(
            multi_tree.get_value(tree1_col, tree1_key1.clone()).unwrap(),
            tree1_value1.clone()
        );

        let proof = multi_tree
            .get_merkle_proof(tree1_col, tree1_key1.clone())
            .unwrap();
        assert!(multi_tree.verify(proof));

        // Test remove
        multi_tree
            .update(tree1_col, tree1_key1.clone(), SMTValue::default())
            .unwrap();
        assert_eq!(
            multi_tree.get_value(tree1_col, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );

        // Test multiple updates
        multi_tree
            .update(tree1_col, tree1_key1.clone(), tree1_value1.clone())
            .unwrap();
        let _tree1_root1 = multi_tree.get_root(tree1_col).unwrap();

        let old_proof = multi_tree
            .get_merkle_proof_old(tree1_col, vec![tree1_key2.clone()])
            .unwrap();
        let _next_root = multi_tree
            .get_next_root(old_proof, vec![(tree1_key2.clone(), tree1_value2.clone())])
            .unwrap();

        let tree2_root1 = multi_tree
            .update(tree1_col, tree1_key2.clone(), tree1_value2.clone())
            .unwrap();

        assert_eq!(_next_root, tree2_root1);

        // Test clear
        multi_tree.clear(tree1_col).unwrap();
        assert_eq!(
            multi_tree.get_value(tree1_col, tree1_key1.clone()).unwrap(),
            SMTValue::default()
        );

        // Test batch update
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

        multi_tree.update_all(tree1_col, kvs.clone()).unwrap();
    }
}
