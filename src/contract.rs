use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Contract name and version for storage
const CONTRACT_NAME: &str = "crates.io:simple";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Simple { input: Uint256 },
    PlusTwo { a: Uint256, b: Uint256 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PlusTwo { a: Uint256, b: Uint256 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlusTwoResponse {
    pub result: Uint256,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Simple { input } => execute_simple(env, info, input),
        ExecuteMsg::PlusTwo { a, b } => execute_plus_two(a, b),
    }
}

pub fn execute_simple(env: Env, info: MessageInfo, input: Uint256) -> StdResult<Response> {
    Ok(Response::new()
        .add_attribute("method", "simple")
        .add_attribute("block_number", env.block.height.to_string())
        .add_attribute("caller", info.sender.to_string())
        .add_attribute("input", input.to_string()))
}

pub fn execute_plus_two(a: Uint256, b: Uint256) -> StdResult<Response> {
    let result = a + b;
    Ok(Response::new()
        .add_attribute("method", "plus_two")
        .add_attribute("result", result.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PlusTwo { a, b } => to_json_binary(&query_plus_two(deps, a, b)?),
    }
}

fn query_plus_two(_deps: Deps, a: Uint256, b: Uint256) -> StdResult<PlusTwoResponse> {
    Ok(PlusTwoResponse { result: a + b })
} 