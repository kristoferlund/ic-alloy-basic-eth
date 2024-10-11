use alloy::signers::{icp::IcpSigner, Signer};
use candid::Principal;

use crate::{create_derivation_path, get_ecdsa_key_name};

#[ic_cdk::update]
pub async fn get_address(principal: Option<Principal>) -> String {
    let principal = principal.unwrap_or_else(ic_cdk::caller);

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let chain_id = 11155111;
    let derivation_path = create_derivation_path(&principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, Some(chain_id))
        .await
        .unwrap();

    let address = signer.address();
    address.to_string()
}
