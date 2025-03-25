use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use smt_primitives::verify::Proof;
use sparse_merkle_tree::{
    error::{Error, Result as SMTResult},
    traits::{Hasher, Value},
    H256,
};
use std::{fmt::Debug, path::Path};
use utoipa::{ToSchema, __dev::ComposeSchema};

/// Multi Sparse Merkle Tree Store trait
/// Defines common operations for managing multiple Sparse Merkle Trees in a single store
pub trait MSS<T, K, V, H>: Sized
where
    T: AsRef<[u8]>, // tree_id 类型参数，只要求能转换为字节切片
    K: Value + Clone + Serialize + ToSchema + Deserialize<'static> + ComposeSchema + Debug + TypeInfo,
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
{
    /// Associated type for the tree implementation
    type Tree;

    /// Open the database, create it if it does not exist
    fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self>;

    /// Create or open a new tree
    fn new_tree_with_store<'a>(&'a self, tree_id: T) -> Result<Self::Tree, Error>;

    /// Insert a value into a specific Merkle tree
    fn update(&self, tree_id: T, key: K, value: V) -> SMTResult<H256>;

    /// Insert multiple values into a Merkle tree at once
    fn update_all(&self, tree_id: T, kvs: Vec<(K, V)>) -> SMTResult<H256>;

    /// Get the root hash of a specific tree
    fn get_root(&self, tree_id: T) -> Result<H256, Error>;

    /// Get the value of a specific key in a particular tree
    fn get_value(&self, tree_id: T, key: K) -> Result<V, Error>;

    /// Get the Merkle proof for a specific key
    fn get_merkle_proof(&self, tree_id: T, key: K) -> Result<Proof<K, V>, Error>;

    /// Get the Merkle proof in raw bytes format
    fn get_merkle_proof_old(&self, tree_id: T, keys: Vec<K>) -> SMTResult<Vec<u8>>;

    /// Calculate the future root hash before updating data
    fn get_next_root(&self, old_proof: Vec<u8>, next_kvs: Vec<(K, V)>) -> Result<H256, Error>;

    /// Delete/clear a specific Merkle tree
    fn clear(&self, tree_id: T) -> Result<(), Error>;

    /// Verify a Merkle proof
    fn verify(&self, proof: Proof<K, V>) -> bool;
}
