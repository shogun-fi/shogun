use std::cmp;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg, CosmosMsg, Uint128, BankMsg};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PairConfiguration, Surplus, SETTLEMENT_MESSAGES, PAIRS};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:shogun_neutron";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::Prepare { assets } => prepare(deps, env, info, assets),
    ExecuteMsg::Supply { quote } => deposit(deps, env, info, quote),
    ExecuteMsg::Settle {  } => settle(deps, env, info)
    } 
}

fn prepare(deps: DepsMut, env: Env, info: MessageInfo, assets: Vec<PairConfiguration>) ->Result<Response, ContractError> {

    for mut pair in assets {
        let base_demand = pair.quote_supply.div_ceil(pair.exchange_rate);

        if pair.base_supply == base_demand {
            pair.surplus = Surplus::Match;
        } else if pair.base_supply > base_demand {
            let surplus = pair.base_supply - base_demand;
            let proportion_fraction = (surplus, pair.base_supply.clone());
            pair.surplus = Surplus::Base(proportion_fraction);
        } else {
            let surplus = base_demand - pair.quote_supply;
            let proportion_fraction = (surplus, pair.quote_supply.clone());
            pair.surplus = Surplus::Base(proportion_fraction);
        }

        PAIRS.save(deps.storage, (pair.base.clone(), pair.quote.clone()), &pair)?;
    }

    Ok(Response::default())
}

/// Submits a signed order to a pending batch settlement.
/// 
/// On processing of an order submission, the bank module has already transferred the funds that a user wishes to offer to the custody of the execution contract. It is through this fund transfer that the user's offer is inferred (amount and denomination).
fn deposit(deps: DepsMut, env: Env, info: MessageInfo, buy_denom: String) -> Result<Response, ContractError> {
    let user_address = info.sender;
    let supply = match info.funds.get(0) {
        Some(supply) => supply.clone(),
        None => return Err(ContractError::InvalidOrder)
    };

    let base = cmp::max(&supply.denom, &buy_denom);
    let quote = cmp::max(&supply.denom, &buy_denom);

    let pair = match PAIRS.may_load(deps.storage, (base.clone(), quote.clone()))? {
        Some(pair) => pair,
        None => todo!(),
    };

    let mut settlement_messages: Vec<CosmosMsg> = Vec::new();

    let routed: Uint128 = Uint128::new(0);
    match pair.surplus {
        Surplus::Base(routing_proportion) if supply.denom == pair.base => {
            let routing_amount = supply.amount.multiply_ratio(routing_proportion.0, routing_proportion.1);
            let funds_to_route = cosmwasm_std::coins(routing_amount.into(), supply.denom);

            let routing_msg = WasmMsg::Execute {
                contract_addr: todo!(),
                msg: todo!(),
                funds: funds_to_route
            };

            settlement_messages.push(routing_msg.into());

            routed = routed + routing_amount;
        },

        Surplus::Quote(routing_proportion) if supply.denom == pair.quote => {
            let routing_amount = supply.amount.multiply_ratio(routing_proportion.0, routing_proportion.1);
            let funds_to_route = cosmwasm_std::coins(routing_amount.into(), supply.denom);

            let routing_msg = WasmMsg::Execute {
                contract_addr: todo!(),
                msg: todo!(),
                funds: funds_to_route
            };

            settlement_messages.push(routing_msg.into());

            routed = routed + routing_amount;
        },
        _ => {}
    }

    let remainder = supply.amount - routed;
    let exchange_value = remainder.mul_ceil(pair.exchange_rate);

    let settlement_message = BankMsg::Send {
        to_address: user_address.into(),
        amount: cosmwasm_std::coins(exchange_value.into(), supply.denom)

    };
    
    settlement_messages.push(settlement_message.into());

    match SETTLEMENT_MESSAGES.may_load(deps.storage) {
        Ok(data) => {
            let mut msgs = data.unwrap_or_default();

            msgs.append(&mut settlement_messages);
            
            SETTLEMENT_MESSAGES.save(deps.storage, &msgs)?;
        },
        Err(_) => todo!()
    }

    Ok(Response::default())
}

fn settle(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let messages = match SETTLEMENT_MESSAGES.may_load(deps.storage)? {
        Some(msgs) => msgs,
        None => todo!(),
    };

    Ok(Response::default().add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {

}
