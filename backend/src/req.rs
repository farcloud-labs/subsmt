use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use utoipa::{IntoParams, ToSchema};
use utoipa::__dev::ComposeSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqUpdate<K, V> {
    pub prefix: String,
    pub key: K,
    pub value: V,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByKey<K> {
    pub prefix: String,
    // #[serde(flatten)]
    pub key: K,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByKVs<K, V> {
    pub prefix: String,
    #[serde(flatten)]
    pub kvs: Vec<(K, V)>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReqByPrefix {
    pub prefix: String,
}
