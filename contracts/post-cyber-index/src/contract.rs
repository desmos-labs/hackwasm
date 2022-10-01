#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, MessageInfo, Response, Timestamp,
};
use cw2::set_contract_version;
use cyber_std::Link;
use desmos_bindings::posts::querier::PostsQuerier;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, Packet, QueryMsg};
use crate::state::{State, STATE};
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
        subspace_id: msg.subspace_id.into(),
        cyber_contract_address: msg.cyber_contract_address,
        root_hash: msg.root_hash,
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
        ExecuteMsg::CyberIndexPost { post_id } => {
            create_post_cyber_link(deps, env.block.time, post_id.into())
        }
    }
}

fn create_post_cyber_link(
    deps: DepsMut,
    time: Timestamp,
    post_id: u64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let links = get_cyber_links_from_post(deps, state.root_hash, state.subspace_id, post_id)?;
    // TODO: set the right channel id
    let msg = IbcMsg::SendPacket {
        channel_id: "mock-channel-id".into(),
        data: to_binary(&Packet { links })?,
        timeout: time.plus_seconds(PACKET_LIFETIME).into(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "create_post_cyber_link"))
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
