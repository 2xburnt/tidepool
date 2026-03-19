#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::{
        Addr, Coin, ContractResult, Empty, Env, OwnedDeps, SystemResult, Uint128, Uint256,
        WasmQuery,
        testing::{MockApi, message_info, mock_env},
        to_json_binary,
    };

    use crate::contract::{execute, instantiate};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use tidepool_types::{AgentResponse, ESCROW_DENOM, ReputationQueryMsg};

    // Custom MockQuerier that handles reputation contract queries
    use cosmwasm_std::{Binary, Querier, QuerierResult, QueryRequest, SystemError};

    const REP_CONTRACT: &str = "reputation_contract";

    /// A querier that returns AgentResponse for registered addresses,
    /// and an error for unregistered ones.
    struct ReputationMockQuerier {
        registered: Vec<String>,
        base: cosmwasm_std::testing::MockQuerier,
    }

    impl ReputationMockQuerier {
        fn new(registered: Vec<String>) -> Self {
            Self {
                registered,
                base: cosmwasm_std::testing::MockQuerier::default(),
            }
        }
    }

    impl Querier for ReputationMockQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
            let request: QueryRequest<Empty> = match cosmwasm_std::from_json(bin_request) {
                Ok(r) => r,
                Err(e) => {
                    return SystemResult::Err(SystemError::InvalidRequest {
                        error: e.to_string(),
                        request: Binary::from(bin_request.to_vec()),
                    });
                }
            };

            match &request {
                QueryRequest::Wasm(WasmQuery::Smart { msg, .. }) => {
                    let query_msg: ReputationQueryMsg = cosmwasm_std::from_json(msg).unwrap();
                    match query_msg {
                        ReputationQueryMsg::GetAgent { address } => {
                            if self.registered.contains(&address) {
                                let resp = AgentResponse {
                                    address: Addr::unchecked(&address),
                                    name: "Test Agent".to_string(),
                                    skills: vec![],
                                    total_earned: Uint128::zero(),
                                    total_spent: Uint128::zero(),
                                    jobs_completed: 0,
                                    jobs_posted: 0,
                                    registered_at: 0,
                                };
                                SystemResult::Ok(ContractResult::Ok(to_json_binary(&resp).unwrap()))
                            } else {
                                SystemResult::Ok(ContractResult::Err("Agent not found".to_string()))
                            }
                        }
                    }
                }
                _ => self.base.raw_query(bin_request),
            }
        }
    }

    type TestDeps = OwnedDeps<MockStorage, MockApi, ReputationMockQuerier, Empty>;

    fn owner() -> Addr {
        MockApi::default().addr_make("owner")
    }

    fn alice() -> Addr {
        MockApi::default().addr_make("alice")
    }

    fn unregistered_user() -> Addr {
        MockApi::default().addr_make("unregistered")
    }

    fn rep_contract() -> Addr {
        MockApi::default().addr_make(REP_CONTRACT)
    }

    fn setup_contract(registered_agents: Vec<String>) -> (TestDeps, Env) {
        let querier = ReputationMockQuerier::new(registered_agents);
        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier,
            custom_query_type: std::marker::PhantomData,
        };
        let env = mock_env();
        let info = message_info(&owner(), &[]);
        instantiate(
            deps.as_mut(),
            env.clone(),
            info,
            InstantiateMsg {
                reputation_contract: rep_contract().to_string(),
                minimum_escrow: Some(Uint128::new(100_000)),
            },
        )
        .unwrap();
        (deps, env)
    }

    fn valid_escrow() -> Vec<Coin> {
        vec![Coin {
            denom: ESCROW_DENOM.to_string(),
            amount: Uint256::from(200_000u128),
        }]
    }

    #[test]
    fn test_post_task_registered_poster_succeeds() {
        let alice = alice();
        let (mut deps, env) = setup_contract(vec![alice.to_string()]);

        let res = execute(
            deps.as_mut(),
            env,
            message_info(&alice, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Test Task".to_string(),
                description: "Do the thing".to_string(),
                required_skills: vec!["rust".to_string()],
                expires_in_blocks: None,
            },
        );

        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(
            resp.attributes
                .iter()
                .any(|a| a.key == "method" && a.value == "post_task")
        );
        assert!(
            resp.attributes
                .iter()
                .any(|a| a.key == "task_id" && a.value == "1")
        );
    }

    #[test]
    fn test_post_task_unregistered_poster_fails() {
        let unreg = unregistered_user();
        // Set up with no registered agents
        let (mut deps, env) = setup_contract(vec![]);

        let err = execute(
            deps.as_mut(),
            env,
            message_info(&unreg, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Test Task".to_string(),
                description: "Do the thing".to_string(),
                required_skills: vec!["rust".to_string()],
                expires_in_blocks: None,
            },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::AgentNotRegistered {}),
            "Expected AgentNotRegistered, got: {err:?}"
        );
    }
}
