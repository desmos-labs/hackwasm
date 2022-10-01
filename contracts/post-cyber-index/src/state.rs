use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub subspace_id: u64,
    pub cyber_contract_address: Addr,
    pub channel_id: String,
    pub root_hash: String,
}

pub const STATE: Item<State> = Item::new("state");
