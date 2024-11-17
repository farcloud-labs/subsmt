use crate::{mock::*, Error, Something};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_smt_verify() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        let proofs = creat_db_and_get_proof(100 as u8);
        let who = RuntimeOrigin::signed(1);
        for p in proofs {
            assert_ok!(TemplateModule::smt_verify(who.clone(), p));
        }
    });
}
