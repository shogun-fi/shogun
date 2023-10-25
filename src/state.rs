use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Addr, Coin, Timestamp, Decimal, CosmosMsg, Fraction};
use cw_storage_plus::{Map, Item};

pub const PAIRS: Map<(String, String), PairConfiguration> = Map::new("pairs");
pub const SETTLEMENT_MESSAGES: Item<Vec<CosmosMsg>> = Item::new("settlement_messages");
pub const ASTROPORT_ADDRESS: Item<Addr> = Item::new("astroport_address");

#[cw_serde]
pub enum Surplus {
    Base((Uint128, Uint128)),
    Quote((Uint128, Uint128)),
    Match
}

// TODO: Maybe support fixed set of currencies through enums or remain flexible?
#[cw_serde]
pub struct PairConfiguration {
    pub base: String,
    pub base_supply: Uint128,

    pub quote: String,
    pub quote_supply: Uint128,

    pub surplus: Option<Surplus>,

    pub exchange_rate: Decimal,

    // IDEA: Add amount of orders on each side of the pair for contractual guarantees and security?
}
