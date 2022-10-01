use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cyber_std::Link;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, IbcChannelOpenMsg, to_binary, Binary};
use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Packet {
    pub links: Vec<Link>,
}

