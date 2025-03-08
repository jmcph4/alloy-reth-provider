use alloy_eips::BlockId;
use alloy_primitives::utils::parse_units;
use alloy_primitives::{address, Bytes, B256, U256};
use alloy_provider::ProviderBuilder;
use alloy_reth_provider::AlloyRethProvider;
use reth_provider::StateProviderFactory;
use reth_revm::database::StateProviderDatabase;
use revm::primitives::{BlockEnv, Env, ResultAndState, SpecId, TransactTo, TxEnv};
use revm::Evm;
use std::str::FromStr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http("https://eth.merkle.io".parse()?);
    let db_provider = AlloyRethProvider::new(provider);
    // Top of block state previous block
    let state_provider = db_provider.state_by_block_id(BlockId::number(16148322))?;
    let state_db = StateProviderDatabase::new(state_provider);

    let mut env = Env {
        block: BlockEnv {
            // next block
            number: U256::from(16148323),
            timestamp: U256::from(1670565947),
            basefee: parse_units("176.658583385", "gwei")?.get_absolute(),
            ..BlockEnv::default()
        },
        ..Env::default()
    };
    // Simulate 0x2bb4f9fd58b3cd99591737eca784d0cf281034661bbdc509b58cf6a499d3d998
    env.tx = TxEnv {
        caller: address!("0x80d3aB7f834f786CAb110Bea9E8A96d45B2dc9C2"),
        gas_limit: 55_984,
        gas_price: parse_units("241.400106299", "gwei")?.get_absolute(),
        gas_priority_fee: Some(parse_units("1.5", "gwei")?.get_absolute()),
        transact_to: TransactTo::Call(address!("0x8686525d6627A25C68De82c228448f43c97999F2")),
        data: Bytes::from_str(
            "0x095ea7b3\
        00000000000000000000000068b3465833fb72a70ecdf485e0e4c7bd8665fc45\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )?,
        nonce: Some(401),
        chain_id: Some(1),
        ..TxEnv::default()
    };

    let mut evm = Evm::builder().with_spec_id(SpecId::SHANGHAI).with_ref_db(state_db).with_env(Box::new(env)).build();
    let ResultAndState { result, state } = evm.transact()?;

    println!("Success: {}", result.is_success());
    let slot = state
        .get(&address!("0x8686525d6627A25C68De82c228448f43c97999F2"))
        .unwrap()
        .storage
        .get(&U256::from_str("0xd3e71b600a79d9f8f28b05f66aae715ed55392acd344d20f75999f38afc352d5")?)
        .unwrap();
    println!("Storage (before): {:?}", B256::from(slot.original_value));
    println!("Storage (after) : {:?}", B256::from(slot.present_value));

    Ok(())
}
