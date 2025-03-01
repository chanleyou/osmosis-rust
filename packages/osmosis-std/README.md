# osmosis-std

[![osmosis-std on crates.io](https://img.shields.io/crates/v/osmosis-std.svg)](https://crates.io/crates/osmosis-std) [![Docs](https://docs.rs/osmosis-std/badge.svg)](https://docs.rs/osmosis-std)

Osmosis's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.

## CosmWasm stargate message and stargate query

You can find all types and querier generated from osmosis's protobuf in their respective module in `osmosis_std`.

[Full working example contract can be found here.](https://github.com/osmosis-labs/osmosis-rust/tree/main/examples/cosmwasm/contracts/osmosis-stargate)

### Publishing Osmosis' message from CosmWasm Contract

```rust
use cosmwasm_std::{CosmosMsg, Response, Env};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

# type ContractError = cosmwasm_std::StdError;
// ..

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let sender = env.contract.address.into();

    // construct message and convet them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}

```

## Querying Osmosis' module

[This will not currently work on mainnet until osmosis v12.](https://github.com/osmosis-labs/osmosis/issues/2433).
But meanwhile, if you want to test this out, you can by using [osmosis hackatom-seoul branch](https://github.com/osmosis-labs/osmosis/tree/hackatom-seoul) which we enabled all stargate query for hacking purpose!

```sh
git clone git@github.com:osmosis-labs/osmosis.git
cd osmosis
git checkout remotes/origin/hackatom-seoul

# build and run localosmosis with hackatom-seoul branch (docker required)
make localnet-build && make localnet-start
```

```rust
use cosmwasm_std::{Deps, Env, StdResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{TokenfactoryQuerier, QueryDenomsFromCreatorResponse};

// ..

fn query_creator_denoms(deps: Deps, env: Env) -> StdResult<QueryDenomsFromCreatorResponse> {
    // create `TokenfactoryQuerier`
    let tokenfactory = TokenfactoryQuerier::new(&deps.querier);

    // `TokenfactoryQuerier` has all the fns for querying the module
    let res = tokenfactory.denoms_from_creator(env.contract.address.into())?;

    Ok(QueryDenomsFromCreatorResponse { denoms: res.denoms })
}
```

## Non-CosmWasm Client

(WIP)
