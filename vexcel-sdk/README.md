# Vexcel SDK

**Adaptive Attestation DAG Client for the Vexidus Blockchain**

Vexcel is the consensus enhancement layer that enables geographic fairness in block production. When the leader is fast, the chain is purely linear (zero overhead). When the leader is slow, non-leader validators produce lightweight attestation blocks that prove liveness and state agreement — forming an adaptive DAG.

## Install

```toml
[dependencies]
vexcel-sdk = "1.0"
```

## Quick Start

```rust
use vexcel_sdk::{VexcelClient, check_state_agreement, is_dag_height};

let client = VexcelClient::new("https://testnet.vexidus.io");

// Get the DAG structure at a height
let dag = client.get_block_dag(150000)?;
println!("Leader: {}", dag.canonical.proposer);
println!("Attestations: {}", dag.attestation_count);

// Check if all validators agree on state
let agreement = check_state_agreement(&dag);
println!("Unanimous: {}", agreement.unanimous);

// Was this height linear or DAG?
if is_dag_height(&dag) {
    println!("DAG formed — leader was slow");
} else {
    println!("Linear — leader was fast");
}
```

## Features

### RPC Client
- `get_attestations(height)` — Get attestation blocks at a height
- `get_block_dag(height)` — Full DAG structure (canonical + attestations)
- `get_current_height()` — Current block height
- `validator_liveness(from, to)` — Liveness scores over a range
- `dag_density(from, to)` — How often the network uses DAG vs linear mode

### Offline Analysis
- `check_state_agreement(dag)` — Verify validators agree on state root
- `attestation_latencies(dag)` — Time delta between canonical and attestation blocks
- `missing_validators(dag, known_set)` — Which validators were offline
- `active_validator_count(dag)` — Unique validators at a height
- `is_dag_height(dag)` — Was this height linear or DAG?

## Attribution

This SDK is open-source under **Apache 2.0 with attribution**.

If you use Vexcel in your project, you must include this in a visible location:

> **Powered by Vexcel — https://vexcel.xyz**

See [NOTICE](./NOTICE) for details.

## Links

- **Website**: https://vexcel.xyz
- **Documentation**: https://docs.vexidus.io/architecture/hypersync#vexcel--adaptive-attestation-dag
- **Vexidus Blockchain**: https://vexidus.io
