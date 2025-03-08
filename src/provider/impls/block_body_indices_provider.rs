use crate::AlloyRethProvider;
use alloy_network::Network;
use alloy_primitives::BlockNumber;
use alloy_provider::Provider;
use reth_db_models::StoredBlockBodyIndices;
use reth_errors::ProviderResult;
pub(crate) use reth_provider::BlockBodyIndicesProvider;
use std::ops::RangeInclusive;

impl<N, P> BlockBodyIndicesProvider for AlloyRethProvider<N, P>
where
    N: Network,
    P: 'static + Clone + Provider<N> + Send + Sync,
{
    fn block_body_indices(&self, _num: u64) -> ProviderResult<Option<StoredBlockBodyIndices>> {
        todo!()
    }

    fn block_body_indices_range(&self, _range: RangeInclusive<BlockNumber>) -> ProviderResult<Vec<StoredBlockBodyIndices>> {
        todo!()
    }
}
