#![allow(unused_imports)]
use serde::{self, Deserialize, Serialize};
use smt_primitives::kv::{SMTKey, SMTValue};
use std::{fmt::Debug, marker::PhantomData};
use utoipa::{IntoParams, ToSchema, __dev::ComposeSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqUpdate<K, V> {
    pub prefix: String,
    #[serde(flatten)]
    pub key: K,
    #[serde(flatten)]
    pub value: V,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByKey<K> {
    pub prefix: String,
    #[serde(flatten)]
    pub key: K,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByKVs<KVPair> {
    pub prefix: String,
    #[serde(flatten)]
    pub kv: KVPair,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct KVPair<K, V> {
    #[serde(flatten)]
    pub key: K,
    #[serde(flatten)]
    pub value: V,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByPrefix {
    pub prefix: String,
}
