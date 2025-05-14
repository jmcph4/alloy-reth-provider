use alloy_rpc_types_eth::Block as RpcBlock;
use reth_ethereum_primitives::TransactionSigned;
use reth_primitives::{Block, BlockBody};
use reth_primitives_traits::{RecoveredBlock, SealedBlock, SealedHeader};

pub fn rpc_block_to_recovered_block(block: RpcBlock) -> eyre::Result<RecoveredBlock<Block>> {
    let block = block.map_transactions(|tx| tx.into_inner().into());
    let block_body = BlockBody::<TransactionSigned> {
        transactions: block.transactions.into_transactions().collect(),
        ommers: vec![],
        withdrawals: block.withdrawals,
    };
    let sealed_header = SealedHeader::new(block.header.inner, block.header.hash);
    let sealed_block = SealedBlock::from_sealed_parts(sealed_header, block_body);
    let recovered_block = RecoveredBlock::try_recover_sealed(sealed_block)?;
    Ok(recovered_block)
}
