#![cfg(test)]

// Unit tests for the rps (Rock Paper Scissors) contract using a simple mock GameHub.

use crate::{Error, PlayerMove, RpsContract, RpsContractClient};
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

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

    pub fn end_game(_env: Env, _session_id: u32, _player1_won: bool) {}

    pub fn add_game(_env: Env, _game_address: Address) {}
}

// ============================================================================
// Test Helpers
// ============================================================================

fn setup_test() -> (
    Env,
    RpsContractClient<'static>,
    MockGameHubClient<'static>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set(soroban_sdk::testutils::LedgerInfo {
        timestamp: 1441065600,
        protocol_version: 25,
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
    let contract_id = env.register(RpsContract, (&admin, &hub_addr));
    let client = RpsContractClient::new(&env, &contract_id);

    game_hub.add_game(&contract_id);

    let player1 = Address::generate(&env);
    let player2 = Address::generate(&env);

    (env, client, game_hub, player1, player2)
}

fn assert_rps_error<T, E>(
    result: &Result<Result<T, E>, Result<Error, soroban_sdk::InvokeError>>,
    expected_error: Error,
) {
    match result {
        Err(Ok(actual_error)) => {
            assert_eq!(*actual_error, expected_error);
        }
        _ => panic!("Expected error {:?}, got something else", expected_error),
    }
}

// ============================================================================
// Basic Game Flow Tests
// ============================================================================

#[test]
fn test_complete_game() {
    let (_env, client, _hub, player1, player2) = setup_test();

    let session_id = 1u32;
    let points = 100_0000000;

    client.start_game(&session_id, &player1, &player2, &points, &points);

    let game = client.get_game(&session_id);
    assert_eq!(game.player1_move, PlayerMove::None);
    assert_eq!(game.player2_move, PlayerMove::None);
    assert!(game.winner.is_none());
    assert_eq!(game.player1, player1);
    assert_eq!(game.player2, player2);

    client.submit_move(&session_id, &player1, &PlayerMove::Rock);
    client.submit_move(&session_id, &player2, &PlayerMove::Scissors);

    let winner = client.reveal_winner(&session_id);
    assert_eq!(winner, Some(player1.clone())); // Rock beats Scissors

    let final_game = client.get_game(&session_id);
    assert_eq!(final_game.winner.unwrap(), player1);
}

#[test]
fn test_multiple_sessions() {
    let (env, client, _hub, player1, player2) = setup_test();
    let player3 = Address::generate(&env);
    let player4 = Address::generate(&env);

    client.start_game(&3u32, &player1, &player2, &100_0000000, &100_0000000);
    client.start_game(&4u32, &player3, &player4, &50_0000000, &50_0000000);

    assert_eq!(client.get_game(&3u32).player1, player1);
    assert_eq!(client.get_game(&4u32).player1, player3);
}

// ============================================================================
// RPS Logic Tests
// ============================================================================

#[test]
fn test_rock_beats_scissors() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&1u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&1u32, &player1, &PlayerMove::Rock);
    client.submit_move(&1u32, &player2, &PlayerMove::Scissors);
    assert_eq!(client.reveal_winner(&1u32), Some(player1.clone()));
}

#[test]
fn test_scissors_beats_paper() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&2u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&2u32, &player1, &PlayerMove::Scissors);
    client.submit_move(&2u32, &player2, &PlayerMove::Paper);
    assert_eq!(client.reveal_winner(&2u32), Some(player1.clone()));
}

#[test]
fn test_paper_beats_rock() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&3u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&3u32, &player1, &PlayerMove::Paper);
    client.submit_move(&3u32, &player2, &PlayerMove::Rock);
    assert_eq!(client.reveal_winner(&3u32), Some(player1.clone()));
}

#[test]
fn test_player2_wins() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&4u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&4u32, &player1, &PlayerMove::Scissors);
    client.submit_move(&4u32, &player2, &PlayerMove::Rock); // Rock beats Scissors
    assert_eq!(client.reveal_winner(&4u32), Some(player2.clone()));
}

#[test]
fn test_tie_resets_moves() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&5u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&5u32, &player1, &PlayerMove::Rock);
    client.submit_move(&5u32, &player2, &PlayerMove::Rock);
    // Tie: returns None and resets moves
    assert_eq!(client.reveal_winner(&5u32), None);
    let game = client.get_game(&5u32);
    assert_eq!(game.player1_move, PlayerMove::None);
    assert_eq!(game.player2_move, PlayerMove::None);
    assert!(game.winner.is_none());
    // Players can now submit again
    client.submit_move(&5u32, &player1, &PlayerMove::Rock);
    client.submit_move(&5u32, &player2, &PlayerMove::Scissors);
    assert_eq!(client.reveal_winner(&5u32), Some(player1.clone()));
}

#[test]
fn test_all_tie_variants_reset_moves() {
    let (_env, client, _hub, player1, player2) = setup_test();

    for (id, m) in [(6u32, PlayerMove::Rock), (7u32, PlayerMove::Paper), (8u32, PlayerMove::Scissors)] {
        client.start_game(&id, &player1, &player2, &100_0000000, &100_0000000);
        client.submit_move(&id, &player1, &m.clone());
        client.submit_move(&id, &player2, &m);
        assert_eq!(client.reveal_winner(&id), None);
        let game = client.get_game(&id);
        assert_eq!(game.player1_move, PlayerMove::None);
        assert_eq!(game.player2_move, PlayerMove::None);
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_cannot_move_twice() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&10u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&10u32, &player1, &PlayerMove::Rock);
    let result = client.try_submit_move(&10u32, &player1, &PlayerMove::Paper);
    assert_rps_error(&result, Error::AlreadyMoved);
}

#[test]
fn test_cannot_reveal_before_both_move() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&11u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&11u32, &player1, &PlayerMove::Rock);
    let result = client.try_reveal_winner(&11u32);
    assert_rps_error(&result, Error::BothPlayersNotMoved);
}

#[test]
fn test_non_player_cannot_move() {
    let (env, client, _hub, player1, player2) = setup_test();
    let outsider = Address::generate(&env);
    client.start_game(&12u32, &player1, &player2, &100_0000000, &100_0000000);
    let result = client.try_submit_move(&12u32, &outsider, &PlayerMove::Rock);
    assert_rps_error(&result, Error::NotPlayer);
}

#[test]
fn test_cannot_move_after_game_ended() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&13u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&13u32, &player1, &PlayerMove::Rock);
    client.submit_move(&13u32, &player2, &PlayerMove::Scissors);
    client.reveal_winner(&13u32);
    let result = client.try_submit_move(&13u32, &player1, &PlayerMove::Paper);
    assert_rps_error(&result, Error::GameAlreadyEnded);
}

#[test]
fn test_cannot_reveal_nonexistent_game() {
    let (_env, client, _hub, _p1, _p2) = setup_test();
    let result = client.try_reveal_winner(&999u32);
    assert_rps_error(&result, Error::GameNotFound);
}

#[test]
fn test_reveal_twice_is_idempotent() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&14u32, &player1, &player2, &100_0000000, &100_0000000);
    client.submit_move(&14u32, &player1, &PlayerMove::Rock);
    client.submit_move(&14u32, &player2, &PlayerMove::Scissors);
    let winner1 = client.reveal_winner(&14u32);
    let winner2 = client.reveal_winner(&14u32);
    assert_eq!(winner1, Some(player1.clone()));
    assert_eq!(winner1, winner2);
}

// ============================================================================
// Multiple Games Tests
// ============================================================================

#[test]
fn test_multiple_games_independent() {
    let (env, client, _hub, player1, player2) = setup_test();
    let player3 = Address::generate(&env);
    let player4 = Address::generate(&env);

    client.start_game(&20u32, &player1, &player2, &100_0000000, &100_0000000);
    client.start_game(&21u32, &player3, &player4, &50_0000000, &50_0000000);

    client.submit_move(&20u32, &player1, &PlayerMove::Rock);
    client.submit_move(&21u32, &player3, &PlayerMove::Paper);
    client.submit_move(&20u32, &player2, &PlayerMove::Scissors);
    client.submit_move(&21u32, &player4, &PlayerMove::Rock);

    let winner1 = client.reveal_winner(&20u32);
    let winner2 = client.reveal_winner(&21u32);

    assert_eq!(winner1, Some(player1.clone())); // Rock beats Scissors
    assert_eq!(winner2, Some(player3.clone())); // Paper beats Rock
}

#[test]
fn test_asymmetric_points() {
    let (_env, client, _hub, player1, player2) = setup_test();
    client.start_game(&15u32, &player1, &player2, &200_0000000, &50_0000000);
    let game = client.get_game(&15u32);
    assert_eq!(game.player1_points, 200_0000000);
    assert_eq!(game.player2_points, 50_0000000);
    client.submit_move(&15u32, &player1, &PlayerMove::Rock);
    client.submit_move(&15u32, &player2, &PlayerMove::Scissors); // Rock beats Scissors
    assert_eq!(client.reveal_winner(&15u32), Some(player1.clone()));
    assert!(client.get_game(&15u32).winner.is_some());
}

// ============================================================================
// Admin Function Tests
// ============================================================================

#[test]
fn test_upgrade_function_exists() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let hub_addr = env.register(MockGameHub, ());
    let contract_id = env.register(RpsContract, (&admin, &hub_addr));
    let client = RpsContractClient::new(&env, &contract_id);

    let new_wasm_hash = BytesN::from_array(&env, &[1u8; 32]);
    let result = client.try_upgrade(&new_wasm_hash);
    assert!(result.is_err());
}

