use std::cmp::{self, Ordering};

use astroport::asset::AssetInfo;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg, CosmosMsg, Uint128, BankMsg, Decimal, to_binary, coin, coins, Coin};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PairConfiguration, Surplus, SETTLEMENT_MESSAGES, PAIRS, ASTROPORT_ADDRESS};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:shogun_neutron";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ASTROPORT_ADDRESS.save(deps.storage, &msg.astroport_address)?;
    Ok(Response::default())
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
        ExecuteMsg::Supply { quote, slippage_tolerance } => deposit(deps, env, info, quote, slippage_tolerance),
        ExecuteMsg::Settle {  } => settle(deps, env, info)
    } 
}

fn prepare(deps: DepsMut, env: Env, info: MessageInfo, assets: Vec<PairConfiguration>) ->Result<Response, ContractError> {

    for mut pair in assets {
        let base_demand = pair.quote.amount / pair.exchange_rate;

        pair.surplus = match pair.base.amount.cmp(&base_demand) {
            Ordering::Equal => None,

            Ordering::Greater => {
                let surplus = pair.base.amount - base_demand;
                Some(coin(surplus.into(), pair.base.denom.clone()))
            },

            Ordering::Less => {
                let surplus = (base_demand - pair.base.amount) * pair.exchange_rate;
                Some(coin(surplus.into(), pair.quote.denom.clone()))
            },
        };

        PAIRS.save(deps.storage, (pair.base.denom.clone(), pair.quote.denom.clone()), &pair)?;
    }

    SETTLEMENT_MESSAGES.save(deps.storage, &vec![])?;

    Ok(Response::default())
}

// TODO: Convert asset IDs from strings to numbers
// TODO: Clean up clones and borrowing

/// Submits a signed order to a pending batch settlement.
/// 
/// On processing of an order submission, the bank module has already transferred the funds that a user wishes to offer to the custody of the execution contract. It is through this fund transfer that the user's offer is inferred (amount and denomination).
fn deposit(deps: DepsMut, env: Env, info: MessageInfo, buy_denom: String, slippage_tolerance: Decimal) -> Result<Response, ContractError> {
    let dex_address = ASTROPORT_ADDRESS.load(deps.storage)?;

    let user_address = &info.sender;
    let mut supply: Coin = match info.funds.get(0) {
        Some(supply) => supply.clone(),
        None => return Err(ContractError::InvalidOrder)
    };

    let pair = match PAIRS.may_load(deps.storage, (cmp::max(&supply.denom, &buy_denom).clone(), cmp::min(&supply.denom, &buy_denom).clone()))? {
        Some(pair) => pair,
        None => return Err(ContractError::InvalidOrder),
    };

    let mut settlement_messages: Vec<CosmosMsg> = Vec::new();

    if let (Some(surplus), true) = (pair.surplus.clone(), pair.surplus.is_some_and(|surplus| surplus.denom == supply.denom)) { 
        let user_residual = {
            if supply.denom == pair.base.denom {
                supply.amount.multiply_ratio(surplus.amount, pair.base.amount)
            } else {
                supply.amount.multiply_ratio(surplus.amount, pair.quote.amount)
            }
        };

        settlement_messages.push(WasmMsg::Execute {
            contract_addr: dex_address.into(),
            msg: to_binary(&astroport::pair::ExecuteMsg::Swap {
                offer_asset: astroport::asset::Asset {
                    info: AssetInfo::NativeToken { denom: supply.denom.clone() },
                    amount: user_residual,
                },
                ask_asset_info: Some(AssetInfo::NativeToken { denom: buy_denom.clone() }),
                belief_price: None,
                max_spread: slippage_tolerance.into(),
                to: Some(user_address.into()),
            })?,
            funds: coins(user_residual.into(), supply.denom.clone())
        }.into());

        supply.amount -= user_residual;
    }
    
    let cow_matched = {
        if supply.denom == pair.base.denom {
            supply.amount * pair.exchange_rate
        } else {
            supply.amount / pair.exchange_rate
        }
    };

    settlement_messages.push(BankMsg::Send {
        to_address: user_address.clone().into(),
        amount: coins(cow_matched.into(), buy_denom),
    }.into());

    SETTLEMENT_MESSAGES.update(deps.storage, |mut msgs| -> StdResult<_> {
        msgs.append(&mut settlement_messages);
        Ok(msgs)
    })?;

    Ok(Response::default())
}

fn settle(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let messages = SETTLEMENT_MESSAGES.load(deps.storage)?;
    Ok(Response::default().add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, Addr, coins, coin, Decimal};

    use crate::{msg::InstantiateMsg, state::{PairConfiguration, SETTLEMENT_MESSAGES}};

    #[test]
    fn init() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(0, "token"));

        let astroport_address = Addr::unchecked("astroport");
        let msg = InstantiateMsg {astroport_address};

        let _res = super::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // check if astroport msg is saved
    }

    #[test]
    fn prepare() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(0, "token"));

        let astroport_address = Addr::unchecked("astroport");
        let msg = InstantiateMsg {astroport_address};

        let _res = super::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn supply() {
        // TODO: Make automated
        let mut deps = mock_dependencies();
        let owner_info = mock_info("creator", &coins(0, "token"));
        let msg = InstantiateMsg {astroport_address: Addr::unchecked("astroport")};
        let _res = super::instantiate(deps.as_mut(), mock_env(), owner_info.clone(), msg).unwrap();

        let pair_configuration = PairConfiguration {
            base: coin(500_000, "ETH"), 
            quote: coin(100_000, "ATOM"), 
            surplus: None,
            exchange_rate: 3u128.into(), 
        };

        let _res = super::prepare(deps.as_mut(), mock_env(), owner_info, vec![pair_configuration]);

        let info = mock_info("user_1", &coins(200_000, "ETH"));
        let _res = super::deposit(deps.as_mut(), mock_env(), info, "ATOM".to_owned(), Decimal::from_str("0.04").unwrap());

        let info = mock_info("user_2", &coins(300_000, "ETH"));
        let _res = super::deposit(deps.as_mut(), mock_env(), info, "ATOM".to_owned(), Decimal::from_str("0.04").unwrap());

        let info = mock_info("user_3", &coins(40_000, "ATOM"));
        let _res = super::deposit(deps.as_mut(), mock_env(), info, "ETH".to_owned(), Decimal::from_str("0.04").unwrap());

        let info = mock_info("user_4", &coins(30_000, "ATOM"));
        let _res = super::deposit(deps.as_mut(), mock_env(), info, "ETH".to_owned(), Decimal::from_str("0.04").unwrap());

        let info = mock_info("user_4", &coins(30_000, "ATOM"));
        let _res = super::deposit(deps.as_mut(), mock_env(), info, "ETH".to_owned(), Decimal::from_str("0.04").unwrap());

        let msgs = SETTLEMENT_MESSAGES.load(&deps.storage).unwrap();
    }
}
