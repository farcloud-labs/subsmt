use rocksdb::{OptimisticTransactionDB, SingleThreaded, WriteBatchWithTransaction, WriteOptions};
fn main() {
    // println!("Hello, world!");
    let db: OptimisticTransactionDB<SingleThreaded> = OptimisticTransactionDB::open_default("db/rocksdb").unwrap();
    let mut txn = db.transaction();
    let mut snap = db.snapshot();
    db.get(b"key1").unwrap();
    db.delete(b"key1").unwrap();
    db.flush().unwrap();
    // db.write(batch)
    txn.put(b"key2", b"value2").unwrap();
    txn.commit().unwrap();
}
