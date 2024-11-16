#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
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
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use primitives::sparse_merkle_tree::{
        traits::{Hasher, Value},
        H256,
    };
    use primitives::verify::{self, Proof};
    use scale_info::prelude::fmt::Debug;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;

        type SMTKey: Value + Default + Debug + Clone + TypeInfo + Encode + Decode + PartialEq;

        type SMTValue: Value + Default + Debug + Clone + TypeInfo + Encode + Decode + PartialEq;

        type SMTHasher: Hasher + Default;
        // type BeforeVerify:

        // type AfterVerify:
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
        SMTVerify {
            account: T::AccountId,
            path: H256,
            root: H256,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        SMTVerifyFaild,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
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
                ) == true,
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
