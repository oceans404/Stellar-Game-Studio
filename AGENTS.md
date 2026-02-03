# AGENTS.md

This repo is the Stellar Game Studio. Use this guide when creating new games.

## Quick Principles
- Follow the existing game patterns in `contracts/number-guess` and `contracts/twenty-one`.
- All games must call `start_game` and `end_game` on the Game Hub contract.
- Keep randomness deterministic between simulation and submission (do not use ledger time/sequence).
- Prefer temporary storage with a 30-day TTL for game state.

## Create Command
- Run `bun run create <game-name>` to scaffold a new contract and standalone frontend.
- It copies `contracts/template` into `contracts/<game-name>` and updates the package name.
- It adds `contracts/<game-name>` to the root `Cargo.toml` workspace members.
- It generates a standalone React app in `<game-name>-frontend/` for the new game.

## Contract Checklist (Soroban)
1. Review `contracts/<game-name>/Cargo.toml` and confirm the package name.
2. Implement the required Game Hub client interface:
   ```rust
   #[contractclient(name = "GameHubClient")]
   pub trait GameHub {
       fn start_game(
           env: Env,
           game_id: Address,
           session_id: u32,
           player1: Address,
           player2: Address,
           player1_points: i128,
           player2_points: i128,
       );

       fn end_game(env: Env, session_id: u32, player1_won: bool);
   }
   ```
3. Require player auth on `start_game` using `require_auth_for_args` for points.
4. Create/track game state in temporary storage and extend TTL.
5. Call `game_hub.start_game(...)` in `start_game` and `game_hub.end_game(...)` in your game-end path.
6. Add tests in `contracts/<game-name>/src/test.rs` (use the mock Game Hub pattern).

## Workspace Wiring
- The create script adds the contract to the workspace members list in root `Cargo.toml`.
- If you add contracts manually, update the list explicitly:
  ```toml
  members = [
    "contracts/mock-game-hub",
    "contracts/twenty-one",
    "contracts/number-guess",
    "contracts/<game-name>",
  ]
  ```

## Frontend Checklist (Standalone)
1. Update `<game-name>-frontend/src/games/<game-name>/` (UI + service files created by the scaffold).
2. Replace `<game-name>-frontend/src/games/<game-name>/bindings.ts` with generated bindings.
3. Set the contract ID in `<game-name>-frontend/public/game-studio-config.js`.
4. Run the standalone app:
   ```bash
   cd <game-name>-frontend && bun run dev
   ```

## Frontend Checklist (Studio Catalog, Optional)
1. Copy the game module into the studio frontend:
   ```bash
   cp -r <game-name>-frontend/src/games/<game-name> sgs_frontend/src/games/
   ```
2. Add routing + a card entry in `sgs_frontend/src/components/GamesCatalog.tsx`.
3. Update `sgs_frontend/src/utils/constants.ts` with a named export if needed.
4. Update `sgs_frontend/src/config.ts` with backwards-compatible aliases if needed.
5. Use the existing service pattern from `number-guess` for multi-sig game creation.

## Bindings Generation
- Build the contract:
  ```bash
  stellar contract build --manifest-path contracts/<game-name>/Cargo.toml
  ```
- Generate TS bindings from the WASM:
  ```bash
  stellar contract bindings typescript \
    --wasm target/wasm32v1-none/release/<game_name>.wasm \
    --output-dir bindings/<game_name> \
    --overwrite
  ```
- Copy the generated `bindings/<game_name>/src/index.ts` into:
  `<game-name>-frontend/src/games/<game-name>/bindings.ts` (and optionally into `sgs_frontend/src/games/<game-name>/bindings.ts`).

## Deployment / Local Testing
- Build all contracts:
  ```bash
  bun run build
  ```
- Deploy to testnet and generate bindings:
  ```bash
  bun run deploy
  bun run bindings
  ```
- Start the studio frontend:
  ```bash
  cd sgs_frontend && bun run dev
  ```

## Game UX Guidelines
- Provide clear create/import/load flows (see `NumberGuessGame.tsx`).
- Include a lightweight animation or visual feedback for key actions.
- Show points, player addresses (shortened), and win/lose state.

## Final QA Checklist
- Contract builds successfully.
- `start_game` + `end_game` are called.
- Standalone frontend connects to the correct contract ID.
- Game card appears in the studio catalog (if imported).
- Both players can complete a full game flow.
