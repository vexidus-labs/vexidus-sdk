# Vexidus Validator SDK

> 🛡️ **BETA VALIDATOR PROGRAM — BY APPLICATION.** Onboarding is **coordinated, not self-serve**: we're accepting a first cohort of **up to ~20 beta-testnet validators** (more slots later). The public setup steps below are **retained for reference but cannot currently auto-join** the live network (recent consensus-core migration — a node built from them will start, dial peers, then silently fail to sync). **Accepted operators receive the correct build, genesis, and bootstrap details directly.** Apply: [application form](https://docs.google.com/forms/d/e/1FAIpQLSczUhY4F_eNL_buvMphFMPFwGkth3lxtMwB7ynirgBXX6-kCA/viewform) or **validators@vexidus.com** — see [VALIDATOR_APPLICATION.md](./VALIDATOR_APPLICATION.md).

Everything you need to run a validator node and manage staking on Vexidus.

## Contents

- [Setup Guide](#setup-guide) — Run a validator node
- [Staking SDK](#staking-sdk) — Delegation, rewards, commission
- [Genesis Files](./genesis/) — Network genesis configuration

## Setup Guide

### Requirements

- **OS**: Ubuntu 22.04+ (or any Linux with glibc 2.35+)
- **CPU**: 6+ cores, dedicated (no co-hosted heavy workloads — resource contention causes missed blocks)
- **RAM**: 16 GB minimum, 32 GB recommended
- **Disk**: 200 GB NVMe SSD
- **Network**: 100 Mbps, static IP, port `9946/UDP` open

### Install

> **Coordinated onboarding (beta).** The exact binary, genesis snapshot, current bootstrap peers, and start command are provided directly to accepted beta operators — the live network's bootstrap peers and consensus wire format are not the ones in older public examples, so a self-serve join will not sync. Apply via the [application form](https://docs.google.com/forms/d/e/1FAIpQLSczUhY4F_eNL_buvMphFMPFwGkth3lxtMwB7ynirgBXX6-kCA/viewform) or **validators@vexidus.com** to receive a working setup package.
>
> General notes:
> - The validator signing key is 32 random bytes — generate with `openssl rand -hex 32 > validator.key && chmod 600 validator.key`. **There is no `keygen` subcommand.**
> - Ports: `9946/UDP` (QUIC P2P) open inbound; `9933/TCP` (RPC) firewalled to localhost / trusted IPs only.

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
  --bootnodes <current-bootstrap-peers-provided-to-accepted-operators> \
  --validator-key /opt/vexidus/keys/validator.key
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

> Bootstrap peers are provided directly to accepted beta operators. **Peer IDs rotate on network resets**, so any hard-coded `/p2p/<peer-id>` values in older examples are unreliable — accepted operators use bare multiaddrs (peer IDs are discovered on connect) from the current bootstrap set.
