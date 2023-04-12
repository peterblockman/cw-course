use cosmwasm_std::{Deps, DepsMut, Response, StdResult};
use crate::msg::ValueResp;
use crate::state::COUNTER;

pub fn instantiate (deps: DepsMut, counter: u64) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};
    use crate::msg::ValueResp;
    use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
       let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp{ value })
   }

    pub fn incremented(value: u64) -> ValueResp {
        ValueResp{ value: value + 1}
    }

}