use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Addr};

use crate::state::PairConfiguration;

#[cw_serde]
pub struct InstantiateMsg {
    pub astroport_address: Addr
}

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
