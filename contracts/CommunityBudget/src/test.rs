#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

#[test]
fn test_happy_path_send_and_claim() {
    // Test 1: Happy path — full end-to-end MVP transaction.
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RemitNow);
    let client = RemitNowClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.send_remittance(&sender, &recipient, &1000i128);
    let claimed = client.claim_remittance(&recipient);

    assert_eq!(claimed, 1000);
}

#[test]
#[should_panic(expected = "no funds to claim")]
fn test_claim_with_zero_balance_fails() {
    // Test 2: Edge case — claiming with no prior remittance must fail.
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RemitNow);
    let client = RemitNowClient::new(&env, &contract_id);

    let recipient = Address::generate(&env);
    client.claim_remittance(&recipient);
}

#[test]
fn test_state_reflects_balance_after_send() {
    // Test 3: State verification — storage correctly reflects balance after send.
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RemitNow);
    let client = RemitNowClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.send_remittance(&sender, &recipient, &500i128);
    let balance = client.get_balance(&recipient);

    assert_eq!(balance, 500);
}