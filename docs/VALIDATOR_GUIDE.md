# Vexidus Validator Guide

> 🛡️ **BETA VALIDATOR PROGRAM — BY APPLICATION (updated 2026-07).** We're onboarding a first cohort of **up to ~20 beta-testnet validators** (more slots in later waves). **Onboarding is coordinated, not self-serve** — the network recently migrated to a new BFT consensus core, so a node built from the public steps below **cannot yet auto-join the live network** (it will start, dial peers, then silently fail to sync). **Accepted operators receive the correct binary, genesis snapshot, and current bootstrap peers directly.** To apply: [application form](https://docs.google.com/forms/d/e/1FAIpQLSczUhY4F_eNL_buvMphFMPFwGkth3lxtMwB7ynirgBXX6-kCA/viewform) or email **validators@vexidus.com**. The steps below are retained for reference and will be updated when self-serve onboarding opens.

Run a validator node on the Vexidus network and earn VXS rewards for securing the chain.

---

## Overview

Vexidus is a high-performance L1 blockchain secured by a production BFT consensus core (2f+1 quorum, instant finality). Validators produce blocks, earn block rewards and transaction fees, and participate in on-chain upgrade governance via VexVisor.

| Parameter | Value |
|-----------|-------|
| Minimum stake | 1,000 VXS |
| Unbonding period | 21 days |
| Max active validators | 100 |
| Block time | Consensus-paced (sub-second to low-seconds) |
| Epoch duration | 300 seconds |
| Jailing threshold | 50 consecutive missed blocks |
| Jail cooldown | 5 minutes |
| Slashing | **None** — Vexidus uses jailing, not economic penalties |

### Rewards

- **Block rewards** — Time-weighted from a 10-year emission schedule. 85% to validators (density-weighted 0.5x–2.0x), 15% to Foundation Treasury
- **Transaction fees** — 80% to block proposer, 20% to Foundation Treasury (no burn)
- **Delegation commission** — Configurable (0–50%), earned from delegated stake rewards

---

## System Requirements

### Minimum

| Resource | Spec |
|----------|------|
| CPU | 6 cores, dedicated |
| RAM | 16 GB |
| Storage | 200 GB NVMe SSD |
| Network | 100 Mbps, static IP |
| OS | Ubuntu 22.04+ / Debian 12+ (x86_64) |

> One dedicated machine per validator — do **not** co-host other heavy workloads (build servers, other chains, AI/inference); resource contention causes missed blocks.

### Recommended

| Resource | Spec |
|----------|------|
| CPU | 8+ cores / 16 threads |
| RAM | 16 GB+ |
| Storage | 500 GB NVMe SSD |
| Network | 1 Gbps, static IP, low latency |

### Required Ports

| Port | Protocol | Purpose | Access |
|------|----------|---------|--------|
| 9946 | UDP (QUIC) | P2P consensus and block sync | Open to all |
| 9933 | TCP | JSON-RPC API | **Localhost only** (restrict via firewall) |

---

## Quick Install

```bash
curl -sSf https://vexidus.io/install-validator.sh | bash
```

This downloads the pre-built binary, sets up VexVisor (upgrade manager), genesis files, system user, directories, and a systemd service template. No Rust toolchain required.

> Inspect before running: `curl -sSf https://vexidus.io/install-validator.sh | less`

### What it creates

```
/opt/vexidus/
├── data/                      Chain state (RocksDB)
├── genesis_vxs.json           VXS token allocations (must be identical across all validators)
├── genesis_tokens.json        Bridged token registry
├── validator.key              Your Ed25519 signing key (you create this)
├── tools/
│   └── vexvisor.sh            Upgrade manager (handles binary swaps)
└── vexvisor/
    ├── current/
    │   └── vexidus-node       Active binary
    └── upgrades/              Downloaded upgrade binaries
```

---

## Step-by-Step Setup

### 1. Generate your validator key

Your validator key is an Ed25519 keypair used to sign blocks and votes. **If you lose this key, your staked VXS cannot be recovered.**

```bash
# Generate a random 32-byte key
openssl rand -hex 32 > validator.key

# Secure it
sudo mv validator.key /opt/vexidus/validator.key
sudo chown vexidus:vexidus /opt/vexidus/validator.key
sudo chmod 600 /opt/vexidus/validator.key
```

**Back up your key** to an encrypted USB drive, password manager, or HSM. Never share it.

### 2. Get your public key

```bash
/opt/vexidus/vexvisor/current/vexidus-node \
  --validator-key /opt/vexidus/validator.key \
  --data-dir /tmp/vex-keytest 2>&1 | grep "public key"

rm -rf /tmp/vex-keytest
```

This prints your public key in `0x...` format (64 hex characters = 32 bytes). You'll need this for staking.

### 3. Configure the systemd service

The installer creates `/etc/systemd/system/vexidus-validator.service`. You need to update it with your public IP:

```bash
# Replace the placeholder with your actual IP
PUBLIC_IP=$(curl -sf https://ifconfig.me)
sudo sed -i "s|REPLACE_WITH_YOUR_PUBLIC_IP|${PUBLIC_IP}|g" /etc/systemd/system/vexidus-validator.service
sudo systemctl daemon-reload
```

### 4. Configure your firewall

```bash
# Allow P2P traffic (required for consensus)
sudo ufw allow 9946/udp

# Block public RPC access (keep localhost only)
sudo ufw deny 9933/tcp

# If you need RPC from a specific trusted IP:
# sudo ufw allow from YOUR_TRUSTED_IP to any port 9933 proto tcp

sudo ufw enable
```

### 5. Request testnet VXS

Validators need a minimum of **1,000 VXS** to join the active set. During the testnet phase, we distribute VXS directly to verified validators.

**To request VXS, contact us with:**
- Your validator **public key** (from step 2)
- Your server **location** (city/country)
- A brief description of your setup (specs, hosting provider)

**Contact channels:**
- Email: **validators@vexidus.com**
- Discord: **https://discord.com/invite/F4QcnKZHf2**

We review requests and send the staking amount within 24 hours. This process lets us monitor network health and onboard validators at a sustainable pace.

### 6. Start your node

```bash
sudo systemctl enable vexidus-validator
sudo systemctl start vexidus-validator
```

Watch the logs:

```bash
sudo journalctl -u vexidus-validator -f
```

You should see:

```
Vexidus Blockchain Node v0.x.x
Loading validator key from "/opt/vexidus/validator.key"
Validator public key: 0x...
P2P Network started successfully
Listening on /ip4/0.0.0.0/udp/9946/quic-v1
RPC server listening on 0.0.0.0:9933
```

### 7. Wait for sync

Before staking, ensure your node is fully synced:

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_getNetworkStats","params":[],"id":1}' | jq .result.blockHeight
```

Compare with the network height on [vexscan.io](https://vexscan.io).

### 8. Stake and register

Once synced and you've received VXS:

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "vex_stake",
    "params": ["YOUR_STAKER_ADDRESS", "1000", "YOUR_VALIDATOR_PUBKEY_HEX"],
    "id": 1
  }' | jq .
```

| Parameter | Description |
|-----------|-------------|
| `YOUR_STAKER_ADDRESS` | Account holding your VXS (hex-encoded 32 bytes) |
| `1000` | Amount of VXS to stake (minimum 1,000) |
| `YOUR_VALIDATOR_PUBKEY_HEX` | 64-char hex public key from step 2 |

### 9. Set your validator profile (optional)

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "vex_setValidatorMetadata",
    "params": ["YOUR_ADDRESS", "My Validator", "Reliable node operator", "https://mysite.com", "https://mysite.com/avatar.png"],
    "id": 1
  }' | jq .
```

### 10. Set commission rate (optional)

Commission is the percentage of delegator rewards you keep (basis points, max 5000 = 50%):

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "vex_setCommission",
    "params": ["YOUR_ADDRESS", 500],
    "id": 1
  }' | jq .
```

`500` = 5% commission. Default is 0%.

---

## VexVisor — Upgrade Manager

VexVisor is a Cosmovisor-style wrapper that manages binary upgrades without manual intervention. It runs your node binary and handles lifecycle events based on exit codes.

### How upgrades work

1. **Schedule** — The upgrade authority (or validator governance) proposes an upgrade at a target block height, specifying a binary download URL and SHA256 checksum
2. **Halt** — When your node reaches the target height, it writes `data/upgrade-info.json` and exits with code 42
3. **Download** — VexVisor downloads the new binary, verifies the SHA256 checksum, and validates it
4. **Swap** — The old binary is backed up and the new one takes its place
5. **Restart** — VexVisor restarts with the new binary
6. **Auto-rollback** — If the new binary crashes within 60 seconds, VexVisor automatically restores the previous binary

### Upgrade modes

#### Auto mode (default)

VexVisor handles everything. When an upgrade is scheduled on-chain:

```
Node reaches target block → halts → VexVisor downloads → verifies → swaps → restarts
```

No action needed from you. This is the recommended mode.

#### Manual mode

For validators who prefer to review binaries before applying them:

1. **Monitor** — Watch for upgrade announcements on Discord or check:
   ```bash
   curl -s http://localhost:9933 \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"vex_getUpgradePlan","params":[],"id":1}' | jq .
   ```

2. **Pre-stage** — Download and verify the binary before the target height:
   ```bash
   # Get upgrade details
   UPGRADE_NAME="v0.2.0"  # from the upgrade plan
   BINARY_URL="https://vexidus.io/releases/vexidus-node"
   EXPECTED_SHA="abc123..."

   # Download and verify
   mkdir -p /opt/vexidus/vexvisor/upgrades/${UPGRADE_NAME}
   curl -fSL -o /opt/vexidus/vexvisor/upgrades/${UPGRADE_NAME}/vexidus-node "${BINARY_URL}"
   sha256sum /opt/vexidus/vexvisor/upgrades/${UPGRADE_NAME}/vexidus-node
   # Verify the hash matches EXPECTED_SHA
   chmod +x /opt/vexidus/vexvisor/upgrades/${UPGRADE_NAME}/vexidus-node
   ```

3. **Wait** — At the target height, VexVisor detects the pre-staged binary, verifies its checksum, and swaps without downloading

### Vote on upgrades (full governance mode)

When the network moves to full governance mode, validators vote on proposed upgrades:

```bash
# View the active upgrade proposal
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_getUpgradePlan","params":[],"id":1}' | jq .

# Vote to approve
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"vex_voteUpgrade",
    "params":["YOUR_ADDRESS", "v0.2.0", true],
    "id":1
  }' | jq .
```

Upgrades require 67% stake-weighted approval to proceed.

### VexVisor safety features

| Feature | Description |
|---------|-------------|
| **SHA256 verification** | Downloaded binary must match the on-chain checksum exactly |
| **Binary validation** | Checks ELF format and `--version` response before swapping |
| **Auto-rollback** | Crashes within 60s of swap restore the previous binary |
| **Exponential backoff** | Crash retries: 2s, 4s, 8s, 16s, 32s (max 5 retries) |
| **Loop breaker** | 10 cumulative failures across systemd restarts = full stop |
| **Pre-staging** | Download binaries ahead of time to avoid upgrade-time network issues |
| **Backup pruning** | Keeps the 3 most recent backups, cleans older ones |

---

## Monitoring

### Quick health check

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_getNetworkStats","params":[],"id":1}' | jq .result
```

### Check your validator status

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_getValidator","params":["YOUR_ADDRESS"],"id":1}' | jq .
```

### Monitoring script

Save as `/opt/vexidus/monitor.sh`:

```bash
#!/bin/bash
RPC="http://localhost:9933"

rpc_call() {
  curl -sf "$RPC" \
    -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"method\":\"$1\",\"params\":$2,\"id\":1}" 2>/dev/null
}

stats=$(rpc_call "vex_getNetworkStats" "[]")
block=$(echo "$stats" | jq -r '.result.blockHeight // "unreachable"')
peers=$(echo "$stats" | jq -r '.result.peerCount // 0')
validators=$(echo "$stats" | jq -r '.result.validatorCount // 0')
status=$(systemctl is-active vexidus-validator)
uptime=$(systemctl show vexidus-validator --property=ActiveEnterTimestamp --value)

echo "=============================="
echo "  Vexidus Validator Status"
echo "=============================="
echo "  Service:       $status"
echo "  Block height:  $block"
echo "  Peers:         $peers"
echo "  Validators:    $validators"
echo "  Up since:      $uptime"
echo "=============================="
```

---

## Security Best Practices

- **Key file**: Permissions `600`, owned by `vexidus` user only. Back up offline.
- **RPC port**: Never expose 9933 to the internet. Use `ufw deny 9933/tcp`.
- **P2P port**: 9946/udp must be open for consensus participation.
- **System updates**: `sudo apt update && sudo apt upgrade -y` regularly.
- **Unattended security updates**: `sudo apt install -y unattended-upgrades`
- **SSH**: Key-based authentication only, disable password login.
- **Dedicated user**: Run the node as `vexidus`, never as root.

---

## Maintenance

### Manually updating the binary

If you need to update outside the VexVisor upgrade cycle:

```bash
# Download new binary
curl -fSL -o /tmp/vexidus-node https://vexidus.io/releases/vexidus-node

# Verify checksum (get expected hash from Discord or release notes)
sha256sum /tmp/vexidus-node

# Stop, swap, start
sudo systemctl stop vexidus-validator
sudo cp /tmp/vexidus-node /opt/vexidus/vexvisor/current/vexidus-node
sudo chmod +x /opt/vexidus/vexvisor/current/vexidus-node
sudo chown vexidus:vexidus /opt/vexidus/vexvisor/current/vexidus-node
sudo systemctl start vexidus-validator
```

### Unstaking

To leave the validator set:

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "vex_unstake",
    "params": ["YOUR_ADDRESS", "1000"],
    "id": 1
  }' | jq .
```

After 21 days, claim your VXS:

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_claimUnstake","params":["YOUR_ADDRESS"],"id":1}' | jq .
```

### Full resync

If your node state is corrupted:

```bash
sudo systemctl stop vexidus-validator
sudo rm -rf /opt/vexidus/data
sudo systemctl start vexidus-validator
```

Your `validator.key` and peer identity are preserved.

---

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Permission denied on key file | `sudo chown vexidus:vexidus /opt/vexidus/validator.key && sudo chmod 600 /opt/vexidus/validator.key` |
| Port already in use | `sudo lsof -i :9946` to find conflicts |
| No peers connecting | Verify 9946/udp is open: `nc -zuv YOUR_IP 9946` from another machine |
| Not producing blocks | Ensure synced, staked >= 1,000 VXS, not jailed |
| Validator jailed | Wait 5 min, then: `vex_unjail` RPC call |
| RPC not responding | `systemctl is-active vexidus-validator` and `ss -tlnp \| grep 9933` |
| VexVisor loop breaker | `rm /tmp/vexvisor-failures-*` to reset failure counter |

### Unjail your validator

```bash
curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"vex_unjail","params":["YOUR_ADDRESS"],"id":1}' | jq .
```

---

## Testnet Rewards

Testnet validators who maintain high performance earn **mainnet VXS airdrops** at launch. Your on-chain reputation score (VSC-REP) tracks:

- Uptime and block production consistency
- Vote participation rate
- Stake amount and duration
- Commission fairness
- Transaction processing volume
- Peer connectivity
- Governance participation

New validators start with a performance score of 0.8 (out of 1.0). Higher scores increase your leader selection weight and reward share.

---

## Network Bootstrap Nodes

> The current bootstrap peers are provided directly to accepted beta operators. **Peer IDs rotate on network resets**, so hard-coded `/p2p/<peer-id>` values from older releases are unreliable — accepted operators use bare multiaddrs (peer IDs are discovered on connect) from the current bootstrap set.

---

## CLI Reference

```
vexidus-node [OPTIONS]

Options:
  -d, --data-dir <PATH>           Data directory [default: ./data]
  -v, --verbose                   Enable verbose logging
      --rpc-port <PORT>           RPC listen port [default: 9933]
      --p2p-port <PORT>           P2P listen port, QUIC/UDP [default: 9946]
      --bootnodes <MULTIADDRS>    Comma-separated bootstrap peer multiaddrs
      --validator-key <PATH>      Path to Ed25519 signing key file
      --external-addr <MULTIADDR> Public address to advertise
      --gas-price <N>             Base gas price in nanoVXS [default: 10]
      --pruning <MODE>            archive (all blocks) | validator (1000 blocks) [default: archive]
  -h, --help                      Print help
```

---

## Links

- **Explorer**: [vexscan.io](https://vexscan.io)
- **Docs**: [docs.vexidus.io](https://docs.vexidus.io)
- **SDK**: [github.com/vexidus-labs/vexidus-sdk](https://github.com/vexidus-labs/vexidus-sdk)
- **Discord**: https://discord.com/invite/F4QcnKZHf2
- **Contact**: validators@vexidus.com

---

*Vexidus is its own L1 blockchain. Native addresses use the `Vx` prefix. The `0x` format exists only for EVM compatibility.*
