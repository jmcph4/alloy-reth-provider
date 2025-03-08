use crate::AlloyRethProvider;
use alloy_network::Network;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
pub(crate) use reth_provider::BlockIdReader;

impl<N, P> BlockIdReader for AlloyRethProvider<N, P>
where
    N: Network,
    P: Provider<N> + Send + Sync + Clone + 'static,
{
    fn pending_block_num_hash(&self) -> ProviderResult<Option<alloy_eips::BlockNumHash>> {
        todo!()
    }

    fn safe_block_num_hash(&self) -> ProviderResult<Option<alloy_eips::BlockNumHash>> {
        todo!()
    }

    fn finalized_block_num_hash(&self) -> ProviderResult<Option<alloy_eips::BlockNumHash>> {
        todo!()
    }
}
