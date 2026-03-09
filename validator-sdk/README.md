# Vexidus Validator SDK

Everything you need to run a validator node and manage staking on Vexidus.

## Contents

- [Setup Guide](#setup-guide) — Run a validator node
- [Staking SDK](#staking-sdk) — Delegation, rewards, commission
- [Genesis Files](./genesis/) — Network genesis configuration

## Setup Guide

### Requirements

- **OS**: Ubuntu 22.04+ (or any Linux with glibc 2.35+)
- **CPU**: 4+ cores
- **RAM**: 8 GB minimum, 16 GB recommended
- **Disk**: 100 GB SSD (NVMe recommended)
- **Network**: 100 Mbps, static IP, ports 9945-9946 open

### Install

```bash
# Download the latest binary
curl -LO https://vexidus.io/releases/vexidus-node
chmod +x vexidus-node

# Generate validator keypair
./vexidus-node keygen --output validator-key.json

# Start node (testnet)
./vexidus-node \
  --data-dir ./data \
  --rpc-port 9933 \
  --p2p-port 9946 \
  --bootnodes /ip4/51.255.80.34/udp/9945/quic-v1/p2p/12D3KooWDoZq8QTmASfidE9aKYqLcnabEKkmARdSnuZmkqJwxKHe \
  --validator-key validator-key.json
```

### systemd Service

```ini
[Unit]
Description=Vexidus Validator Node
After=network.target

[Service]
Type=simple
User=vexidus
ExecStart=/opt/vexidus/bin/vexidus-node \
  --data-dir /opt/vexidus/data \
  --rpc-port 9933 \
  --p2p-port 9946 \
  --bootnodes /ip4/51.255.80.34/udp/9945/quic-v1/p2p/12D3KooWDoZq8QTmASfidE9aKYqLcnabEKkmARdSnuZmkqJwxKHe \
  --validator-key /opt/vexidus/keys/validator-key.json
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

### Register as Validator

```bash
vexidus validator register \
  --from <your-pubkey> \
  --stake 10000000000000 \
  --commission 1000 \
  --rpc http://localhost:9933
```

Minimum stake: 1,000 VXS. Commission: basis points (1000 = 10%).

## Staking SDK

```rust
use vexidus_sdk::{ValidatorClient, ValidatorKeypair, ValidatorConfig};

// Load validator keypair
let keypair = ValidatorKeypair::from_file("validator-key.json")?;

// Connect
let client = ValidatorClient::new("https://testnet.vexidus.io");

// Check validator status
let status = client.get_validator(&keypair.public_key()).await?;
println!("Stake: {} VXS", status.self_stake);
println!("Delegated: {} VXS", status.delegated_stake);
println!("Reputation: {} ({})", status.reputation, status.reputation_grade);
println!("Blocks produced: {}", status.blocks_produced);

// Claim rewards
let bundle = BundleBuilder::new()
    .claim_rewards()
    .sign(&keypair)?
    .build();
client.submit_bundle(&bundle).await?;
```

## Staking Operations

| Operation | CLI | Description |
|-----------|-----|-------------|
| Register | `validator register` | Join the validator set |
| Delegate | `validator delegate` | Stake tokens to a validator |
| Undelegate | `validator undelegate` | Begin 21-day unbonding |
| Claim Rewards | `validator claim-rewards` | Withdraw pending rewards |
| Set Commission | `validator set-commission` | Update commission rate (7-day cooldown) |
| Set Metadata | `validator set-metadata` | Update name, description, website |

## Network Parameters

| Parameter | Value |
|-----------|-------|
| Minimum stake | 1,000 VXS |
| Unbonding period | 21 days |
| Commission range | 0–50% |
| Commission cooldown | 7 days |
| Reward split | 85% validator / 15% Foundation |
| Fee split | 80% proposer / 20% Foundation |
| Jailing | Escalating: 1h → 24h → 7d → 30d → permanent |

## Seed Nodes

| Region | Address |
|--------|---------|
| EU (Gravelines) | `/ip4/51.255.80.34/udp/9945/quic-v1/p2p/12D3KooWDoZq8QTmASfidE9aKYqLcnabEKkmARdSnuZmkqJwxKHe` |
| NA (Beauharnois) | `/ip4/158.69.203.54/udp/9945/quic-v1/p2p/<peer-id>` |
| SG (Singapore) | `/ip4/205.198.87.150/udp/9945/quic-v1/p2p/<peer-id>` |
