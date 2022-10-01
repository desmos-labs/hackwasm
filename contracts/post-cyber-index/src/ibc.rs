use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketTimeoutMsg, StdResult,
};

pub const VERSION: &str = "post-cyber-link";
pub const ORDERING: IbcOrder = IbcOrder::Unordered;

#[cw_serde]
pub enum Ack {
    Result(Binary),
    Error(String),
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
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();
    let channel_id = &channel.endpoint.channel_id;
    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_connect")
        .add_attribute("channel_id", channel_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let cyber_ack: Ack = from_binary(&msg.acknowledgement.data)?;
    match cyber_ack {
        Ack::Result(_) => on_packet_success(deps, msg.original_packet),
        Ack::Error(err) => on_packet_failure(deps, msg.original_packet, err),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    on_packet_failure(deps, msg.packet, "timeout".to_string())
}

fn on_packet_success(
    _deps: DepsMut,
    _packet: IbcPacket,
) -> Result<IbcBasicResponse, ContractError> {
    // do nothing and set events only
    let attributes = vec![attr("action", "acknowledge"), attr("success", "true")];
    Ok(IbcBasicResponse::new().add_attributes(attributes))
}

fn on_packet_failure(
    _deps: DepsMut,
    _packet: IbcPacket,
    err: String,
) -> Result<IbcBasicResponse, ContractError> {
    // do nothing and set events only
    let res = IbcBasicResponse::new()
        .add_attribute("action", "acknowledge")
        .add_attribute("success", "false")
        .add_attribute("error", err);
    Ok(res)
}
