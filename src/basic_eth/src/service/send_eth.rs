use crate::{create_derivation_path, get_caller_pricipal, get_ecdsa_key_name};
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::icp::IcpSigner,
    transports::icp::{EthSepoliaService, IcpConfig, RpcService},
};
use candid::Nat;
use core::panic;

#[ic_cdk::update]
async fn send_eth(to_address: String, amount: Nat) -> String {
    // Calls to send_eth need to be authenticated
    let principal = get_caller_pricipal();

    let to_address =
        Address::parse_checksummed(to_address, None).unwrap_or_else(|_| panic!("Invalid address"));

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let derivation_path = create_derivation_path(&principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, None)
        .await
        .unwrap();

    // Setup provider
    let wallet = EthereumWallet::from(signer);
    let rpc_service = RpcService::EthSepolia(EthSepoliaService::Alchemy);
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
            format!("{:?}", tx_response)
        }
        Err(e) => format!("Error: {:?}", e),
    }
}
