use crate::{create_derivation_path, get_ecdsa_key_name};
use alloy::{
    providers::{Provider, ProviderBuilder},
    signers::{icp::IcpSigner, Signer},
    transports::icp::{EthSepoliaService, IcpConfig, RpcService},
};
use candid::Principal;

#[ic_cdk::update]
async fn get_balance(principal: Option<Principal>) -> String {
    let principal = principal.unwrap_or_else(ic_cdk::caller);

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let chain_id = 11155111; // Sepolia
    let derivation_path = create_derivation_path(&principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, Some(chain_id))
        .await
        .unwrap();

    // Setup provider
    let rpc_service = RpcService::EthSepolia(EthSepoliaService::Alchemy);
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    // Get balance for signer address
    let address = signer.address();
    let result = provider.get_balance(address).await;

    match result {
        Ok(balance) => balance.to_string(),
        Err(e) => e.to_string(),
    }
}
