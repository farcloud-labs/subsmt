
# Test Guide
"This project has undergone comprehensive testing and is suitable for production use."

## Unit test

### Test code link
- https://github.com/farcloud-labs/subsmt/blob/main/backend/src/apis.rs#L188
- https://github.com/farcloud-labs/subsmt/blob/main/backend/src/store.rs#L114
- https://github.com/farcloud-labs/subsmt/blob/main/pallet/SMT/src/tests.rs
- https://github.com/farcloud-labs/subsmt/blob/main/ink-contract/SMT/lib.rs#L74
- https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/keccak_hasher.rs#L52
- https://github.com/farcloud-labs/subsmt/blob/main/primitives/src/kv.rs#L178
### install  Rust environment

[https://docs.substrate.io/install/](https://docs.substrate.io/install/)
> "Follow this document to set up your Rust environment."
### clone project
```
git clone git clone https://github.com/farcloud-labs/subsmt.git

cd subsmt 
git submodule update --init --recursive
```
### test

```
cargo test -- --nocapture

```
### Obtain test coverage report

```
cargo install cargo-tarpaulin
```

Generate test report.

```
cargo tarpaulin --out Html --output-dir ./docs --exclude-files ./sparse-merkle-tree/*

```
> [test report](./tarpaulin-report.html)
## 手动测试
### 启动docker
```
docker-compose up
```

### swagger-ui测试

### 链上测试


