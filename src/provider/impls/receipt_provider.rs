use crate::AlloyRethProvider;
use alloy_eips::BlockHashOrNumber;
use alloy_network::Network;
use alloy_primitives::{TxHash, TxNumber};
use alloy_provider::Provider;
use reth_errors::ProviderResult;
use reth_primitives::Receipt;
pub(crate) use reth_provider::ReceiptProvider;
use std::ops::RangeBounds;

impl<N, P> ReceiptProvider for AlloyRethProvider<N, P>
where
    N: Network,
    P: 'static + Clone + Provider<N> + Send + Sync,
{
    type Receipt = reth_primitives::Receipt;

    fn receipt(&self, _id: TxNumber) -> ProviderResult<Option<Receipt>> {
        todo!()
    }

    fn receipt_by_hash(&self, _hash: TxHash) -> ProviderResult<Option<Receipt>> {
        todo!()
    }

    fn receipts_by_block(&self, _block: BlockHashOrNumber) -> ProviderResult<Option<Vec<Receipt>>> {
        todo!()
    }

    fn receipts_by_tx_range(&self, _range: impl RangeBounds<TxNumber>) -> ProviderResult<Vec<Receipt>> {
        todo!()
    }
}
