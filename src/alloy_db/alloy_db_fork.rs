// FORK FROM: https://github.com/bluealloy/revm/blob/4e24be4/crates/database/src/alloydb.rs

use crate::alloy_db::async_db::DatabaseAsyncRef;
pub use alloy_eips::BlockId;
use alloy_network::primitives::HeaderResponse;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::network::primitives::BlockTransactionsKind;
use alloy_provider::network::BlockResponse;
use alloy_provider::{Network, Provider};
use alloy_transport::TransportError;
use revm::primitives::{AccountInfo, Bytecode};

/// An alloy-powered REVM [revm::Database].
///
/// When accessing the database, it'll use the given provider to fetch the corresponding account's data.
#[derive(Debug)]
pub struct AlloyDBFork<N: Network, P: Provider<N>> {
    /// The provider to fetch the data from.
    provider: P,
    /// The block number on which the queries will be based on.
    block_number: BlockId,
    _n: std::marker::PhantomData<N>,
}

impl<N: Network, P: Provider<N>> AlloyDBFork<N, P> {
    /// Create a new AlloyDB instance, with a [Provider] and a block.
    pub fn new(provider: P, block_number: BlockId) -> Self {
        Self { provider, block_number, _n: std::marker::PhantomData }
    }

    /// Set the block number on which the queries will be based on.
    pub fn set_block_number(&mut self, block_number: BlockId) {
        self.block_number = block_number;
    }
}

impl<N: Network, P: Provider<N>> DatabaseAsyncRef for AlloyDBFork<N, P> {
    type Error = TransportError;

    async fn basic_async_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let nonce = self.provider.get_transaction_count(address).block_id(self.block_number);
        let balance = self.provider.get_balance(address).block_id(self.block_number);
        let code = self.provider.get_code_at(address).block_id(self.block_number);

        let (nonce, balance, code) = tokio::join!(nonce, balance, code,);

        let balance = balance?;
        let code = Bytecode::new_raw(code?.0.into());
        let code_hash = code.hash_slow();
        let nonce = nonce?;

        Ok(Some(AccountInfo::new(balance, nonce, code_hash, code)))
    }

    async fn block_hash_async_ref(&self, number: u64) -> Result<B256, Self::Error> {
        let block = self
            .provider
            // SAFETY: We know number <= u64::MAX, so we can safely convert it to u64
            .get_block_by_number(number.into(), BlockTransactionsKind::Hashes)
            .await?;
        // SAFETY: If the number is given, the block is supposed to be finalized, so unwrapping is safe.
        Ok(B256::new(*block.unwrap().header().hash()))
    }

    async fn code_by_hash_async_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        panic!("This should not be called, as the code is already loaded");
        // This is not needed, as the code is already loaded with basic_ref
    }

    async fn storage_async_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        self.provider.get_storage_at(address, index).block_id(self.block_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloy_db::async_db::WrapDatabaseAsync;
    use alloy_primitives::ruint::__private::ruint_macro::uint;
    use alloy_provider::ProviderBuilder;
    use revm::DatabaseRef;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn can_get_basic() {
        let client = ProviderBuilder::new().on_http("https://eth.merkle.io".parse().unwrap());
        let alloydb = AlloyDBFork::new(client, BlockId::from(16148323));
        let wrapped_alloydb = WrapDatabaseAsync::new(alloydb).unwrap();

        let address: Address = "0x220866b1a2219f40e72f5c628b65d54268ca3a9d".parse().unwrap();

        let acc_info = wrapped_alloydb.basic_ref(address).unwrap().unwrap();
        assert!(acc_info.exists());
        assert_eq!(acc_info.nonce, 1);
        assert_eq!(acc_info.balance, uint!(250001010477701567100010_U256));
    }
}
