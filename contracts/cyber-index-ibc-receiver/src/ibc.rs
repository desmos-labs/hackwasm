use crate::error::ContractError;
use crate::state::{ChannelInfo, CHANNEL_INFO};
use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, from_binary, to_binary, Addr, Binary, Env, IbcBasicResponse, IbcChannel,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacket, IbcPacketReceiveMsg,
    IbcReceiveResponse, StdResult,
};
use cyber_std::{create_cyberlink_msg, CyberMsgWrapper, DepsMut, Link};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub enum Ack {
    Result(Binary),
    Error(String),
}

// create a serialized success message
fn ack_success() -> Binary {
    let res = Ack::Result(b"1".into());
    to_binary(&res).unwrap()
}

// create a serialized error message
fn ack_fail(err: String) -> Binary {
    let res = Ack::Error(err);
    to_binary(&res).unwrap()
}

pub const IBC_APP_VERSION: &str = "desmos-cyber-link-v0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    let channel = msg.channel();
    if channel.order != IbcOrder::Ordered {
        return Err(ContractError::OnlyOrderedChannel {});
    }
    if channel.version.as_str() != IBC_APP_VERSION {
        return Err(ContractError::InvalidIbcVersion {
            version: IBC_APP_VERSION.into(),
        });
    }
    if let Some(counter_version) = msg.counterparty_version() {
        if counter_version != IBC_APP_VERSION {
            return Err(ContractError::InvalidCounterPartyIbcVersion {
                version: IBC_APP_VERSION.into(),
            });
        }
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> StdResult<IbcBasicResponse> {
    let channel: IbcChannel = msg.into();
    let info = ChannelInfo {
        id: channel.endpoint.channel_id.clone(),
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
    };
    CHANNEL_INFO.save(deps.storage, &info)?;
    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_connect")
        .add_attribute("chain_id", channel.endpoint.channel_id))
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Packet {
    pub links: Vec<Link>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse<CyberMsgWrapper>, ContractError> {
    do_ibc_packet_receive(env.contract.address, &msg.packet).or_else(|err| {
        Ok(IbcReceiveResponse::new()
            .set_ack(ack_fail(err.to_string()))
            .add_attributes(vec![
                attr("action", "ibc_receive"),
                attr("success", "false"),
                attr("error", err.to_string()),
            ]))
    })
}

fn do_ibc_packet_receive(
    address: Addr,
    packet: &IbcPacket,
) -> Result<IbcReceiveResponse<CyberMsgWrapper>, ContractError> {
    let packet: Packet = from_binary(&packet.data)?;
    Ok(IbcReceiveResponse::new()
        .add_message(create_cyberlink_msg(address.into(), packet.links))
        .add_attribute("action", "ibc_recieve")
        .set_ack(ack_success()))
}
