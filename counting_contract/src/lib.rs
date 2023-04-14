use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, entry_point, Deps, Binary, to_binary};
use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg};


mod contract;
pub mod msg;
mod state;
mod error;

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
        Incremented { value } => to_binary(&query::incremented(value))
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{Empty, Addr, coin, coins};
    use cw_multi_test::{App, ContractWrapper, Contract, Executor};
    use crate::{execute, instantiate, query};
    use crate::error::ContractError;
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
    fn reset() {
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

    #[test]
    fn withdraw() {
        let owner = Addr::unchecked("owner");
        let sender = Addr::unchecked("sender");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        )
            .unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
            .unwrap();

        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(10, "atom")
        );
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            vec![]
        );
    }

    #[test]
    fn withdraw_to() {
        let owner = Addr::unchecked("owner");
        let sender = Addr::unchecked("sender");
        let receiver = Addr::unchecked("receiver");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap()
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg{
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        ).unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::WithdrawTo {
                receiver: receiver.to_string(),
                funds: coins(5, "atom"),
            },
            &[]
        ).unwrap();

    }

    #[test]
    fn unauthorized_withdraw() {
        let owner = Addr::unchecked("owner");
        let member = Addr::unchecked("member");

        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(member, contract_addr, &ExecMsg::Withdraw {}, &[])
            .unwrap_err(); // get the error

        assert_eq!(
            ContractError::Unauthorized {
                owner: owner.into()
            },
            // downcast convert Error to ContractError
            err.downcast().unwrap()
        );
    }

    #[test]
    fn unauthorized_reset() {
        let owner = Addr::unchecked("owner");
        let member = Addr::unchecked("member");

        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(member, contract_addr, &ExecMsg::Reset { counter: 10 }, &[])
            .unwrap_err();


        assert_eq!(
            ContractError::Unauthorized {
                owner: owner.into()
            },
            err.downcast().unwrap()
        );
    }

    #[test]
    fn unauthorized_withdraw_to() {
        let owner = Addr::unchecked("owner");
        let member = Addr::unchecked("member");

        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(
                member,
                contract_addr,
                &ExecMsg::WithdrawTo {
                    receiver: owner.to_string(),
                    funds: vec![],
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                owner: owner.into()
            },
            err.downcast().unwrap()
        );
    }
}




