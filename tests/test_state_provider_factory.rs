mod utils;

use crate::utils::{mock_node_response_file, mock_node_response_json};
use alloy_eips::BlockId;
use alloy_primitives::address;
use alloy_provider::ProviderBuilder;
use alloy_reth_provider::AlloyRethProvider;
use reth_provider::{AccountReader, StateProviderFactory};
use ruint::__private::ruint_macro::uint;
use serde_json::json;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use wiremock::MockServer;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_state_provider_factory_state_by_block_id() {
    // Test with real provider
    let provider = ProviderBuilder::new().on_http("https://eth.merkle.io".parse().unwrap());
    let db_provider = AlloyRethProvider::new(provider);
    let state = db_provider.state_by_block_id(BlockId::number(16148323)).unwrap();
    let acc_info = state.basic_account(&address!("220866b1a2219f40e72f5c628b65d54268ca3a9d")).unwrap().unwrap();

    assert_eq!(acc_info.nonce, 1);
    assert_eq!(acc_info.balance, uint!(250001010477701567100010_U256));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_state_provider_factory_latest() {
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from("debug")).init();

    let mock_server = MockServer::start().await;
    mock_node_response_json(&mock_server, "eth_blockNumber", json!("0xf66763")).await;
    mock_node_response_file(&mock_server, "eth_getBlockByNumber", "fixtures/eth_getBlockByNumber_0xf66763.json").await;
    mock_node_response_json(
        &mock_server,
        "eth_getCode",
        json!(
            "0x608060405273ffffffffffffffffffffffffffffffffffffffff600054167fa619486e\
        0000000000000000000000000000000000000000000000000000000060003514156050578\
        060005260206000f35b3660008037600080366000845af43d6000803e6000811415607057\
        3d6000fd5b3d6000f3fea265627a7a72315820d8a00dc4fe6bf675a9d7416fc2d00bb3433\
        362aa8186b750f76c4027269667ff64736f6c634300050e0032"
        ),
    )
    .await;
    mock_node_response_json(&mock_server, "eth_getBalance", json!("0x34f094f9a3590f9c346a")).await;
    mock_node_response_json(&mock_server, "eth_getTransactionCount", json!("0x1")).await;

    let provider = ProviderBuilder::new().on_http(mock_server.uri().parse().unwrap());
    let db_provider = AlloyRethProvider::new(provider);
    let state = db_provider.latest().unwrap();
    let acc_info = state.basic_account(&address!("220866b1a2219f40e72f5c628b65d54268ca3a9d")).unwrap().unwrap();

    assert_eq!(acc_info.nonce, 1);
    assert_eq!(acc_info.balance, uint!(250001010477701567100010_U256));
}
