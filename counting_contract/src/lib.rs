use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Empty, StdResult, entry_point, Deps, Binary, to_binary};
use crate::msg::{ExecMsg, InstantiateMsg};
use crate::state::COUNTER;


mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response>{
    contract::instantiate(deps, msg.counter, msg.minimal_donation)
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecMsg
) -> StdResult<Response> {
    use crate::msg::ExecMsg::*;
    use crate::contract::exec;

    match  msg {
        Donate {} => exec::donate(deps, info),
        Reset { counter } => exec::reset(deps, info, counter)
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
        Incremented { value } => to_binary(&query::incremented(value))
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{Empty, Addr, entry_point, coin, coins};
    use cw_multi_test::{App, ContractWrapper, Contract, Executor};
    use crate::{execute, instantiate, query};
    use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};

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
            &InstantiateMsg { counter: 1, minimal_donation: coin(10, "atom") },
            &[],
            "Counting contract",
            None, // admin
        ).unwrap();

        let resp: ValueResp = app
            .wrap() // convert app to QuerierWrapper
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    fn increment(){
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            Addr::unchecked("sender"), // create and address without validating it
            &InstantiateMsg { counter: 1 , minimal_donation: coin(10, "atom") },
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

    #[test]
    fn donate() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg{ counter: 0, minimal_donation: coin(10, "atom") },
                &[],
                "Counting contract",
                None, // admin
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(), // clone here so that it can be used below
            &ExecMsg::Donate {},
            &[],
        ).unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn donate_with_funds() {
        let sender = Addr::unchecked("sender");
        let mut app = App::new(|router, _api, storage|{
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg{ counter: 0, minimal_donation: coin(10, "atom") },
                &[],
                "Counting contract",
                None, // admin
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(), // clone here so that it can be used below
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        ).unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    fn expecting_no_funds() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg{ counter: 0, minimal_donation: coin(0, "atom") },
                &[],
                "Counting contract",
                None, // admin
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(), // clone here so that it can be used below
            &ExecMsg::Donate {},
            &[],
        ).unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    pub fn reset() {
        let mut app =  App::default();

        let contract_id = app.store_code(counting_contract());
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg{ counter: 0, minimal_donation: coin(10, "atom"), },
                &[],
                "Counting contract",
                None, // admin
            )
            .unwrap();

        // app.execute_contract(
        //     Addr::unchecked("sender"),
        //     contract_addr.clone(), // clone here so that it can be used below
        //     &ExecMsg::Poke {},
        //     &[],
        // ).unwrap();
        //
        // let poke_resp: ValueResp = app
        //     .wrap()
        //     .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
        //     .unwrap();
        //
        // assert_eq!(poke_resp, ValueResp { value: 1 });

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(), // clone here so that it can be used below
            &ExecMsg::Reset { counter: 10 },
            &[],
        ).unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 10 });
    }
}




