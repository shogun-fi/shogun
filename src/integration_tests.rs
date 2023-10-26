#[cfg(test)]
mod tests {
    use crate::helpers::{MockContract};
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, Uint256};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn execution_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate(astro_address: Addr) -> (App, MockContract) {
        let mut app = mock_app();
        let execution_contract_id = app.store_code(execution_contract());

        let execution_instantiation = InstantiateMsg {
            astroport_address: astro_address,
        };
        let execution_contract_address = app
            .instantiate_contract(
                execution_contract_id,
                Addr::unchecked(ADMIN),
                &execution_instantiation,
                &[],
                "test",
                None,
            )
            .unwrap();

        let execution_contract = MockContract(execution_contract_address);

        (app, execution_contract) 

    }

    mod execution {
        use std::{str::FromStr, cmp};

        use cosmwasm_std::{Decimal, coin};

        use crate::{state::PairConfiguration, msg::ExecuteMsg};

        use super::*;

        struct MockUser {
            address: Addr,
            supply: Coin,
            slippage_tolerance: String,
        }

        fn generate_random_address() -> Addr {
            Addr::unchecked(cosmrs::crypto::secp256k1::SigningKey::random()
                .public_key()
                .account_id("shogun")
                .unwrap()
                .to_string())
        }

        #[test]
        fn complete_single_pair() {
            let astroport_address = generate_random_address();
            let (mut app, execution_contract) = proper_instantiate(astroport_address);

            let base = "ETH";
            let quote = "ATOM";
            let price: u128 = 250_000_000;

            // we use the smallest denomination of each coin, microatom for ATOM and wei for ETH
            let users: Vec<MockUser> = vec![
                MockUser { address: generate_random_address(), supply: coin(2_000_000_000_000_000_000, "ETH"), slippage_tolerance: "0.02".to_string() }, // A
                MockUser { address: generate_random_address(), supply: coin(2_000_000_000_000_000_000, "ETH"), slippage_tolerance: "0.015".to_string() }, // B
                MockUser { address: generate_random_address(), supply: coin(5_000_000_000_000_000_000, "ETH"), slippage_tolerance: "0.01".to_string() }, // C
                MockUser { address: generate_random_address(), supply: coin(100_000_000, "ATOM"), slippage_tolerance: "0.015".to_string() }, // D
                MockUser { address: generate_random_address(), supply: coin(300_000_000, "ATOM"), slippage_tolerance: "0.015".to_string() }, // E
                MockUser { address: generate_random_address(), supply: coin(450_000_000, "ATOM"), slippage_tolerance: "0.015".to_string() }, // F
                MockUser { address: generate_random_address(), supply: coin(600_000_000, "ATOM"), slippage_tolerance: "0.01".to_string() }, // G
            ];

            // load user balances into their wallets and calculate total amounts
            let base_supply: Uint128 = 0u128.into();
            let quote_supply: Uint128 = 0u128.into();

            for user in &users {
                app.sudo(cw_multi_test::SudoMsg::Bank(
                    cw_multi_test::BankSudo::Mint { to_address: user.address.to_string(), amount: vec![user.supply.clone()]}
                ))
                .unwrap();
            }

            let pair_config = PairConfiguration {
                base: "ETH".into(),
                base_supply,
                quote: "ATOM".into(),
                surplus: None,
                quote_supply,
                exchange_rate: price,
            };

            // initialise the batch settelement
            let prepare_msg = ExecuteMsg::Prepare { assets: vec![pair_config] };
            let _ = app.execute_contract::<ExecuteMsg>(Addr::unchecked("solver"), execution_contract.addr(), &prepare_msg, &vec![coin(0, "")]); 

            for user in &users {
                let supply_msg = ExecuteMsg::Supply {
                    quote: "ETH".into(),
                    slippage_tolerance: Decimal::from_str(&user.slippage_tolerance).unwrap()
                };
                let _ = app.execute_contract::<ExecuteMsg>(user.address.clone().into(), execution_contract.addr(), &supply_msg, &vec![]);
            }

            let settle_msg = ExecuteMsg::Settle{};
            let resp = app.execute_contract(Addr::unchecked("solver"), execution_contract.addr(), &settle_msg, &vec![]);
        }
    }
}
