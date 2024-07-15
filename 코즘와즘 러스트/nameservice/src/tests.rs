#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_json, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
    use crate::state::Config;

    // --- initiate
    #[test]
    fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: None,
                transfer_price: None,
            },
        );
    }

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: Some(coin(3, "token")),
                transfer_price: Some(coin(4, "token")),
            },
        );
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_json(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn mock_init_with_price(deps: DepsMut, purchase_price: Coin, transfer_price: Coin) {
        let msg = InstantiateMsg {
            purchase_price: Some(purchase_price),
            transfer_price: Some(transfer_price),
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");

        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: None,
                transfer_price: None,
            },
        );
    }

    fn mock_init_no_price(deps: DepsMut) {
        let msg = InstantiateMsg {
            purchase_price: None,
            transfer_price: None,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }
    // --- end

    // --- register
    #[test]
    fn register_available_name_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // then
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    #[test]
    fn register_available_name_and_query_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        let info = mock_info("bob_key", &coins(5, "token"));
        let msg = ExecuteMsg::Register {
            name: "bob".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");

        // then
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
        assert_name_owner(deps.as_ref(), "bob", "bob_key");
    }

    #[test]
    fn fails_on_register_already_taken_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);
        
        // "alice" 이름 중복 
        let info = mock_info("bob_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
        
        // "alice" 이름 중복 
        let info = mock_info("alice_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    fn mock_alice_registers_name(deps: DepsMut, sent: &[Coin]) { 
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }

    fn assert_name_owner(deps: Deps, name: &str, owner: &str) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::ResolveRecord {
                name: name.to_string(),
            },
        )
        .unwrap();

        let value: ResolveRecordResponse = from_json(&res).unwrap();
        assert_eq!(Some(owner.to_string()), value.address);
    }

    // --- register: 수수료 검증
    #[test]
    fn fails_on_register_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // 금액 미입력
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn fails_on_register_wrong_fee_denom() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // denom이 다른 경우 
        let info = mock_info("alice_key", &coins(2, "earth"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
    // --- end
    
    // --- register: name 입력 데이터 검증
    #[test]
    fn register_available_name_fails_with_invalid_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        let info = mock_info("bob_key", &coins(2, "token"));

        // hi is too short
        let msg = ExecuteMsg::Register {
            name: "hi".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooShort { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // 65 chars is too long
        let msg = ExecuteMsg::Register {
            name: "01234567890123456789012345678901234567890123456789012345678901234".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooLong { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // no upper case...
        let msg = ExecuteMsg::Register {
            name: "LOUD".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { c }) => assert_eq!(c, 'L'),
            Err(_) => panic!("Unknown error"),
        }
        // ... or spaces
        let msg = ExecuteMsg::Register {
            name: "two words".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
    }
    // --- end

    // --- transfer 
    #[test]
    fn transfer_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // alice_key 보유자가 'alice'를 bob_key로 전송
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        
        // then
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }

    #[test]
    fn transfer_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        // alice_key 보유자가 'alice'를 bob_key로 전송
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        
        // then
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }

    #[test]
    fn fails_on_transfer_non_existent() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // frank_key 보유자가 등록되지 않은 'alice42'를 bob_key로 전송
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::Transfer {
            name: "alice42".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        // then
        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameNotExists { name }) => assert_eq!(name, "alice42"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    #[test]
    fn fails_on_transfer_from_nonowner() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // frank_key 보유자가 자신 보유하고 있지 않은 'alice'를 bob_key로 전송
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        // then
        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::Unauthorized { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    #[test]
    fn fails_on_transfer_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(5, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        // alice_key 보유자가 'alice'를 bob_key로 충분하지 않은 금액과 함께 전송
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        // then
        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    // --- end 

    #[test]
    fn returns_empty_on_query_unregistered_name() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        // querying for unregistered name results in NotFound error
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ResolveRecord {
                name: "alice".to_string(),
            },
        )
        .unwrap();
        let value: ResolveRecordResponse = from_json(&res).unwrap();
        assert_eq!(None, value.address);
    }

}