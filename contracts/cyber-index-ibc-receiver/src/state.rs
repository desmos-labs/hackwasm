use cosmwasm_schema::cw_serde;
use cosmwasm_std::IbcEndpoint;
use cw_storage_plus::Item;

#[cw_serde]
pub struct ChannelInfo {
    pub id: String,
    pub counterparty_endpoint: IbcEndpoint,
    pub connection_id: String,
}

pub const CHANNEL_INFO: Item<ChannelInfo> = Item::new("channel_info");
