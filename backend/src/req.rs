#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use utoipa::{IntoParams, ToSchema};
use utoipa::__dev::ComposeSchema;
use crate::kv::{SMTKey, SMTValue};
use serde;

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
