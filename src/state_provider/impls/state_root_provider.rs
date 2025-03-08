use crate::AlloyRethStateProvider;
use alloy_network::Network;
use alloy_primitives::B256;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
use reth_provider::StateRootProvider;
use reth_trie::updates::TrieUpdates;
use reth_trie::{HashedPostState, TrieInput};

impl<N, P> StateRootProvider for AlloyRethStateProvider<N, P>
where
    N: Network,
    P: Provider<N> + Clone,
{
    fn state_root(&self, _hashed_state: HashedPostState) -> ProviderResult<B256> {
        todo!()
    }

    fn state_root_from_nodes(&self, _input: TrieInput) -> ProviderResult<B256> {
        todo!()
    }

    fn state_root_with_updates(&self, _hashed_state: HashedPostState) -> ProviderResult<(B256, TrieUpdates)> {
        todo!()
    }

    fn state_root_from_nodes_with_updates(&self, _input: TrieInput) -> ProviderResult<(B256, TrieUpdates)> {
        todo!()
    }
}
