use actix_web::middleware::Logger as ALogger;
use actix_web::{
    cookie::time::util::weeks_in_year, get, post, web, App, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use codec::{Decode, Encode};
use ethers::utils::keccak256;
use flexi_logger::{Age, Cleanup, Criterion, Logger, Naming, WriteMode};
use http::status::{InvalidStatusCode, StatusCode};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use smt_primitives::{
    keccak_hasher::Keccak256Hasher,
    verify::{verify as smt_verify, Proof},
};
use serde_with::{DisplayFromStr};
use sparse_merkle_tree::{traits::Value, H256};
use thiserror::Error as ThisError;
use tokio::signal::ctrl_c;
use utoipa::{IntoParams, ToSchema};


#[serde_as]
#[derive(Encode, Decode, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, ToSchema, IntoParams)]
pub struct SMTValue {
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u128,
}


#[serde_as]
#[derive(Encode, Decode, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, ToSchema, IntoParams)]
pub struct SMTKey {
    #[serde_as(as = "DisplayFromStr")]
    pub user_id: u64,
}

impl Value for SMTKey {
    fn zero() -> Self {
        SMTKey::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        if self == &Default::default() {
            return H256::zero();
        }
        keccak256(self.encode()).into()
    }
}

impl Value for SMTValue {
    fn zero() -> Self {
        Default::default()
    }

    fn to_h256(&self) -> sparse_merkle_tree::H256 {
        if self == &Default::default() {
            return H256::zero();
        }
        return keccak256(self.encode()).into();
    }
}

impl Into<Vec<u8>> for SMTValue {
    fn into(self) -> Vec<u8> {
        self.encode()
    }
}

impl From<Vec<u8>> for SMTValue {
    fn from(value: Vec<u8>) -> Self {
        let a: SMTValue = Decode::decode::<&[u8]>(&mut value.as_slice()).unwrap_or_default();
        a
    }
}

