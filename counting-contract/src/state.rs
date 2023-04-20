use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

// access a single object in the blockchain
pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
pub const OWNER: Item<Addr> = Item::new("owner");