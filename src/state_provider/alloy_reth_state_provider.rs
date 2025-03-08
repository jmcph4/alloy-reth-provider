use crate::alloy_db::{AlloyDBFork, WrapDatabaseAsync};
use alloy_eips::BlockId;
use alloy_network::Network;
use alloy_primitives::map::HashMap;
use alloy_primitives::B256;
use alloy_provider::Provider;
use parking_lot::RwLock;
use reth_primitives::Bytecode;
use std::marker::PhantomData;
use tokio::runtime::Runtime;

pub struct AlloyRethStateProvider<N: Network, P: Provider<N> + Clone> {
    rt: Option<Runtime>,
    pub(crate) alloy_db: WrapDatabaseAsync<AlloyDBFork<N, P>>,
    pub(crate) bytecode: RwLock<HashMap<B256, Bytecode>>,
    _n: PhantomData<N>,
}

impl<N: Network, P: Provider<N> + Clone> AlloyRethStateProvider<N, P> {
    pub fn new(provider: P, block_id: BlockId) -> Self {
        let rt = Runtime::new().unwrap();
        let alloy_db = AlloyDBFork::new(provider.clone(), block_id);
        let wrapped_db = WrapDatabaseAsync::with_handle(alloy_db, rt.handle().clone());
        Self { rt: Some(rt), alloy_db: wrapped_db, bytecode: RwLock::new(HashMap::default()), _n: PhantomData }
    }
}

impl<N: Network, P: Provider<N> + Clone> Drop for AlloyRethStateProvider<N, P> {
    fn drop(&mut self) {
        if let Some(runtime) = self.rt.take() {
            runtime.shutdown_background();
        }
    }
}
