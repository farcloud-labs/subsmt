//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use primitives::kv::{SMTKey, SMTValue};
use mock::creat_db_and_get_proof;

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
