use crate::AlloyRethStateProvider;
use alloy_network::Network;
use alloy_primitives::{Address, B256};
use alloy_provider::Provider;
use reth_errors::ProviderResult;
use reth_provider::StorageRootProvider;
use reth_trie::{HashedStorage, StorageMultiProof, StorageProof};

impl<N, P> StorageRootProvider for AlloyRethStateProvider<N, P>
where
    N: Network,
    P: Provider<N> + Clone,
{
    fn storage_root(&self, _address: Address, _hashed_storage: HashedStorage) -> ProviderResult<B256> {
        todo!()
    }

    fn storage_proof(&self, _address: Address, _slot: B256, _hashed_storage: HashedStorage) -> ProviderResult<StorageProof> {
        todo!()
    }

    fn storage_multiproof(&self, _address: Address, _slots: &[B256], _hashed_storage: HashedStorage) -> ProviderResult<StorageMultiProof> {
        todo!()
    }
}
