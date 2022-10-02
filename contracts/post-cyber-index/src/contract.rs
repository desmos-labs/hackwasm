#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, DepsMut, Empty, Env, IbcMsg, MessageInfo, Response, Timestamp};
use cw2::set_contract_version;
use desmos_bindings::posts::querier::PostsQuerier;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, Link, Packet};
use crate::state::CHANNEL_INFO;
use crate::particle::prepare_particle;
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:post-cyber-index";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKET_LIFETIME: u64 = 60 * 60;
const DESMOS_NAMESPACE: &str = "desmos";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
        ExecuteMsg::CyberIndexPost {
            subspace_id,
            post_id,
        } => create_post_cyber_link(
            deps,
            env.block.time,
            subspace_id.into(),
            post_id.into(),
        ),
    }
}

fn create_post_cyber_link(
    deps: DepsMut,
    time: Timestamp,
    subspace_id: u64,
    post_id: u64,
) -> Result<Response, ContractError> {
    let channel_info = CHANNEL_INFO.load(deps.storage)?;
    let msg = IbcMsg::SendPacket {
        channel_id: channel_info.id,
        data: to_binary(&Packet { links: get_cyber_links_from_post(deps, subspace_id, post_id)? })?,
        timeout: time.plus_seconds(PACKET_LIFETIME).into(),
    };
    Ok(Response::new()
        .add_attribute("method", "create_post_cyber_link")
        .add_message(msg))
}

fn get_cyber_links_from_post(
    deps: DepsMut,
    subspace_id: u64,
    post_id: u64,
) -> Result<Vec<Link>, ContractError> {
    let post = PostsQuerier::new(deps.querier.deref())
        .query_post(subspace_id, post_id)?
        .post;
    let mut links: Vec<Link> = vec![];
    // Add from desmos namespace to subspace cyber link 
    let subspace_cid = get_subspace_cid(subspace_id)?;
    links.push(Link{
        from: prepare_particle(DESMOS_NAMESPACE.into())?.to_string(),
        to: subspace_cid.clone(),
    });
    // Add from tags to post uri cyber link
    let post_uri_cid = get_post_uri_cid(subspace_id, post_id)?;
    for tag in post.tags {
        links.push(Link {
            from: prepare_particle(tag)?.to_string(),
            to: post_uri_cid.clone(),
        });
    }
    // Add from subspace to post uri cyber link 
    links.push(Link {
        from: subspace_cid,
        to: post_uri_cid,
    });
    Ok(links)
}

fn get_subspace_cid(subspace_id: u64) -> Result<String, ContractError> {
    Ok(prepare_particle(format!("desmos-subspace-{}", subspace_id))?.to_string())
}

fn get_post_uri_cid(subspace_id: u64, post_id: u64) -> Result<String, ContractError> {
    Ok(prepare_particle(format!("desmos://{}/{}", subspace_id, post_id))?.to_string())
}