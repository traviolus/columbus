use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Island {
    pub name: String,
    pub resources: u64,
    pub last_gather_time: u64,
    pub token_supply: Uint128,
}

#[cw_serde]
pub struct State {
    pub islands_discovered: u64,
    pub total_tokens_minted: Vec<Coin>,
}

pub const ISLANDS: Map<(&Addr, String), Island> = Map::new("islands");
pub const STATE: Item<State> = Item::new("state");
