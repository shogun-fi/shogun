use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Timestamp, Decimal};

use crate::state::PairConfiguration;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Prepare {
        assets: Vec<PairConfiguration>,
    },
    Supply {
        quote: String,
        slippage_tolerance: Decimal
    },
    Settle {

    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
