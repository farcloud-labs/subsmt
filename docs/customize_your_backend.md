s a developer, you don't even need to understand the specific implementation of the sparse Merkle tree, you can simply write a small amount of code to customize and extend your own Merkle tree backend. If you have such a need, you can modify the code in these few places.

## change your kv
[https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/kv.rs](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/kv.rs)

Since the values in the leaves of the Merkle tree are actually stored in a KVDB, such as RocksDB, defining your own project's key and value is very important. You only need to implement a few basic traits according to the requirements here to achieve this.

## change your hasher
[https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/keccak_hasher.rs](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/keccak_hasher.rs)
There are many types of hash algorithms in the field of cryptography, and each project has different use cases. Developers typically choose the hash algorithm that best fits their needs, such as Keccak256 used by the Ethereum community, or Poseidon, which is more suited for the field of zero-knowledge proofs. Here, you can also choose your own hash algorithm and implement it.

## Add your APIs or add permissions to your APIs.
[https://github.com/farcloud-labs/subsmt/blob/main/backend/src/main.rs](https://github.com/farcloud-labs/subsmt/blob/main/backend/src/main.rs)