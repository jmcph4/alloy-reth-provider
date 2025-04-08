use crate::AlloyRethStateProvider;
use alloy_network::Network;
use alloy_primitives::B256;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
use reth_provider::StateRootProvider;
use reth_trie::updates::TrieUpdates;
use reth_trie::{HashedPostState, TrieInput};
use tracing::warn;

impl<N, P> StateRootProvider for AlloyRethStateProvider<N, P>
where
    N: Network,
    P: Provider<N> + Clone,
{
    fn state_root(&self, hashed_state: HashedPostState) -> ProviderResult<B256> {
        self.state_root_from_nodes(TrieInput::from_state(hashed_state))
    }

    fn state_root_from_nodes(&self, _input: TrieInput) -> ProviderResult<B256> {
        warn!("state_root_from_nodes is not implemented and will return zero");
        Ok(B256::ZERO)
    }

    fn state_root_with_updates(&self, hashed_state: HashedPostState) -> ProviderResult<(B256, TrieUpdates)> {
        self.state_root_from_nodes_with_updates(TrieInput::from_state(hashed_state))
    }

    fn state_root_from_nodes_with_updates(&self, _input: TrieInput) -> ProviderResult<(B256, TrieUpdates)> {
        warn!("state_root_from_nodes_with_updates is not implemented and will return zero");
        Ok((B256::ZERO, TrieUpdates::default()))
    }
}
