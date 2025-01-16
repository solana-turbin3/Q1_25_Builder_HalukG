# Prerequisites Tasks

This folder contains the prerequisite tasks for the Turbin3 Q1 2024 Cohort.

## TypeScript Task

A series of scripts to interact with Solana's devnet, demonstrating basic blockchain interactions.

## Typescript-enrollment

### Scripts

1. `keygen.ts`: Generates a new Solana keypair
2. `airdrop.ts`: Requests devnet SOL tokens
3. `transfer.ts`: Transfers SOL between wallets
4. `enroll.ts`: Interacts with the Turbin3 prerequisite program

### Usage

```
yarn ts-node keygen.ts     # Generate keypair
yarn ts-node airdrop.ts    # Request SOL
yarn ts-node transfer.ts   # Transfer SOL
yarn ts-node enroll.ts     # Complete enrollment
```

## Rust-enrollment

### Scripts

1. `lib.rs`: Includes the code for keygen, airdrop, transfer and enroll

### Usage

```
cargo build

cargo test
```