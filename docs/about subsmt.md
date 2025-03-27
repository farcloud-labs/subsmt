# About SubSMT

SubSMT is a comprehensive sparse Merkle tree implementation designed specifically for the Polkadot ecosystem. It provides a unified solution for projects that need to store data off-chain while maintaining data integrity through on-chain verification.

## What is SubSMT?

SubSMT is a Rust-based implementation that includes:

1. A substrate pallet for on-chain verification
2. An ink! smart contract implementation 
3. Multiple backend storage solutions (RocksDB and ParityDB)
4. A common backend interface for extensibility

## Why Use SubSMT?

As blockchain ecosystems grow, storing all data on-chain becomes increasingly expensive and impractical. Many teams, especially L2 solutions, need to move data and calculations off-chain while maintaining security and verifiability. SubSMT addresses this need by providing:

- Efficient off-chain storage with on-chain verification
- Logarithmic time complexity (log(n)) for all operations
- Proof of both existence and non-existence of data
- A unified, production-ready solution for the Polkadot ecosystem

## How SubSMT Works

### Core Components

1. **Backend Storage**
   - RocksDB implementation for traditional storage needs
   - ParityDB implementation for Substrate-native storage
   - Common backend interface for easy extension to other storage solutions

2. **On-chain Verification**
   - Substrate pallet for native chain integration
   - ink! smart contract for parachain deployment
   - Efficient proof verification with minimal gas usage

3. **API Layer**
   - RESTful API for easy integration
   - Core operations: update, get, remove, verify
   - Future root calculation for transaction planning

### Key Operations

1. **Updating Values**
```rust
// Example structure of an update
{
    "address": "account_address",
    "balance": "1000000000000",
    "nonce": 1,
    "prefix": "tree_name"
}
```

2. **Generating Proofs**
- Creates compact Merkle proofs for data verification
- Includes both value and path information
- Optimized for minimal proof size

3. **Verifying Proofs**
- On-chain verification through pallet or smart contract
- Efficient verification algorithm
- Support for batch proof verification

### Performance Characteristics

All core operations maintain logarithmic time complexity:

| Operation | Complexity |
|-----------|------------|
| Update    | O(log n)   |
| Get       | O(log n)   |
| Verify    | O(log n)   |
| Proof Gen | O(log n)   |

## Use Cases

1. **L2 Solutions**
   - Off-chain data storage with on-chain verification
   - State channel implementations
   - Optimistic rollup solutions

2. **DeFi Applications**
   - Account balance tracking
   - State management
   - Efficient proof generation for cross-chain operations

3. **Data Availability**
   - Storing large datasets off-chain
   - Maintaining data integrity
   - Efficient data verification

## Getting Started

### Installation

```bash
git clone https://github.com/farcloud-labs/subsmt.git
cd subsmt
git submodule update --init --recursive
```

### Basic Usage

1. Start the backend service:
```bash
docker-compose up
```

2. Access the API:
- RocksDB backend: http://localhost:8080/swagger-ui/#
- ParityDB backend: http://localhost:8081/swagger-ui/#

## Future Development

SubSMT continues to evolve with planned features including:

- Enhanced on-chain functionality beyond root verification
- Additional storage backend implementations
- EVM compatibility for broader ecosystem support
- Zero-knowledge proof integration possibilities

## Conclusion

SubSMT provides a robust, efficient, and production-ready solution for off-chain data storage with on-chain verification in the Polkadot ecosystem. Its modular design, comprehensive testing, and focus on performance make it an ideal choice for projects requiring secure off-chain data management.

For detailed implementation examples and API documentation, please refer to our [Readme](https://github.com/farcloud-labs/subsmt/blob/main/README.md) and [test guide m1](https://github.com/farcloud-labs/subsmt/blob/main/docs/test-guide-m1.md) and [test guide m2](https://github.com/farcloud-labs/subsmt/blob/main/docs/test-guide-m2.md)