use alloy_eips::BlockId;
use alloy_node_bindings::Anvil;
use alloy_primitives::{address, uint};
use alloy_provider::ProviderBuilder;
use alloy_reth_provider::AlloyRethProvider;
use reth_provider::{AccountReader, StateProviderFactory};
use std::env;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_state_provider_factory_state_by_block_id() {
    let node_url = env::var("MAINNET_HTTP").unwrap_or("https://eth.merkle.io".to_string());
    let provider = ProviderBuilder::new().on_http(node_url.parse().unwrap());

    let db_provider = AlloyRethProvider::new(provider);
    let state = db_provider.state_by_block_id(BlockId::number(16148323)).unwrap();
    let acc_info = state.basic_account(&address!("220866b1a2219f40e72f5c628b65d54268ca3a9d")).unwrap().unwrap();

    assert_eq!(acc_info.nonce, 1);
    assert_eq!(acc_info.balance, uint!(250001010477701567100010_U256));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_state_provider_factory_latest() {
    let node_url = env::var("MAINNET_HTTP").unwrap_or("https://eth.merkle.io".to_string());
    let anvil = Anvil::new().fork(node_url).fork_block_number(16148323).spawn();
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let db_provider = AlloyRethProvider::new(provider);
    let state = db_provider.latest().unwrap();
    let acc_info = state.basic_account(&address!("220866b1a2219f40e72f5c628b65d54268ca3a9d")).unwrap().unwrap();

    assert_eq!(acc_info.nonce, 1);
    assert_eq!(acc_info.balance, uint!(250001010477701567100010_U256));
}
