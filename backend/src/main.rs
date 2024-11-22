// This file is part of farcloud-labs/subsmt.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An SMT backend based on Actix and Swagger-UI, providing RPC for external service calls.
//! A single database can create multiple Merkle trees.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use actix_web::middleware::Logger as ALogger;
// use serde_json;
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
use smt_backend_lib::{
    apis::MultiSMTStore,
    error::Error,
    req::{KVPair, ReqByKVs, ReqByKey, ReqByPrefix, ReqUpdate},
};
use std::env;
use dotenv::dotenv;
use smt_primitives::{
    keccak_hasher::Keccak256Hasher,
    kv::*,
    verify::{verify as smt_verify, Proof},
};
use sparse_merkle_tree::{traits::Value, H256};
use std::{future, path::Path, result::Result, sync::Mutex};
use thiserror::Error as ThisError;
use tokio::signal::ctrl_c;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_actix_web::AppExt;
use utoipa_redoc::Redoc;
use utoipa_swagger_ui::SwaggerUi;

const SMT_API: &str = "SMT API";

#[derive(OpenApi)]
#[openapi(
        tags(
            (name = "SMT API", description = "Provides sparse Morkel tree related APIs")
        ),
    )]
struct ApiDoc;

/// Insert a value into a specific Merkle tree.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Insert a value into a specific Merkle tree.", body = [H256])
    )
)]
#[post("/update")]
async fn update_value(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqUpdate<SMTKey, SMTValue>>,
) -> Result<HttpResponse, Error> {
    let mut multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let root = multi_tree
        .update(info.prefix.as_ref(), info.key.clone(), info.value.clone())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:#?}",
        format!("[Update] info: {:#?}, root: {:?}", info, root)
    );
    Ok(HttpResponse::Ok().json(root))
}

/// Remove a value by key
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Remove a value by key", body = [H256])
    )
)]
#[post("/remove")]
async fn remove_value(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByKey<SMTKey>>,
) -> Result<HttpResponse, Error> {
    let mut multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let root = multi_tree
        .update(info.prefix.as_ref(), info.key.clone(), Default::default())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:#?}",
        format!("[Remove] info: {:#?}, root: {:?}", info, root)
    );
    Ok(HttpResponse::Ok().json(root))
}

/// Get the Merkle proof.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Get the Merkle proof.", body = [Proof<SMTKey, SMTValue>])
    )
)]
#[post("/merkle_proof")]
async fn get_merkle_proof(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByKey<SMTKey>>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let proof = multi_tree
        .get_merkle_proof(info.prefix.as_ref(), info.key.clone())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:?}",
        format!("[Get Merkle Proof] info: {:?}, proof: {:?}", info, proof)
    );
    Ok(HttpResponse::Ok().json(proof))
}

/// Before data is updated, the future value of the root hash can be calculated in advance.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Before data is updated, the future value of the root hash can be calculated in advance.", body = [H256])
    )
)]
#[post("/next_root")]
async fn get_next_root(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByKVs<KVPair<SMTKey, SMTValue>>>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let old_proof = multi_tree
        .get_merkle_proof_old(info.prefix.as_ref(), vec![info.kv.key.clone()])
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let next_root = multi_tree
        .get_next_root(
            old_proof,
            vec![(info.kv.key.clone(), info.kv.value.clone())],
        )
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:?}",
        format!(
            "[Get Next Root] info: {:?}, next root: {:?}",
            info, next_root
        )
    );
    Ok(HttpResponse::Ok().json(next_root))
}

/// Get the root hash.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Get the root hash.", body = [H256])
    )
)]
#[post("/root")]
async fn get_root(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByPrefix>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let root = multi_tree
        .get_root(info.prefix.clone().to_string().as_ref())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:?}",
        format!(
            "[Get Root] info: {:?}, root: {:?}",
            info,
            serde_json::to_string(&root)
        )
    );
    Ok(HttpResponse::Ok().json(root))
}

/// Get the value of a specific key in a particular tree.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Get the value of a specific key in a particular tree.", body = [SMTValue])
    )
)]
#[post("/value")]
async fn get_value(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByKey<SMTKey>>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let value = multi_tree
        .get_value(info.prefix.as_ref(), info.key.clone())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!(
        "{:?}",
        format!("[Get Value] info: {:?}, value: {:?}", info, value)
    );
    Ok(HttpResponse::Ok().json(value))
}

/// Verify the Merkle proof.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Verify the Merkle proof.", body = [bool])
    )
)]
#[post("/verify")]
async fn verify(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<Proof<SMTKey, SMTValue>>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;
    let res = multi_tree.verify(Proof {
        key: info.key.clone(),
        value: info.value.clone(),
        path: info.key.to_h256(),
        value_hash: info.value.to_h256(),
        leave_bitmap: info.leave_bitmap,
        siblings: info.siblings.clone(),
        root: info.root,
    });
    log::info!("{:?}", format!("[Verify] info: {:?}, res: {:?}", info, res));
    Ok(HttpResponse::Ok().json(res))
}

/// Delete a specific Merkle tree.
#[utoipa::path(
    tag = SMT_API,
    params(
    ),
    responses(
        (status = 200, description = "Delete a specific Merkle tree.", body = [H256])
    )
)]
#[post("/clear")]
async fn clear(
    multi_tree: web::Data<Mutex<MultiSMTStore<SMTKey, SMTValue, Keccak256Hasher>>>,
    info: web::Json<ReqByPrefix>,
) -> Result<HttpResponse, Error> {
    let multi_tree = multi_tree
        .lock()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    multi_tree.clear(info.prefix.as_ref());
    let root = multi_tree
        .get_root(info.prefix.as_ref())
        .map_err(|e| Error::InternalError(e.to_string()))?;
    log::info!("{:?}", format!("[Clear] info: {:?}, res: {:?}", info, root));
    Ok(HttpResponse::Ok().json(root))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let base_path =env::var("DB_PATH").unwrap();
    let log_path =env::var("LOG_PATH").unwrap();
    let multi_tree = web::Data::new(Mutex::new(
        MultiSMTStore::<SMTKey, SMTValue, Keccak256Hasher>::open(Path::new(&base_path)).unwrap(),
    ));
    print!("log path: {:?}", log_path);

    // let l = async {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory(log_path))
        // .write_mode(WriteMode::BufferAndFlush)
        .rotate(
            flexi_logger::Criterion::Age(Age::Day),
            Naming::TimestampsDirect,
            Cleanup::Never,
        )
        .append()
        .log_to_stdout()
        .start()
        .unwrap();
    // std::future::pending::<()>().await;
    // };

    let app = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(update_value)
            .service(get_value)
            .service(get_merkle_proof)
            .service(get_next_root)
            .service(get_root)
            .service(verify)
            .service(remove_value)
            .service(clear)
            .app_data(multi_tree.clone())
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .shutdown_timeout(30)
    .bind(("0.0.0.0", 8080))?
    .run();

    let graceful_shutdown = async {
        ctrl_c().await.expect("Failed to listen for event");
        println!("Received CTRL-C, shutting down gracefully...");
    };

    let graceful_shutdown_task = tokio::spawn(graceful_shutdown);

    let result = tokio::select! {
        _ = app => Ok(()),
        _ = graceful_shutdown_task => Ok(()),
        // _ = l => Ok(()),
    };

    result
}
