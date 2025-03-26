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
