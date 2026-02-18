# RPS Deployment Instructions

## Current Status

| Network | Contract ID |
|---------|-------------|
| **Testnet** ✅ | `CC6IZLDV5HU3IBJB2LYXLHTF3EWS7LKJHLARZZ2F7S34ZB5SOO4T42AS` |
| Mainnet | Not yet deployed |

Game Hub (testnet mock): `CB4VZAT2U3UC6XFK3N23SKRF2NDCMP3QHJYMCHHFMZO7MRQO6DQ2EMYG`

---

## Testnet (already done)

To redeploy to testnet (e.g. after contract changes):

```bash
bun run build rps
bun run deploy rps
bun run bindings rps
```

Then copy the new bindings:
```bash
cp bindings/rps/src/index.ts rps-frontend/src/games/rps/bindings.ts
```

---

## Mainnet

### Step 1 — Build the contract ✅

```bash
bun run build rps
```

### Step 2 — Generate admin keypair ✅

```bash
stellar keys generate rps-admin --network mainnet
stellar keys address rps-admin
```

**Admin address:** `GDCORDHPHN46OUNJZQ2V7ZZGN3QLYH6LSZY7C2YGE376S2KMRAY7FMXJ`

### Step 3 — Upload the wasm ✅

```bash
stellar contract upload \
  --wasm target/wasm32v1-none/release/rps.wasm \
  --source-account rps-admin \
  --rpc-url https://rpc.ankr.com/stellar_soroban \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

**Wasm hash:** `feee457c765f3523b252fc90254f746b6c1cf491f659f577163e11527f200354`

### Step 4 — Deploy the contract ⏳ (waiting on mainnet Game Hub ID)

Once you have `GAME_HUB_MAINNET_CONTRACT_ID`:

```bash
stellar contract deploy \
  --wasm-hash feee457c765f3523b252fc90254f746b6c1cf491f659f577163e11527f200354 \
  --source-account rps-admin \
  --rpc-url https://rpc.ankr.com/stellar_soroban \
  --network-passphrase "Public Global Stellar Network ; September 2015" \
  -- \
  --admin GDCORDHPHN46OUNJZQ2V7ZZGN3QLYH6LSZY7C2YGE376S2KMRAY7FMXJ \
  --game-hub GAME_HUB_MAINNET_CONTRACT_ID
```

Note the `CONTRACT_ID` output.

### Step 4 — Register with Game Hub (admin only)

The mainnet Game Hub only accepts outcomes from approved contracts. Ask the Game Hub admin to run:

```bash
stellar contract invoke \
  --id GAME_HUB_MAINNET_CONTRACT_ID \
  --source-account GAME_HUB_ADMIN_SECRET \
  --network mainnet \
  -- \
  add_game \
  --game_id YOUR_MAINNET_CONTRACT_ID \
  --developer YOUR_ADDRESS
```

### Step 5 — Build the production frontend

```bash
bun run publish rps --build
```

Output is in `dist/rps-frontend/`.

### Step 6 — Configure runtime settings

Edit `dist/rps-frontend/public/game-studio-config.js`:

```js
window.__STELLAR_GAME_STUDIO_CONFIG__ = {
  rpcUrl: "https://soroban-mainnet.stellar.org",
  networkPassphrase: "Public Global Stellar Network ; September 2015",
  contractIds: {
    "rps": "YOUR_MAINNET_CONTRACT_ID"
  },
  simulationSourceAddress: ""
};
```

### Step 7 — Deploy the frontend

The `dist/rps-frontend/` folder is a static site. Deploy to any static host:

```bash
# Vercel
vercel --prod

# Cloudflare Pages
wrangler pages deploy dist/rps-frontend
```
