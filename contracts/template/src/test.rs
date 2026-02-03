#![cfg(test)]

use crate::{Error, TemplateGameContract, TemplateGameContractClient};
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{contract, contractimpl, Address, Env};

// ============================================================================
// Mock GameHub for Unit Testing
// ============================================================================

#[contract]
pub struct MockGameHub;

#[contractimpl]
impl MockGameHub {
    pub fn start_game(
        _env: Env,
        _game_id: Address,
        _session_id: u32,
        _player1: Address,
        _player2: Address,
        _player1_points: i128,
        _player2_points: i128,
    ) {
    }

    pub fn end_game(_env: Env, _session_id: u32, _player1_won: bool) {
    }

    pub fn add_game(_env: Env, _game_address: Address) {
    }
}

// ============================================================================
// Test Helpers
// ============================================================================

fn setup_test() -> (
    Env,
    TemplateGameContractClient<'static>,
    MockGameHubClient<'static>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set(soroban_sdk::testutils::LedgerInfo {
        timestamp: 1441065600,
        protocol_version: 23,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: u32::MAX / 2,
        min_persistent_entry_ttl: u32::MAX / 2,
        max_entry_ttl: u32::MAX / 2,
    });

    let hub_addr = env.register(MockGameHub, ());
    let game_hub = MockGameHubClient::new(&env, &hub_addr);

    let admin = Address::generate(&env);
    let contract_id = env.register(TemplateGameContract, (&admin, &hub_addr));
    let client = TemplateGameContractClient::new(&env, &contract_id);

    game_hub.add_game(&contract_id);

    let player1 = Address::generate(&env);
    let player2 = Address::generate(&env);

    (env, client, game_hub, player1, player2)
}

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_start_and_finish_game() {
    let (_env, client, _hub, player1, player2) = setup_test();
    let session_id = 7u32;

    client
        .start_game(&session_id, &player1, &player2, &10, &12)
        .unwrap();

    let game = client.get_game(&session_id).unwrap();
    assert_eq!(game.player1, player1);
    assert_eq!(game.player2, player2);
    assert_eq!(game.winner, None);

    client.finish_game(&session_id, &player1, &true).unwrap();

    let finished = client.get_game(&session_id).unwrap();
    assert_eq!(finished.winner, Some(player1));
}

#[test]
fn test_finish_game_requires_existing_session() {
    let (_env, client, _hub, player1, _player2) = setup_test();
    let result = client.try_finish_game(&42, &player1, &true);

    match result {
        Err(Ok(Error::GameNotFound)) => {}
        _ => panic!("Expected GameNotFound error"),
    }
}
