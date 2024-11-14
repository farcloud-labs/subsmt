use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use utoipa::{IntoParams, ToSchema};
use utoipa::__dev::ComposeSchema;
use crate::kv::{SMTKey, SMTValue};
use serde;

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqUpdate {
    pub prefix: String,
    #[serde(flatten)]
    pub key: SMTKey,
    #[serde(flatten)]
    pub value: SMTValue,
}



#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByKey {
    pub prefix: String,
    #[serde(flatten)]
    pub key: SMTKey,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByKVs {
    pub prefix: String,
    #[serde(flatten)]
    pub kv: KVPair,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct KVPair {
    #[serde(flatten)]
    pub key: SMTKey,
    #[serde(flatten)]
    pub value: SMTValue,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByPrefix {
    pub prefix: String,
}
