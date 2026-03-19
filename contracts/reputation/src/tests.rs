#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
        Addr, Attribute, Empty, Env, OwnedDeps, Uint128,
    };

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use tidepool_types::{
        AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse,
        Skill, SkillInput,
    };

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>;

    fn owner() -> Addr {
        MockApi::default().addr_make("owner")
    }

    fn alice() -> Addr {
        MockApi::default().addr_make("alice")
    }

    fn bob() -> Addr {
        MockApi::default().addr_make("bob")
    }

    fn charlie() -> Addr {
        MockApi::default().addr_make("charlie")
    }

    fn tasks_addr() -> Addr {
        MockApi::default().addr_make("tasks-contract")
    }

    fn arbitrary_addr(seed: &str) -> Addr {
        MockApi::default().addr_make(seed)
    }

    fn setup_contract() -> (TestDeps, Env) {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&owner(), &[]);
        instantiate(deps.as_mut(), env.clone(), info, InstantiateMsg {}).unwrap();
        (deps, env)
    }

    fn register_agent(deps: &mut TestDeps, env: &Env, sender: &Addr, name: &str) {
        let info = message_info(sender, &[]);
        execute(
            deps.as_mut(),
            env.clone(),
            info,
            ExecuteMsg::Register {
                name: name.to_string(),
                skills: vec![],
            },
        )
        .unwrap();
    }

    fn register_agent_with_skills(
        deps: &mut TestDeps,
        env: &Env,
        sender: &Addr,
        name: &str,
        skills: Vec<SkillInput>,
    ) {
        let info = message_info(sender, &[]);
        execute(
            deps.as_mut(),
            env.clone(),
            info,
            ExecuteMsg::Register {
                name: name.to_string(),
                skills,
            },
        )
        .unwrap();
    }

    fn get_agent(deps: &TestDeps, env: &Env, address: &Addr) -> AgentResponse {
        cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::GetAgent {
                    address: address.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn get_config(deps: &TestDeps, env: &Env) -> ReputationConfigResponse {
        cosmwasm_std::from_json(query(deps.as_ref(), env.clone(), QueryMsg::Config {}).unwrap())
            .unwrap()
    }

    #[test]
    fn test_register_agent_happy_path() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let res = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::Register {
                name: "Alice Agent".to_string(),
                skills: vec![
                    SkillInput { name: "rust".to_string(), rating: 4 },
                    SkillInput { name: "cosmwasm".to_string(), rating: 5 },
                ],
            },
        )
        .unwrap();

        assert!(res.attributes.contains(&Attribute::new("method", "register")));
        assert!(res.attributes.contains(&Attribute::new("agent", alice.to_string())));
        assert!(res.attributes.contains(&Attribute::new("name", "Alice Agent")));

        let agent = get_agent(&deps, &env, &alice);
        assert_eq!(agent.name, "Alice Agent");
        assert_eq!(agent.skills.len(), 2);
        assert_eq!(agent.skills[0].name, "rust");
        assert_eq!(agent.skills[0].self_rating, 4);
        assert_eq!(agent.skills[0].jobs_completed, 0);
        assert_eq!(agent.skills[0].total_earned, Uint128::zero());
        assert_eq!(agent.skills[1].name, "cosmwasm");
        assert_eq!(agent.skills[1].self_rating, 5);
        assert_eq!(agent.total_earned, Uint128::zero());
        assert_eq!(agent.total_spent, Uint128::zero());
        assert_eq!(agent.jobs_completed, 0);
        assert_eq!(agent.jobs_posted, 0);
        assert_eq!(agent.registered_at, env.block.height);

        let config = get_config(&deps, &env);
        assert_eq!(config.agent_count, 1);
    }

    #[test]
    fn test_register_invalid_rating_rejected() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::Register {
                name: "Alice".to_string(),
                skills: vec![SkillInput { name: "rust".to_string(), rating: 0 }],
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::InvalidSkillRating { rating: 0 }));

        let err2 = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::Register {
                name: "Alice".to_string(),
                skills: vec![SkillInput { name: "rust".to_string(), rating: 6 }],
            },
        )
        .unwrap_err();
        assert!(matches!(err2, ContractError::InvalidSkillRating { rating: 6 }));
    }

    #[test]
    fn test_register_agent_duplicate_blocked() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        register_agent(&mut deps, &env, &alice, "Alice");

        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::Register {
                name: "Alice 2".to_string(),
                skills: vec![],
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::AlreadyRegistered {}));

        let config = get_config(&deps, &env);
        assert_eq!(config.agent_count, 1);
        assert_eq!(get_agent(&deps, &env, &alice).name, "Alice");
    }

    #[test]
    fn test_update_skills() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        register_agent_with_skills(
            &mut deps,
            &env,
            &alice,
            "Alice",
            vec![SkillInput { name: "rust".to_string(), rating: 3 }],
        );

        // Update existing + add new
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::UpdateSkills {
                skills: vec![
                    SkillInput { name: "rust".to_string(), rating: 5 },
                    SkillInput { name: "typescript".to_string(), rating: 2 },
                ],
            },
        )
        .unwrap();

        let agent = get_agent(&deps, &env, &alice);
        assert_eq!(agent.skills.len(), 2);
        assert_eq!(agent.skills[0].name, "rust");
        assert_eq!(agent.skills[0].self_rating, 5); // updated
        assert_eq!(agent.skills[1].name, "typescript");
        assert_eq!(agent.skills[1].self_rating, 2);
    }

    #[test]
    fn test_update_skills_unregistered_fails() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::UpdateSkills {
                skills: vec![SkillInput { name: "rust".to_string(), rating: 3 }],
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::AgentNotFound {}));
    }

    #[test]
    fn test_owner_can_set_task_contract() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let tasks = tasks_addr();

        let res = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::SetTaskContract {
                address: tasks.to_string(),
            },
        )
        .unwrap();

        assert!(res.attributes.contains(&Attribute::new("method", "set_task_contract")));
        assert_eq!(get_config(&deps, &env).task_contract, Some(tasks));
    }

    #[test]
    fn test_non_owner_cannot_set_task_contract() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let tasks = tasks_addr();

        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&alice, &[]),
            ExecuteMsg::SetTaskContract {
                address: tasks.to_string(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized {}));
        assert!(get_config(&deps, &env).task_contract.is_none());
    }

    #[test]
    fn test_update_volume_by_task_contract() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let bob = bob();
        let tasks = tasks_addr();
        register_agent_with_skills(
            &mut deps,
            &env,
            &alice,
            "Alice",
            vec![
                SkillInput { name: "cosmwasm".to_string(), rating: 5 },
                SkillInput { name: "rust".to_string(), rating: 4 },
            ],
        );
        register_agent(&mut deps, &env, &bob, "Bob");

        // Set task contract
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::SetTaskContract {
                address: tasks.to_string(),
            },
        )
        .unwrap();

        // Update volume with skills (worker=alice, poster=bob)
        let res = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&tasks, &[]),
            ExecuteMsg::UpdateVolume {
                worker: alice.to_string(),
                poster: bob.to_string(),
                amount: Uint128::new(500_000),
                skills: vec!["cosmwasm".to_string()],
            },
        )
        .unwrap();

        assert!(res.attributes.contains(&Attribute::new("method", "update_volume")));
        assert!(res.attributes.contains(&Attribute::new("amount", "500000")));

        let alice_agent = get_agent(&deps, &env, &alice);
        assert_eq!(alice_agent.total_earned, Uint128::new(500_000));
        assert_eq!(alice_agent.jobs_completed, 1);

        // cosmwasm skill should be incremented
        let cosmwasm_skill = alice_agent.skills.iter().find(|s| s.name == "cosmwasm").unwrap();
        assert_eq!(cosmwasm_skill.jobs_completed, 1);
        assert_eq!(cosmwasm_skill.total_earned, Uint128::new(500_000));

        // rust skill should NOT be incremented (not in task skills)
        let rust_skill = alice_agent.skills.iter().find(|s| s.name == "rust").unwrap();
        assert_eq!(rust_skill.jobs_completed, 0);
        assert_eq!(rust_skill.total_earned, Uint128::zero());

        let bob_agent = get_agent(&deps, &env, &bob);
        assert_eq!(bob_agent.total_spent, Uint128::new(500_000));
        assert_eq!(bob_agent.jobs_posted, 1);
    }

    #[test]
    fn test_unauthorized_cannot_update_volume() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        register_agent(&mut deps, &env, &alice, "Alice");
        register_agent(&mut deps, &env, &bob, "Bob");

        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&charlie, &[]),
            ExecuteMsg::UpdateVolume {
                worker: alice.to_string(),
                poster: bob.to_string(),
                amount: Uint128::new(100),
                skills: vec![],
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_update_volume_nonexistent_agent_fails() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let missing = arbitrary_addr("missing");
        register_agent(&mut deps, &env, &alice, "Alice");

        let err = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::UpdateVolume {
                worker: missing.to_string(),
                poster: alice.to_string(),
                amount: Uint128::new(100),
                skills: vec![],
            },
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::AgentNotFound {}));
    }

    #[test]
    fn test_leaderboard_sorts_by_total_earned() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        register_agent(&mut deps, &env, &alice, "Alice");
        register_agent(&mut deps, &env, &bob, "Bob");
        register_agent(&mut deps, &env, &charlie, "Charlie");

        // alice earns 100, bob earns 300, charlie earns 50
        for (worker, poster, amount) in [
            (&alice, &bob, 100u128),
            (&bob, &alice, 300u128),
            (&charlie, &alice, 50u128),
        ] {
            execute(
                deps.as_mut(),
                env.clone(),
                message_info(&owner, &[]),
                ExecuteMsg::UpdateVolume {
                    worker: worker.to_string(),
                    poster: poster.to_string(),
                    amount: Uint128::new(amount),
                    skills: vec![],
                },
            )
            .unwrap();
        }

        let leaderboard: LeaderboardResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::Leaderboard { limit: Some(10), skill: None },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(leaderboard.agents.len(), 3);
        assert_eq!(leaderboard.agents[0].address, bob);
        assert_eq!(leaderboard.agents[0].total_earned, Uint128::new(300));
        assert_eq!(leaderboard.agents[1].address, alice);
        assert_eq!(leaderboard.agents[1].total_earned, Uint128::new(100));
        assert_eq!(leaderboard.agents[2].address, charlie);
        assert_eq!(leaderboard.agents[2].total_earned, Uint128::new(50));
    }

    #[test]
    fn test_leaderboard_by_skill() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let bob = bob();

        register_agent_with_skills(
            &mut deps,
            &env,
            &alice,
            "Alice",
            vec![SkillInput { name: "cosmwasm".to_string(), rating: 5 }],
        );
        register_agent_with_skills(
            &mut deps,
            &env,
            &bob,
            "Bob",
            vec![SkillInput { name: "cosmwasm".to_string(), rating: 3 }],
        );

        // Bob earns more overall, but Alice earns more on cosmwasm jobs
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::UpdateVolume {
                worker: alice.to_string(),
                poster: bob.to_string(),
                amount: Uint128::new(200),
                skills: vec!["cosmwasm".to_string()],
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::UpdateVolume {
                worker: bob.to_string(),
                poster: alice.to_string(),
                amount: Uint128::new(100),
                skills: vec!["cosmwasm".to_string()],
            },
        )
        .unwrap();

        let lb: LeaderboardResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::Leaderboard {
                    limit: Some(10),
                    skill: Some("cosmwasm".to_string()),
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(lb.agents.len(), 2);
        assert_eq!(lb.agents[0].address, alice); // 200 earned in cosmwasm
        assert_eq!(lb.agents[1].address, bob);   // 100 earned in cosmwasm
    }

    #[test]
    fn test_get_agents_by_skill() {
        let (mut deps, env) = setup_contract();
        let alice = alice();
        let bob = bob();
        let charlie = charlie();

        register_agent_with_skills(
            &mut deps,
            &env,
            &alice,
            "Alice",
            vec![
                SkillInput { name: "rust".to_string(), rating: 5 },
                SkillInput { name: "cosmwasm".to_string(), rating: 4 },
            ],
        );
        register_agent_with_skills(
            &mut deps,
            &env,
            &bob,
            "Bob",
            vec![SkillInput { name: "rust".to_string(), rating: 2 }],
        );
        register_agent_with_skills(
            &mut deps,
            &env,
            &charlie,
            "Charlie",
            vec![SkillInput { name: "typescript".to_string(), rating: 5 }],
        );

        // All rust agents
        let result: AgentsListResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::GetAgentsBySkill {
                    skill: "rust".to_string(),
                    min_rating: None,
                    limit: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result.agents.len(), 2);

        // Rust agents with min_rating 4
        let result2: AgentsListResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::GetAgentsBySkill {
                    skill: "rust".to_string(),
                    min_rating: Some(4),
                    limit: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result2.agents.len(), 1);
        assert_eq!(result2.agents[0].address, alice);

        // No agents with solidity
        let result3: AgentsListResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::GetAgentsBySkill {
                    skill: "solidity".to_string(),
                    min_rating: None,
                    limit: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result3.agents.len(), 0);
    }

    #[test]
    fn test_leaderboard_respects_limit() {
        let (mut deps, env) = setup_contract();
        for i in 0..5 {
            let addr = arbitrary_addr(&format!("agent-{i}"));
            register_agent(&mut deps, &env, &addr, &format!("agent-{i}"));
        }

        let leaderboard: LeaderboardResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env,
                QueryMsg::Leaderboard { limit: Some(2), skill: None },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(leaderboard.agents.len(), 2);
    }

    #[test]
    fn test_list_agents_pagination() {
        let (mut deps, env) = setup_contract();
        for i in 0..5 {
            let addr = arbitrary_addr(&format!("page-agent-{i}"));
            register_agent(&mut deps, &env, &addr, &format!("agent-{i}"));
        }

        let page1: AgentsListResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env.clone(),
                QueryMsg::ListAgents {
                    start_after: None,
                    limit: Some(2),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(page1.agents.len(), 2);

        let last_addr = page1.agents.last().unwrap().address.to_string();
        let page2: AgentsListResponse = cosmwasm_std::from_json(
            query(
                deps.as_ref(),
                env,
                QueryMsg::ListAgents {
                    start_after: Some(last_addr),
                    limit: Some(2),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(page2.agents.len(), 2);
        assert_ne!(page1.agents[0].address, page2.agents[0].address);
    }

    #[test]
    fn test_query_config_reflects_state() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let tasks = tasks_addr();

        let config = get_config(&deps, &env);
        assert_eq!(config.owner, owner);
        assert!(config.task_contract.is_none());
        assert_eq!(config.agent_count, 0);

        register_agent(&mut deps, &env, &alice, "Alice");
        execute(
            deps.as_mut(),
            env.clone(),
            message_info(&owner, &[]),
            ExecuteMsg::SetTaskContract {
                address: tasks.to_string(),
            },
        )
        .unwrap();

        let config2 = get_config(&deps, &env);
        assert_eq!(config2.agent_count, 1);
        assert_eq!(config2.task_contract, Some(tasks));
    }

    #[test]
    fn test_cumulative_volume_tracking() {
        let (mut deps, env) = setup_contract();
        let owner = owner();
        let alice = alice();
        let bob = bob();
        register_agent_with_skills(
            &mut deps,
            &env,
            &alice,
            "Alice",
            vec![SkillInput { name: "cosmwasm".to_string(), rating: 5 }],
        );
        register_agent(&mut deps, &env, &bob, "Bob");

        // Multiple settlements
        for _ in 0..3 {
            execute(
                deps.as_mut(),
                env.clone(),
                message_info(&owner, &[]),
                ExecuteMsg::UpdateVolume {
                    worker: alice.to_string(),
                    poster: bob.to_string(),
                    amount: Uint128::new(200_000),
                    skills: vec!["cosmwasm".to_string()],
                },
            )
            .unwrap();
        }

        let alice_agent = get_agent(&deps, &env, &alice);
        assert_eq!(alice_agent.total_earned, Uint128::new(600_000));
        assert_eq!(alice_agent.jobs_completed, 3);

        let cosmwasm_skill = alice_agent.skills.iter().find(|s| s.name == "cosmwasm").unwrap();
        assert_eq!(cosmwasm_skill.jobs_completed, 3);
        assert_eq!(cosmwasm_skill.total_earned, Uint128::new(600_000));

        let bob_agent = get_agent(&deps, &env, &bob);
        assert_eq!(bob_agent.total_spent, Uint128::new(600_000));
        assert_eq!(bob_agent.jobs_posted, 3);
    }
}
