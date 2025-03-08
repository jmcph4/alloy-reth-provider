use crate::AlloyRethProvider;
use alloy_eips::BlockHashOrNumber;
use alloy_network::Network;
use alloy_provider::Provider;
use reth_errors::ProviderResult;
pub(crate) use reth_provider::OmmersProvider;

impl<N, P> OmmersProvider for AlloyRethProvider<N, P>
where
    N: Network<HeaderResponse = alloy_rpc_types_eth::Header>,
    P: 'static + Clone + Provider<N> + Send + Sync,
{
    fn ommers(&self, _id: BlockHashOrNumber) -> ProviderResult<Option<Vec<Self::Header>>> {
        todo!()
    }
}
