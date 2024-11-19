# subsmt
Sparse Merkle tree implementation in ink and substrate. Provide permanent storage off-chain backend based on rocksdb and paritydb. 

It can be applied in scenarios where Sparse Merkle Trees are used to save on-chain storage or computation resources, such as airdrops, game reward claims, and more. It provides a complete web backend functionality and the option to choose custom [hash algorithms](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/keccak_hasher.rs). Developers only need to modify the specific [implementation of keys and values](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/kv.rs) in the kv database, allowing them to quickly set up a web backend tailored to their project, along with on-chain verification templates or smart contracts. Its design follows the principle of minimizing the abuse of on-chain resources, ensuring that only the final verification needs to be implemented on-chain.

## apis

1. update_value
2. get_value
3. remove_value
4. get_merkle_proof
5. verify
6. get_next_root
7. get_root
8. clear

## Tech stack
* rust
* python
* polkadot-sdk
* actix-web
* swagger-ui
* parity-scale-codec
* sha3
* kvdb-rocksdb
* [sparse-merkle-tree](https://github.com/nervosnetwork/sparse-merkle-tree)

***

## **DEV**

Clone the project
```
git clone https://github.com/farcloud-labs/subsmt.git
```


```
cd subsmt 
submodule update --init --recursive.
```
### build


```
cargo build --release
```

### run

```
cargo run
```
or 
```
./target/release/smt
```

### Testing

#### Manual Testing
[http://localhost:8080/swagger-ui/](http://localhost:8080/swagger-ui/)

![swagger-ui](./docs/images/swagger.jpg)

#### Unit Test Case

```
cargo test
```

## docker
todo

## License
This project is licensed under the LICENSE_APACHE2. See the [LICENSE](./LICENSE) file for details.




