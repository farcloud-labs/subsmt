use frame_support::{derive_impl, parameter_types, traits::Everything};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use smt_backend_lib::apis::MultiSMTStore;
use std::path::Path;
use primitives::{keccak_hasher::Keccak256Hasher, kv::{SMTKey, SMTValue}, verify::Proof};
type Block = frame_system::mocking::MockBlock<Test>;


// 创建数据库并且获得10条证明
// 获取两条数据
pub fn creat_db_and_get_proof(size: u8) -> Vec<Proof<SMTKey, SMTValue>> {
    let base_path = "./smt_mock_db";
    let multi_tree =
            MultiSMTStore::<SMTKey, SMTValue, Keccak256Hasher>::open(Path::new(base_path)).unwrap();
    // 创建一个tree
    let tree = "tree1";
    multi_tree.clear(tree.as_ref());
    let mut kvs: Vec<(SMTKey, SMTValue)> = vec![];

    for i in 1..size {
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
        let p = multi_tree.get_merkle_proof(tree.as_ref(), kv.0.clone()).unwrap();
        println!("{:#?}", p.clone());
        assert_eq!(multi_tree.verify(p), true);
    }

    let mut proofs: Vec<Proof<SMTKey, SMTValue>> = vec![];
    for kv in kvs.clone() {
        let proof = multi_tree.get_merkle_proof(tree.as_ref(), kv.0.clone()).unwrap();
        proofs.push(proof);
    }

    proofs
    }


// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        TemplateModule: crate::{Pallet, Call, Storage, Event<T>},
    }
);


parameter_types! {
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type SMTHasher = Keccak256Hasher;
    type SMTKey = SMTKey;
    type SMTValue = SMTValue;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
