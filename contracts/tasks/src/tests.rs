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

    fn bob() -> Addr {
        MockApi::default().addr_make("bob")
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

    /// Regression test: full lifecycle (post → claim → submit → approve) settles correctly
    /// when both poster and worker are registered. Verifies settlement emits escrow payout
    /// and reputation update messages.
    #[test]
    fn test_full_lifecycle_settlement_succeeds() {
        let poster = alice();
        let worker = bob();
        let (mut deps, env) =
            setup_contract(vec![poster.to_string(), worker.to_string()]);

        // Post
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Settle test".to_string(),
                description: "Full lifecycle".to_string(),
                required_skills: vec!["rust".to_string()],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        // Claim
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&worker, &[]),
            ExecuteMsg::ClaimTask { task_id: 1 },
        )
        .unwrap();

        // Submit
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&worker, &[]),
            ExecuteMsg::SubmitTask { task_id: 1 },
        )
        .unwrap();

        // Approve → settlement
        let res = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &[]),
            ExecuteMsg::ApproveTask { task_id: 1 },
        )
        .unwrap();

        // Settlement should produce 2 messages: BankMsg::Send + WasmMsg::Execute (UpdateVolume)
        assert_eq!(res.messages.len(), 2, "Expected 2 settlement messages");
        assert!(
            res.attributes
                .iter()
                .any(|a| a.key == "method" && a.value == "approve_task"),
        );
        assert!(
            res.attributes
                .iter()
                .any(|a| a.key == "claimant" && a.value == worker.to_string()),
        );
    }

    /// Regression test for the exact stuck-funds scenario from DO-264:
    /// Before the fix, an unregistered poster could PostTask successfully, but settlement
    /// would fail because the reputation contract couldn't find the poster profile —
    /// leaving escrow permanently stuck. Now PostTask rejects unregistered posters upfront,
    /// so the stuck-funds path is unreachable.
    #[test]
    fn test_unregistered_poster_cannot_reach_settlement() {
        let unreg = unregistered_user();
        let worker = bob();
        // Worker is registered, poster is NOT
        let (mut deps, env) = setup_contract(vec![worker.to_string()]);

        // Unregistered poster cannot even post — escrow never enters the contract
        let err = execute(
            deps.as_mut(),
            env,
            message_info(&unreg, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Stuck funds scenario".to_string(),
                description: "Should be rejected at post time".to_string(),
                required_skills: vec!["rust".to_string()],
                expires_in_blocks: None,
            },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::AgentNotRegistered {}),
            "Unregistered poster must be rejected at PostTask, got: {err:?}"
        );
    }

    #[test]
    fn test_claim_task_unregistered_worker_fails() {
        let poster = alice();
        let unreg = unregistered_user();
        let (mut deps, env) = setup_contract(vec![poster.to_string()]);

        // Poster posts successfully
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Worker reg test".to_string(),
                description: "Only registered workers can claim".to_string(),
                required_skills: vec!["rust".to_string()],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        // Unregistered worker cannot claim
        let err = execute(
            deps.as_mut(),
            env,
            message_info(&unreg, &[]),
            ExecuteMsg::ClaimTask { task_id: 1 },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::AgentNotRegistered {}),
            "Unregistered worker must be rejected at ClaimTask, got: {err:?}"
        );
    }

    #[test]
    fn test_self_claim_rejected() {
        let poster = alice();
        let (mut deps, env) = setup_contract(vec![poster.to_string()]);

        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Self claim test".to_string(),
                description: "Poster cannot claim own task".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        let err = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &[]),
            ExecuteMsg::ClaimTask { task_id: 1 },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::CannotClaimOwnTask {}),
            "Expected CannotClaimOwnTask, got: {err:?}"
        );
    }

    #[test]
    fn test_cancel_only_when_open() {
        let poster = alice();
        let worker = bob();
        let (mut deps, env) =
            setup_contract(vec![poster.to_string(), worker.to_string()]);

        // Post
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Cancel test".to_string(),
                description: "Can only cancel open tasks".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        // Claim it
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&worker, &[]),
            ExecuteMsg::ClaimTask { task_id: 1 },
        )
        .unwrap();

        // Try to cancel after claim — should fail
        let err = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &[]),
            ExecuteMsg::CancelTask { task_id: 1 },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::CannotCancel {}),
            "Expected CannotCancel, got: {err:?}"
        );
    }

    #[test]
    fn test_escrow_too_low_rejected() {
        let poster = alice();
        let (mut deps, env) = setup_contract(vec![poster.to_string()]);

        let low_escrow = vec![Coin {
            denom: ESCROW_DENOM.to_string(),
            amount: Uint256::from(50_000u128), // below 100_000 minimum
        }];

        let err = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &low_escrow),
            ExecuteMsg::PostTask {
                title: "Low escrow".to_string(),
                description: "Should fail".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::EscrowTooLow { .. }),
            "Expected EscrowTooLow, got: {err:?}"
        );
    }

    #[test]
    fn test_wrong_denom_rejected() {
        let poster = alice();
        let (mut deps, env) = setup_contract(vec![poster.to_string()]);

        let wrong_denom = vec![Coin {
            denom: "uatom".to_string(),
            amount: Uint256::from(200_000u128),
        }];

        let err = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &wrong_denom),
            ExecuteMsg::PostTask {
                title: "Wrong denom".to_string(),
                description: "Should fail".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::WrongDenom { .. }),
            "Expected WrongDenom, got: {err:?}"
        );
    }

    #[test]
    fn test_submit_requires_claimed_status() {
        let poster = alice();
        let worker = bob();
        let (mut deps, env) =
            setup_contract(vec![poster.to_string(), worker.to_string()]);

        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Submit test".to_string(),
                description: "Must be claimed first".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        // Try to submit without claiming first
        let err = execute(
            deps.as_mut(),
            env,
            message_info(&worker, &[]),
            ExecuteMsg::SubmitTask { task_id: 1 },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::TaskNotClaimed {}),
            "Expected TaskNotClaimed, got: {err:?}"
        );
    }

    #[test]
    fn test_approve_requires_submitted_status() {
        let poster = alice();
        let worker = bob();
        let (mut deps, env) =
            setup_contract(vec![poster.to_string(), worker.to_string()]);

        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&poster, &valid_escrow()),
            ExecuteMsg::PostTask {
                title: "Approve test".to_string(),
                description: "Must be submitted first".to_string(),
                required_skills: vec![],
                expires_in_blocks: None,
            },
        )
        .unwrap();

        // Claim but don't submit
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&worker, &[]),
            ExecuteMsg::ClaimTask { task_id: 1 },
        )
        .unwrap();

        // Try to approve before submit
        let err = execute(
            deps.as_mut(),
            env,
            message_info(&poster, &[]),
            ExecuteMsg::ApproveTask { task_id: 1 },
        )
        .unwrap_err();

        assert!(
            matches!(err, ContractError::TaskNotSubmitted {}),
            "Expected TaskNotSubmitted, got: {err:?}"
        );
    }
}
