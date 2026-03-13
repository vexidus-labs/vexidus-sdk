//! Vexcel types — attestation blocks, DAG structures, and validator metadata.

use serde::{Deserialize, Serialize};

/// An attestation block — a lightweight, zero-transaction block produced by a
/// non-leader validator to prove liveness and state agreement.
///
/// Attestation blocks are identified by three conditions:
/// 1. Zero transactions (empty bundle list)
/// 2. Proposer is NOT the elected leader for this height
/// 3. Gas limit is 0 (sentinel value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationBlock {
    /// Block hash in Vxh format (e.g. "VxhABC123...")
    pub hash: String,

    /// Block height this attestation references
    pub height: u64,

    /// Unix timestamp (seconds) when the attestation was produced
    pub timestamp: u64,

    /// Validator address in Vx0 format (e.g. "Vx0ABC123...")
    pub proposer: String,

    /// Validator address in 0x hex format (32 bytes)
    pub proposer_hex: String,

    /// State root hash the validator computed at this height
    pub state_root: String,

    /// Parent block hash
    pub parent_hash: String,
}

/// The canonical block in a DAG structure — the elected leader's block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalBlock {
    /// Block hash in Vxh format
    pub hash: String,

    /// Unix timestamp (seconds)
    pub timestamp: u64,

    /// Leader validator address in Vx0 format
    pub proposer: String,

    /// Leader validator address in 0x hex format
    pub proposer_hex: String,

    /// Parent block hashes — when attestation hashes are referenced,
    /// this contains multiple entries forming the DAG structure.
    #[serde(rename = "parentBlocks")]
    pub parent_blocks: Vec<String>,

    /// State root hash after executing all transactions
    #[serde(rename = "stateRoot")]
    pub state_root: String,

    /// Number of transaction bundles in this block
    #[serde(rename = "bundleCount")]
    pub bundle_count: u64,
}

/// Full DAG structure at a given block height.
///
/// Contains the canonical (leader) block plus all attestation blocks
/// produced by non-leader validators at the same height.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDAG {
    /// Block height
    pub height: u64,

    /// The canonical block (produced by the elected leader)
    pub canonical: CanonicalBlock,

    /// Attestation blocks from non-leader validators
    pub attestations: Vec<AttestationBlock>,

    /// Number of attestation blocks at this height
    #[serde(rename = "attestationCount")]
    pub attestation_count: u64,
}

/// Validator liveness score computed from attestation frequency.
#[derive(Debug, Clone)]
pub struct LivenessScore {
    /// Validator address (Vx0 format)
    pub validator: String,

    /// Number of attestations produced in the analyzed range
    pub attestation_count: u64,

    /// Number of blocks where this validator was the canonical leader
    pub leader_count: u64,

    /// Total blocks in the analyzed range
    pub total_blocks: u64,

    /// Liveness percentage (0.0 - 100.0)
    /// A validator is "live" at a height if it produced either the canonical block
    /// or an attestation block.
    pub liveness_pct: f64,
}

/// DAG density metric for a range of blocks.
#[derive(Debug, Clone)]
pub struct DAGDensity {
    /// Start height of the range
    pub from_height: u64,

    /// End height of the range (inclusive)
    pub to_height: u64,

    /// Total canonical blocks in range
    pub total_blocks: u64,

    /// Total attestation blocks in range
    pub total_attestations: u64,

    /// Average attestations per block height
    pub avg_attestations_per_height: f64,

    /// Heights with zero attestations (leader was fast)
    pub linear_heights: u64,

    /// Heights with 1+ attestations (leader was slow, DAG formed)
    pub dag_heights: u64,

    /// Percentage of heights that formed DAG structures
    pub dag_pct: f64,
}
