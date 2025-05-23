use crate::primitives::AlloyRethNodePrimitives;
use crate::{AlloyNetwork, AlloyRethProvider};
use alloy_eips::BlockHashOrNumber;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
use reth_provider::OmmersProvider;
use std::fmt::Debug;

impl<P, NP> OmmersProvider for AlloyRethProvider<P, NP>
where
    P: 'static + Clone + Provider<AlloyNetwork> + Debug + Send + Sync,
    NP: AlloyRethNodePrimitives,
{
    fn ommers(&self, _id: BlockHashOrNumber) -> ProviderResult<Option<Vec<Self::Header>>> {
        todo!()
    }
}
