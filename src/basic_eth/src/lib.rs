mod service;

use candid::{Nat, Principal};
use ic_cdk::export_candid;
use serde_bytes::ByteBuf;

// ICP uses different ECDSA key names for mainnet and local
// development.
fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "local" => "dfx_test_key".to_string(),
        "ic" => "key_1".to_string(),
        _ => panic!("Unsupported network."),
    }
}

// The derivation path determines the Ethereum address generated
// by the signer.
fn create_derivation_path(principal: &Principal) -> Vec<Vec<u8>> {
    const SCHEMA_V1: u8 = 1;
    [
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(principal.as_slice().to_vec()),
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect()
}

fn get_caller_pricipal() -> Principal {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("Calls with the anonymous principal are not allowed.");
    }
    principal
}

export_candid!();
