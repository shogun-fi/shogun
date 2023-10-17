#[cfg(test)]
mod tests {
    use crate::helpers::{MockContract};
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use self::mock_dex::MockDex;

    pub fn execution_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    pub fn mock_dex() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            mock_dex::execute,
            mock_dex::instantiate,
            mock_dex::query,
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

    fn proper_instantiate() -> (App, MockContract, MockDex, MockDex) {
        let mut app = mock_app();
        let execution_contract_id = app.store_code(execution_contract());
        let mock_dex_one_id = app.store_code(mock_dex());
        let mock_dex_two_id = app.store_code(mock_dex());

        let execution_instantiation = InstantiateMsg { };
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

        let mock_dex_one_instantiation = mock_dex::InstantiateMsg {
            bid_denom: todo!(),
            ask_denom: todo!(),
            price: todo!(),
        };
        let mock_dex_one_address = app
            .instantiate_contract(
                mock_dex_one_id,
                Addr::unchecked(ADMIN),
                &mock_dex_one_instantiation,
                &[],
                "test",
                None,
            )
            .unwrap();
        
        let mock_dex_two_instantiation = mock_dex::InstantiateMsg {
            bid_denom: todo!(),
            ask_denom: todo!(),
            price: todo!(),
        };
        let mock_dex_two_address = app
            .instantiate_contract(
                mock_dex_two_id,
                Addr::unchecked(ADMIN),
                &mock_dex_two_instantiation,
                &[],
                "test",
                None,
            )
            .unwrap();

        let execution_contract = MockContract(execution_contract_address);

        let mock_dex_one = MockDex(mock_dex_one_address);
        let mock_dex_two= MockDex(mock_dex_two_address);

        (app, execution_contract, mock_dex_one, mock_dex_two)

    }

    mod execution {
        use super::*;

        #[test]
        fn complete() {
            let (mut app, execution_contract, mock_dex_one, mock_dex_two) = proper_instantiate();

            // let msg = ExecuteMsg { };
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            
            //app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }

    mod mock_dex {
        use cosmwasm_schema::cw_serde;
        use cosmwasm_std::{entry_point, Deps, Binary, StdResult, Decimal, WasmMsg, CosmosMsg, to_binary, Addr};
        use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        use thiserror::Error;

        #[cw_serde]
        pub struct InstantiateMsg {
            pub bid_denom: String,
            pub ask_denom: String,

            pub price: Decimal
        }

        #[cw_serde]
        pub enum ExecuteMsg {
            Swap {
                offer_denom: String
            }
        }

        #[cw_serde]
        pub enum QueryMsg {}

        #[derive(Error, Debug)]
        pub enum ContractError {}

        pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
            unimplemented!()
        }

        pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: ExecuteMsg) -> Result<Response, ContractError> {
            unimplemented!()
        }

        pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
            unimplemented!()
        }

        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
        pub struct MockDex(pub Addr);

        impl MockDex {
            pub fn addr(&self) -> Addr {
                self.0.clone()
            }

            pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
                let msg = to_binary(&msg.into())?;
                Ok(WasmMsg::Execute {
                    contract_addr: self.addr().into(),
                    msg,
                    funds: vec![],
                }
                .into())
            }
        }


    }
}
