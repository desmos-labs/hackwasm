use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, IbcEndpoint};
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub cyber_contract_address: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct ChannelInfo {
    pub id: String,
    pub counterparty_endpoint: IbcEndpoint,
    pub connection_id: String,
}

pub const CHANNEL_INFO: Item<ChannelInfo> = Item::new("channel_info");
