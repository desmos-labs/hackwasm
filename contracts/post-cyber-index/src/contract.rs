#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, MessageInfo, Response, Timestamp,
};
use cw2::set_contract_version;
use desmos_bindings::posts::querier::PostsQuerier;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, Packet, QueryMsg, Link};
use crate::state::{State, CHANNEL_INFO, STATE};
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:post-cyber-index";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PACKET_LIFETIME: u64 = 60 * 60;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        cyber_contract_address: msg.cyber_contract_address,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CyberIndexPost { subspace_id, post_id, root_hash } => {
            create_post_cyber_link(deps, env.block.time, subspace_id.into(), post_id.into(), root_hash)
        }
    }
}

fn create_post_cyber_link(
    deps: DepsMut,
    time: Timestamp,
    subspace_id: u64,
    post_id: u64,
    root_hash: String,
) -> Result<Response, ContractError> {
    let channel_info = CHANNEL_INFO.load(deps.storage)?;
    let links = get_cyber_links_from_post(deps, root_hash, subspace_id, post_id)?;
    let msg = IbcMsg::SendPacket {
        channel_id: channel_info.id,
        data: to_binary(&Packet { links })?,
        timeout: time.plus_seconds(PACKET_LIFETIME).into(),
    };
    Ok(Response::new()
        .add_attribute("method", "create_post_cyber_link")
        .add_message(msg))
}

fn get_cyber_links_from_post(
    deps: DepsMut,
    root_hash: String,
    subspace_id: u64,
    post_id: u64,
) -> Result<Vec<Link>, ContractError> {
    let post = PostsQuerier::new(deps.querier.deref())
        .query_post(subspace_id, post_id)?
        .post;
    let text = post.text.ok_or(ContractError::EmptyContent {})?;
    Ok(vec![Link {
        from: root_hash,
        to: text,
    }])
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}
