#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{attr, Binary, coin, Coins, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::state::{STATE, State, ISLANDS, Island};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:columbus";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOKEN_PER_RESOURCE: u128 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STATE.save(deps.storage, &State {
        islands_discovered: 0,
        total_tokens_minted: vec![],
    })?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DiscoverIsland { name } => try_discover_island(deps, env, info, name),
        ExecuteMsg::GatherResources { island_name } => try_gather_resources(deps, env, info, island_name),
        ExecuteMsg::Mint { island_name } => try_mint(deps, info, island_name),
    }
}

fn try_discover_island(deps: DepsMut, env: Env, info: MessageInfo, name: String) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    return match ISLANDS.may_load(deps.storage, (&sender, name.clone()))? {
        Some(_) => Err(ContractError::Duplicate {}),
        None => {
            let new_island = Island {
                name: name.clone(),
                resources: 0,
                last_gather_time: env.block.time.seconds(),
                token_supply: Uint128::zero(),
            };

            ISLANDS.save(deps.storage, (&sender, name.clone()), &new_island)?;
            STATE.update(deps.storage, |mut state| -> StdResult<_> {
                state.islands_discovered += 1;
                Ok(state)
            })?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "try_discover_island"),
                attr("owner", info.sender.to_string()),
                attr("name", name)
            ]))
        }
    }
}

fn try_gather_resources(deps: DepsMut, env: Env, info: MessageInfo, island_name: String) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    return match ISLANDS.may_load(deps.storage, (&sender, island_name.clone()))? {
        Some(mut island) => {
            let now = env.block.time.seconds();
            let time_diff = now - island.last_gather_time;

            if time_diff < 60 {
                return Err(ContractError::GatherLimit {});
            }

            island.resources += time_diff;
            island.last_gather_time = now;

            ISLANDS.save(deps.storage, (&sender, island_name), &island)?;

            Ok(Response::new().add_attributes(vec![
                attr("method", "try_gather_resources"),
                attr("gathered", time_diff.to_string()),
            ]))
        }
        None => Err(ContractError::NotFound {})
    }
}

fn try_mint(deps: DepsMut, info: MessageInfo, island_name: String) -> Result<Response, ContractError> {
    let sender = info.sender.clone();
    return match ISLANDS.may_load(deps.storage, (&sender, island_name.clone()))? {
        Some(mut island) => {
            let tokens_to_mint = Uint128::from(island.resources as u128 * TOKEN_PER_RESOURCE);

            island.token_supply += tokens_to_mint;
            island.resources = 0;

            ISLANDS.save(deps.storage, (&sender, island_name.clone()), &island)?;
            STATE.update(deps.storage, |mut state| -> StdResult<_> {
                let mut tokens_minted = Coins::try_from(state.total_tokens_minted)?;
                tokens_minted.add(coin(tokens_to_mint.u128(), format!("{}/{}", sender, island_name)))?;
                state.total_tokens_minted = tokens_minted.to_vec();
                Ok(state)
            })?;

            Ok(Response::new().add_attribute("method", "try_mint").add_attribute("amount", tokens_to_mint.to_string()))
        }
        None => Err(ContractError::NotFound {})
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetIsland { founder, name } => to_json_binary(&query_island(deps, founder, name)?),
        QueryMsg::GetState {} => to_json_binary(&query_state(deps)?),
    }
}

fn query_island(deps: Deps, founder: String, name: String) -> StdResult<Island> {
    let addr = deps.api.addr_validate(founder.as_str())?;
    let island = ISLANDS.load(deps.storage, (&addr, name))?;
    Ok(island)
}

fn query_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage)?;
    Ok(state)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}
