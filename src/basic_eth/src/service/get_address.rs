use alloy::signers::{icp::IcpSigner, Signer};
use candid::Principal;

use crate::{create_derivation_path, get_caller_pricipal, get_ecdsa_key_name};

#[ic_cdk::update]
pub async fn get_address(principal: Option<Principal>) -> String {
    // If no principal is specified in call, attempt to use caller principal
    let principal = principal.unwrap_or_else(get_caller_pricipal);

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let derivation_path = create_derivation_path(&principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, None)
        .await
        .unwrap();

    let address = signer.address();
    address.to_string()
}
