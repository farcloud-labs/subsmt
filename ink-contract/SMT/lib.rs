#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unexpected_cfgs)]


#[ink::contract]
mod smt {
    use smt_primitives::{
        keccak_hasher::Keccak256Hasher,
        kv::{SMTKey, SMTValue},
        sparse_merkle_tree::H256,
        verify::{verify as smt_verify, Proof},
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Smt;

    #[ink(event)]
    pub struct SMTVerify {
        who: AccountId,
        #[ink(topic)]
        path: H256,
        #[ink(topic)]
        root: H256,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        SMTVerifyFaild,
    }


    impl Smt {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn smt_verify(&self, proof: Proof<SMTKey, SMTValue>) -> Result<()> {
            self.do_verify(proof)
        }

        fn do_verify(&self, proof: Proof<SMTKey, SMTValue>) -> Result<()> {
            let from = self.env().caller();
            if !smt_verify::<Keccak256Hasher>(
                proof.path,
                proof.value_hash,
                proof.leave_bitmap,
                proof.siblings,
                proof.root,
            ) {
                return Err(Error::SMTVerifyFaild);
            }
            Self::env().emit_event(SMTVerify {
                who: from,
                path: proof.path,
                root: proof.root,
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        #![allow(unused_imports)]
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use smt_backend_lib::apis::MultiSMTStore;
        use smt_primitives::{
            keccak_hasher::Keccak256Hasher,
            kv::{SMTKey, SMTValue},
            verify::Proof,
        };
        use std::path::Path;

        pub fn creat_db_and_get_proof(size: u8) -> Vec<Proof<SMTKey, SMTValue>> {
            let base_path = "./smt_ink_test_db";
            let multi_tree =
                MultiSMTStore::<SMTKey, SMTValue, Keccak256Hasher>::open(Path::new(base_path))
                    .unwrap();
            // 创建一个tree
            let tree = "tree1";
            multi_tree.clear(tree.as_ref());
            let mut kvs: Vec<(SMTKey, SMTValue)> = vec![];

            for i in 0..size {
                kvs.push((
                    SMTKey {
                        address: i.to_string(),
                    },
                    SMTValue {
                        nonce: i as u64,
                        balance: i as u128,
                    },
                ));
            }

            for kv in kvs.clone() {
                multi_tree
                    .update(tree.as_ref(), kv.0.clone(), kv.1.clone())
                    .unwrap();
            }

            let mut proofs: Vec<Proof<SMTKey, SMTValue>> = vec![];
            for kv in kvs.clone() {
                let proof = multi_tree
                    .get_merkle_proof(tree.as_ref(), kv.0.clone())
                    .unwrap();
                proofs.push(proof);
            }

            proofs
        }

        #[ink::test]
        fn smt_verify_works() {
            let smt = Smt::new();
            let proofs = creat_db_and_get_proof(2);
            assert_ne!(smt.smt_verify(proofs[0].clone()), Ok(()));
            proofs[1..].iter().for_each(|p| {
                // ;
                assert_eq!(smt.smt_verify(p.clone()), Ok(()));
            });
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = SmtRef::default();

            // When
            let contract = client
                .instantiate("smt", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Smt>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = SmtRef::new(false);
            let contract = client
                .instantiate("smt", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Smt>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
