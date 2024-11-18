#![allow(unused_imports)]
use crate::{mock::*, Error};
use frame_support::{assert_err, assert_noop, assert_ok};
use primitives::verify::Proof;

#[test]
fn it_works_for_smt_verify() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        let who = RuntimeOrigin::signed(1);
        let proofs = creat_db_and_get_proof(100 as u8);
        assert_err!(
            TemplateModule::smt_verify(who.clone(), proofs[0].clone()),
            Error::<Test>::SMTVerifyFaild
        );
        proofs[1..].iter().for_each(|p| {
            assert_ok!(TemplateModule::smt_verify(who.clone(), p.clone()));
        });
    });
}
