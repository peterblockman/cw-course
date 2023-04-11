use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Empty, StdResult, entry_point, Deps, Binary, to_binary};


mod contract;
pub mod msg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response>{
    Ok(Response::new())
}


#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: msg::QueryMsg
) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;

    match msg {
        Value {} => to_binary(&query::value()),
        Incremented { value } => to_binary(&query::incremented(value))
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{Empty, Addr};
    use cw_multi_test::{App, ContractWrapper, Contract, Executor};
    use crate::{execute, instantiate, query};
    use crate::msg::{QueryMsg, ValueResp};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            Addr::unchecked("sender"), // create and address without validating it
            &Empty {},
            &[],
            "Counting contract",
            None, // admin
        ).unwrap();

        let resp: ValueResp = app
            .wrap() // convert app to QuerierWrapper
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn increment(){
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            Addr::unchecked("sender"), // create and address without validating it
            &Empty {},
            &[],
            "Counting contract",
            None, // admin
        ).unwrap();

        let resp: ValueResp = app
            .wrap() // convert app to QuerierWrapper
            .query_wasm_smart(contract_addr, &QueryMsg::Incremented { value: 1 })
            .unwrap();

        assert_eq!(resp, ValueResp { value: 2 });
    }
}




