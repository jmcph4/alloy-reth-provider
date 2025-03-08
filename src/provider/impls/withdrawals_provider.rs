use crate::AlloyRethProvider;
use alloy_eips::eip4895::Withdrawals;
use alloy_eips::BlockHashOrNumber;
use alloy_network::Network;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
pub(crate) use reth_provider::WithdrawalsProvider;

impl<N, P> WithdrawalsProvider for AlloyRethProvider<N, P>
where
    N: Network,
    P: 'static + Clone + Provider<N> + Send + Sync,
{
    fn withdrawals_by_block(&self, _id: BlockHashOrNumber, _timestamp: u64) -> ProviderResult<Option<Withdrawals>> {
        todo!()
    }
}
