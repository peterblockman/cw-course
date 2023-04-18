use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, entry_point, Deps, Binary, to_binary};
use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg};


mod contract;
pub mod msg;
mod state;
mod error;

#[cfg(test)]
mod multitest;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response>{
    contract::instantiate(deps, info, msg.counter, msg.minimal_donation)
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use crate::msg::ExecMsg::*;
    use crate::contract::exec;

    match  msg {
        Donate {} => exec::donate(deps, info).map_err(ContractError::Std),
        Reset { counter } => exec::reset(deps, info, counter),
        Withdraw {} => exec::withdraw(deps, env, info),
        WithdrawTo { receiver, funds } => {
            exec::withdraw_to(deps, env, info, receiver, funds)
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: msg::QueryMsg
) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
    }
}

