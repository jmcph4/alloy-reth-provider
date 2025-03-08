use crate::AlloyRethProvider;
use alloy_consensus::transaction::TransactionMeta;
use alloy_eips::BlockHashOrNumber;
use alloy_network::Network;
use alloy_primitives::{Address, BlockNumber, TxHash, TxNumber};
use alloy_provider::Provider;
use reth_errors::ProviderResult;
pub(crate) use reth_provider::TransactionsProvider;
use std::ops::RangeBounds;

impl<N, P> TransactionsProvider for AlloyRethProvider<N, P>
where
    N: Network,
    P: 'static + Clone + Provider<N> + Send + Sync,
{
    type Transaction = reth_primitives::TransactionSigned;

    fn transaction_id(&self, _tx_hash: TxHash) -> ProviderResult<Option<TxNumber>> {
        todo!()
    }

    fn transaction_by_id(&self, _id: TxNumber) -> ProviderResult<Option<Self::Transaction>> {
        todo!()
    }

    fn transaction_by_id_unhashed(&self, _id: TxNumber) -> ProviderResult<Option<Self::Transaction>> {
        todo!()
    }

    fn transaction_by_hash(&self, _hash: TxHash) -> ProviderResult<Option<Self::Transaction>> {
        todo!()
    }

    fn transaction_by_hash_with_meta(&self, _hash: TxHash) -> ProviderResult<Option<(Self::Transaction, TransactionMeta)>> {
        todo!()
    }

    fn transaction_block(&self, _id: TxNumber) -> ProviderResult<Option<BlockNumber>> {
        todo!()
    }

    fn transactions_by_block(&self, _block: BlockHashOrNumber) -> ProviderResult<Option<Vec<Self::Transaction>>> {
        todo!()
    }

    fn transactions_by_block_range(&self, _range: impl RangeBounds<BlockNumber>) -> ProviderResult<Vec<Vec<Self::Transaction>>> {
        todo!()
    }

    fn transactions_by_tx_range(&self, _range: impl RangeBounds<TxNumber>) -> ProviderResult<Vec<Self::Transaction>> {
        todo!()
    }

    fn senders_by_tx_range(&self, _range: impl RangeBounds<TxNumber>) -> ProviderResult<Vec<Address>> {
        todo!()
    }

    fn transaction_sender(&self, _id: TxNumber) -> ProviderResult<Option<Address>> {
        todo!()
    }
}
