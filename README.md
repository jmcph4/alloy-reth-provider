# Alloy Reth State DB provider
To test reth ExEx, it can be useful to not be dependent on a real database. This repository illustrates how the reth traits for the state can be implemented to forward the request to an RPC.

## Remarks
There is no `debug_codeByHash` implemented currently (see [feat(rpc): debug_codeByHash #14479](https://github.com/paradigmxyz/reth/issues/14479)). But since revm will first call `basic_account`, we can cache it and return it when `bytecode_by_hash` is called.

Currently, this repos purpose as copy/paste reference and not as library. Not all functions are implemented.

## Acknowledgements
Many, many thanks to the team of [reth](https://github.com/paradigmxyz/reth) and [alloy-rs](https://github.com/alloy-rs/alloy) for the awesome work they do. Some parts of the trait implementation are taken from reth. Also, many thanks to [revm](https://github.com/bluealloy/revm). The `alloy_db` part is a copy/paste from revm because this part is not included in the latest revm version, and it makes it easier to be in sync with the latest Alloy version.

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT). The part in `alloy_db` is licensed as [revm](https://github.com/bluealloy/revm) only under [MIT](./LICENSE-MIT).