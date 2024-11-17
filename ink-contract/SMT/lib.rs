#![cfg_attr(not(feature = "std"), no_std, no_main)]

// 如何表达事件？？？
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

    // impl Default for Smt {
    //     fn default() -> Self {
    //         Self::new()
    //     }
    // }

    impl Smt {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
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
            )
            {
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
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let smt = Smt::default();
            assert_eq!(smt.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut smt = Smt::new(false);
            assert_eq!(smt.get(), false);
            smt.flip();
            assert_eq!(smt.get(), true);
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
