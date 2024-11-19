# subsmt
Sparse Merkle tree implementation in ink and substrate. Provide permanent storage off-chain backend based on rocksdb and paritydb.

It can be applied in scenarios where Sparse Merkle Trees are used to save on-chain storage or computation resources, such as airdrops, game reward claims, and more. It provides a complete web backend functionality and the option to choose custom [hash algorithms](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/keccak_hasher.rs). Developers only need to modify the specific [implementation of keys and values](https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/kv.rs) in the kv database, allowing them to quickly set up a web backend tailored to their project, along with on-chain verification templates or smart contracts. Its design follows the principle of minimizing the abuse of on-chain resources, ensuring that only the final verification needs to be implemented on-chain.

## apis

1. update_value
2. remove_value
3. get_merkle_proof
4. get_next_root
5. get_root
6. get_value
7. verify
8. clear

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




