use alloy_network::Network;
use alloy_provider::Provider;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct AlloyRethProvider<N, P: Send + Sync + Clone + 'static> {
    pub(crate) provider: P,
    _n: PhantomData<N>,
}

impl<N, P> AlloyRethProvider<N, P>
where
    N: Network,
    P: Provider<N> + Send + Sync + Clone + 'static,
{
    pub fn new(provider: P) -> Self {
        Self { provider, _n: PhantomData }
    }
}
