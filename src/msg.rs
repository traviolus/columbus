use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::{Island, State};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    DiscoverIsland { name: String },
    GatherResources { island_name: String },
    Mint { island_name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Island)]
    GetIsland { founder: String, name: String },
    #[returns(State)]
    GetState {},
}

#[cw_serde]
pub struct MigrateMsg {}
