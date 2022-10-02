use crate::error::ContractError;
use crate::state::{ChannelInfo, CHANNEL_INFO};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{entry_point,attr, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketTimeoutMsg, StdResult, IbcChannelCloseMsg, IbcPacketReceiveMsg, IbcReceiveResponse};

#[cw_serde]
pub enum Ack {
    Result(Binary),
    Error(String),
}

pub const IBC_APP_VERSION: &str = "desmos-cyber-link-v0";

#[entry_point]
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

#[entry_point]
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

#[entry_point]
/// On closed channel, simply delete the account from our local store
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();

    // remove the channel
    let channel_id = &channel.endpoint.channel_id;

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_close")
        .add_attribute("channel_id", channel_id))
}

#[entry_point]
/// never should be called as the other side never sends packets
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _packet: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    Ok(IbcReceiveResponse::new()
        .set_ack(b"{}")
        .add_attribute("action", "ibc_packet_ack"))
}

#[entry_point]
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

#[entry_point]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    on_packet_failure(deps, msg.packet, "ibc_timeout".to_string())
}

fn on_packet_success(
    _deps: DepsMut,
    _packet: IbcPacket,
) -> Result<IbcBasicResponse, ContractError> {
    // do nothing and send events only
    let attributes = vec![attr("action", "ibc_acknowledge"), attr("success", "true")];
    Ok(IbcBasicResponse::new().add_attributes(attributes))
}

fn on_packet_failure(
    _deps: DepsMut,
    _packet: IbcPacket,
    err: String,
) -> Result<IbcBasicResponse, ContractError> {
    // do nothing and send events only
    let res = IbcBasicResponse::new()
        .add_attribute("action", "ibc_acknowledge")
        .add_attribute("success", "false")
        .add_attribute("error", err);
    Ok(res)
}
