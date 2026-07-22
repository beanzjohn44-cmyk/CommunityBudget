# RemitNow

Instant, low-fee remittances from overseas workers to their families, settled on Stellar.

## Problem
Marites, a domestic worker from Iloilo, Philippines working in Dubai, sends money home through a remittance agent that takes 3 days and charges a 7% fee plus a bad exchange rate, costing her family roughly $28 out of every $400 sent.

## Solution
Marites sends USDC from her Stellar wallet in Dubai. A Soroban contract instantly credits a claimable balance to her sister's Stellar address in Iloilo, who claims it and cashes out through a local anchor in under a minute. Stellar settles in ~5 seconds for a fraction of a cent, with no correspondent-banking delay or spread.

## Timeline (Bootcamp)
- Day 1: Contract design + `send_remittance` / `claim_remittance` implementation
- Day 2: Tests, wallet integration (Freighter), local anchor mock for cash-out
- Day 3: Demo polish, deploy to testnet, pitch deck

## Stellar Features Used
- USDC transfers
- Soroban smart contracts
- Trustlines

## Vision and Purpose
Migrant remittances are one of the largest cross-border payment flows in the world, and the fees extracted from low-income senders are disproportionately high. RemitNow shows that a two-function Soroban contract can replace days of delay and single-digit-percent fees with near-instant, near-free settlement — a direct, measurable improvement to family income for millions of migrant workers.

## Prerequisites
- Rust (stable, 1.74+)
- Soroban CLI (`stellar-cli`) v21+
- A funded Stellar testnet account (via [Friendbot](https://friendbot.stellar.org))

## How to Build
```bash
soroban contract build
```

## How to Test
```bash
cargo test
```

## How to Deploy to Testnet
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/remit_now.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

## Sample CLI Invocation
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <SENDER_SECRET_KEY> \
  --network testnet \
  -- \
  send_remittance \
  --sender <SENDER_PUBLIC_KEY> \
  --recipient <RECIPIENT_PUBLIC_KEY> \
  --amount 1000
```
link 
https://stellar.expert/explorer/testnet/tx/77511a1eeaef1426bfc16f9aa230b4702a2627e6dcb921f4642d81381fbc40d9
https://lab.stellar.org/r/testnet/contract/CAIW3CRZ6XFA43QZT7J5TU6V2IEXJLOK7UHIKQNQI4Z3IZV2VYWJFV4U
## License
MIT