use cw_storage_plus::Item;

// access a single object in the blockchain
pub const COUNTER: Item<u64> = Item::new("counter");