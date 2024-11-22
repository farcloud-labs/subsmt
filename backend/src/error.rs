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
use actix_web::{
    cookie::time::util::weeks_in_year, get, post, web, App, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Unexpected error occurred")]
    UnexpectedError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Error::InternalError(e) => HttpResponse::BadRequest().body(e.to_string()),
            _ => HttpResponse::BadRequest().body("Unexpected error occurred"),
        }
    }
}
