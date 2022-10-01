#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use cw2::set_contract_version;
use desmos_bindings::posts::querier::PostsQuerier;
use cyber_std::Link;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:post-cyber-index";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
        channel_id: msg.channel_id,
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CyberIndexPost { post_id } => create_post_cyber_link(deps, post_id.into()),
    }
}

fn create_post_cyber_link(deps: DepsMut, post_id: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let links = get_cyber_links_from_post(deps, state.root_hash, state.subspace_id, post_id)?;
    Ok(Response::new())
}

fn get_cyber_links_from_post(deps: DepsMut, root_hash: String, subspace_id: u64, post_id: u64) -> Result<Vec<Link>, ContractError> {
    let post = PostsQuerier::new(deps.querier.deref()).query_post(subspace_id, post_id)?.post;
    let text = post.text.ok_or(ContractError::EmptyContent{})?;
    Ok(vec![Link{
        from: root_hash,
        to: text, 
    }])
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}
