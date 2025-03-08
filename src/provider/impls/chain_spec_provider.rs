use crate::AlloyRethProvider;
use alloy_network::Network;
use alloy_provider::Provider;
use reth_chainspec::{ChainSpec, ChainSpecProvider, HOLESKY, MAINNET, SEPOLIA};
use reth_optimism_chainspec::{BASE_MAINNET, BASE_SEPOLIA, OP_MAINNET, OP_SEPOLIA};
use std::sync::Arc;
use tokio::runtime::Handle;

impl<N, P> ChainSpecProvider for AlloyRethProvider<N, P>
where
    N: Network,
    P: Provider<N> + Send + Sync + Clone + 'static,
{
    type ChainSpec = ChainSpec;

    fn chain_spec(&self) -> Arc<ChainSpec> {
        let chain_id = tokio::task::block_in_place(move || Handle::current().block_on(self.provider.get_chain_id())).unwrap_or(1);
        match chain_id {
            // Ethereum
            1 => MAINNET.clone(),
            17000 => HOLESKY.clone(),
            11155111 => SEPOLIA.clone(),
            // Optimism
            10 => Arc::from(OP_MAINNET.inner.clone()),
            11155420 => Arc::from(OP_SEPOLIA.inner.clone()),
            // Base
            8453 => Arc::from(BASE_MAINNET.inner.clone()),
            84532 => Arc::from(BASE_SEPOLIA.inner.clone()),

            _ => unimplemented!("Unknown chain id: {}", chain_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_provider::ProviderBuilder;
    use reth_chainspec::EthChainSpec;
    use reth_provider::ChainSpecProvider;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_chain_spec_eth() {
        let node_url = env::var("MAINNET_HTTP").unwrap_or("https://eth.merkle.io".to_string());
        let provider = ProviderBuilder::new().on_http(node_url.parse().unwrap());

        let db_provider = AlloyRethProvider::new(provider);
        let chain_spec = db_provider.chain_spec();

        assert_eq!(chain_spec.chain_id(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_chain_spec_base() {
        let node_url = env::var("BASE_HTTP").unwrap_or("https://base.merkle.io".to_string());
        let provider = ProviderBuilder::new().on_http(node_url.parse().unwrap());

        let db_provider = AlloyRethProvider::new(provider);
        let chain_spec = db_provider.chain_spec();

        assert_eq!(chain_spec.chain_id(), 8453);
    }
}
