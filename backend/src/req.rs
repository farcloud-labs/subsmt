use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use utoipa::{IntoParams, ToSchema};
use utoipa::__dev::ComposeSchema;
use crate::kv::{SMTKey, SMTValue};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqUpdate {
    pub prefix: String,
    pub key: SMTKey,
    pub value: SMTValue,
}



#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByKey {
    pub prefix: String,
    pub key: SMTKey,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByKVs {
    pub prefix: String,
    pub kv: KVPair,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct KVPair {
    pub key: SMTKey,
    pub value: SMTValue,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ReqByPrefix {
    pub prefix: String,
}
