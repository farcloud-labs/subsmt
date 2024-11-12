use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqUpdate<K, V> {
    pub prefix: String,
    pub key: K,
    pub value: V,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqByKey<K> {
    pub prefix: String,
    #[serde(flatten)]
    pub key: K,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqByKVs<K, V> {
    pub prefix: String,
    pub keys: Vec<(K, V)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqByPrefix {
    pub prefix: String,
}
