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

//! This template will be responsible for verifying Merkle tree proofs on-chain. We follow the principle of minimizing on-chain computation and storage resources as much as possible, providing only the verification method.  
//! The advantage of Merkle trees lies precisely in this approach.  
//! The Merkle tree proof is provided by the `merkle_proof` API from the SMT backend.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use core::convert::TryInto;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use primitives::{
        sparse_merkle_tree::{
            traits::{Hasher, Value},
            H256,
        },
        verify::{self, Proof},
    };
    use scale_info::prelude::fmt::Debug;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;
        /// The data type of the Key in the KVDB.
        type SMTKey: Value + Default + Debug + Clone + TypeInfo + Encode + Decode + PartialEq;
        /// The data type of the value in the KVDB.
        type SMTValue: Value + Default + Debug + Clone + TypeInfo + Encode + Decode + PartialEq;
        /// The hash algorithm chosen for this Merkle tree off-chain.
        type SMTHasher: Hasher + Default;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Merkle proof verification passed.
        SMTVerify {
            /// Who submitted the proof to the blockchain.
            account: T::AccountId,
            /// The path of the leaf being proven.
            path: H256,
            /// root hash
            root: H256,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Merkle proof verification failed.
        SMTVerifyFaild,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Verify the Merkle proof provided off-chain.
        ///
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::smt_verify().saturating_mul(1_u64 + proof.siblings.len() as u64))]
        pub fn smt_verify(
            origin: OriginFor<T>,
            proof: verify::Proof<T::SMTKey, T::SMTValue>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::do_verify(who, proof)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn do_verify(
            who: T::AccountId,
            proof: Proof<T::SMTKey, T::SMTValue>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                verify::verify::<T::SMTHasher>(
                    proof.path,
                    proof.value_hash,
                    proof.leave_bitmap,
                    proof.siblings,
                    proof.root,
                ),
                Error::<T>::SMTVerifyFaild
            );
            Self::deposit_event(Event::<T>::SMTVerify {
                account: who,
                path: proof.path,
                root: proof.root,
            });

            Ok(().into())
        }
    }
}
