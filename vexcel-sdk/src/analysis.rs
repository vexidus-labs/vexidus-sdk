//! Offline analysis utilities for Vexcel DAG data.
//!
//! These functions work on already-fetched data — no RPC calls required.
//! Use when you've bulk-fetched DAG data and want to analyze it locally.

use crate::types::*;

/// Check if all attestation blocks at a height agree on the same state root
/// as the canonical block. Disagreement indicates a potential fork or state divergence.
pub fn check_state_agreement(dag: &BlockDAG) -> StateAgreement {
    let canonical_root = &dag.canonical.state_root;
    let mut agreeing = Vec::new();
    let mut disagreeing = Vec::new();

    for att in &dag.attestations {
        if att.state_root == *canonical_root {
            agreeing.push(att.proposer.clone());
        } else {
            disagreeing.push(StateDisagreement {
                validator: att.proposer.clone(),
                expected: canonical_root.clone(),
                actual: att.state_root.clone(),
            });
        }
    }

    let unanimous = disagreeing.is_empty();

    StateAgreement {
        height: dag.height,
        canonical_state_root: canonical_root.clone(),
        agreeing_validators: agreeing,
        disagreements: disagreeing,
        unanimous,
    }
}

/// Compute the attestation latency — time between canonical block and each attestation.
/// Lower latency means the validator detected the leader's absence faster.
pub fn attestation_latencies(dag: &BlockDAG) -> Vec<AttestationLatency> {
    let canonical_ts = dag.canonical.timestamp;
    dag.attestations.iter().map(|att| {
        let delta_secs = if att.timestamp >= canonical_ts {
            att.timestamp - canonical_ts
        } else {
            0 // Attestation arrived before canonical (clock skew or fast attester)
        };
        AttestationLatency {
            validator: att.proposer.clone(),
            timestamp: att.timestamp,
            delta_secs,
        }
    }).collect()
}

/// Identify which validators in a known set did NOT produce an attestation
/// or canonical block at a given height. These validators were offline or
/// too slow to contribute.
pub fn missing_validators(dag: &BlockDAG, known_validators: &[String]) -> Vec<String> {
    let active: std::collections::HashSet<&str> = std::iter::once(dag.canonical.proposer.as_str())
        .chain(dag.attestations.iter().map(|a| a.proposer.as_str()))
        .collect();

    known_validators.iter()
        .filter(|v| !active.contains(v.as_str()))
        .cloned()
        .collect()
}

/// Determine if a height was purely linear (leader was fast, no attestations)
/// or formed a DAG (leader was slow, validators produced attestations).
pub fn is_dag_height(dag: &BlockDAG) -> bool {
    dag.attestation_count > 0
}

/// Count how many unique validators participated at a given height
/// (canonical leader + attestation producers).
pub fn active_validator_count(dag: &BlockDAG) -> usize {
    let mut validators: std::collections::HashSet<&str> = std::collections::HashSet::new();
    validators.insert(&dag.canonical.proposer);
    for att in &dag.attestations {
        validators.insert(&att.proposer);
    }
    validators.len()
}

/// Result of checking state agreement at a height.
#[derive(Debug, Clone)]
pub struct StateAgreement {
    /// Block height checked
    pub height: u64,
    /// State root from the canonical block
    pub canonical_state_root: String,
    /// Validators whose attestation state root matches canonical
    pub agreeing_validators: Vec<String>,
    /// Validators with different state roots
    pub disagreements: Vec<StateDisagreement>,
    /// True if all attestation blocks agree with canonical
    pub unanimous: bool,
}

/// A validator whose state root disagrees with canonical.
#[derive(Debug, Clone)]
pub struct StateDisagreement {
    /// Validator address (Vx0 format)
    pub validator: String,
    /// Expected state root (from canonical block)
    pub expected: String,
    /// Actual state root in the attestation
    pub actual: String,
}

/// Attestation latency relative to canonical block.
#[derive(Debug, Clone)]
pub struct AttestationLatency {
    /// Validator address
    pub validator: String,
    /// Attestation timestamp (Unix seconds)
    pub timestamp: u64,
    /// Seconds after canonical block timestamp
    pub delta_secs: u64,
}
