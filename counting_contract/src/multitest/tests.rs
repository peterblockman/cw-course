use cosmwasm_std::{Addr, coin, coins};
use cw_multi_test::App;
use crate::msg::ValueResp;
use crate::multitest::contract::CountingContract;

#[test]
fn donate_with_funds() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });

    let code_id = CountingContract::store_code(&mut app);

    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        coin(10, "atom"),
    )
        .unwrap();

    contract
        .donate(&mut app, &sender, &coins(10, "atom"))
        .unwrap();

    let resp = contract.query_value(&app).unwrap();

    assert_eq!(resp, ValueResp { value: 1});
}