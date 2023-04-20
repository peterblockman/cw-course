use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde] // = #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Coin,
}

#[cw_serde]
#[derive(QueryResponses)] // only for query
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}


#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}

#[cw_serde]
pub enum ExecMsg {
    Donate{},
    Reset {
        #[serde(default)]
        counter: u64,
    },
    Withdraw {},
    WithdrawTo {
        receiver: String,
        funds: Vec<Coin>
    }
}

