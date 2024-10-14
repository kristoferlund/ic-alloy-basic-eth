use crate::{auth_guard, create_derivation_path, get_ecdsa_key_name, get_rpc_service};
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::icp::IcpSigner,
    transports::icp::IcpConfig,
};
use candid::Nat;

#[ic_cdk::update]
async fn send_eth(to_address: String, amount: Nat) -> Result<String, String> {
    // Calls to send_eth need to be authenticated
    auth_guard()?;

    // From address is the method caller
    let from_principal = ic_cdk::caller();

    // Make sure we have a correct to address
    let to_address = Address::parse_checksummed(to_address, None).map_err(|e| e.to_string())?;

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let derivation_path = create_derivation_path(&from_principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, None)
        .await
        .map_err(|e| e.to_string())?;

    // Setup provider
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    // Create transaction
    let transaction_request = TransactionRequest::default()
        .with_to(to_address)
        .with_value(U256::from_le_slice(amount.0.to_bytes_le().as_slice()));

    // Send transaction
    let result = provider.send_transaction(transaction_request.clone()).await;

    match result {
        Ok(pending_tx_builder) => {
            let tx_hash = *pending_tx_builder.tx_hash();
            let tx_response = provider.get_transaction_by_hash(tx_hash).await.unwrap();
            Ok(format!("{:?}", tx_response))
        }
        Err(e) => Err(e.to_string()),
    }
}
