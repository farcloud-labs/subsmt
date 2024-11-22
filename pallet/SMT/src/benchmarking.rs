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

#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use mock::creat_db_and_get_proof;
use primitives::kv::{SMTKey, SMTValue};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn smt_verify() {
        let proof = creat_db_and_get_proof(3);
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        smt_verify(RawOrigin::Signed(caller), p);
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
