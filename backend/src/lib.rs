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

//! Provide APIs related to SMT (Sparse Merkle Tree).  
//! Implement persistent storage for SMT.  
//! A single database can store multiple Merkle trees, and they do not interfere with each other.

pub mod error;
pub mod cli;
pub mod parity;
pub mod rocks;
pub use rocks::*;
pub use parity::*;