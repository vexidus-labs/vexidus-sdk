//! Vexcel RPC client — typed wrapper around vex_getAttestations and vex_getBlockDAG.

use crate::types::*;
use std::time::Duration;

/// HTTP request timeout for RPC calls.
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);

/// Vexcel client — connects to a Vexidus RPC endpoint to query attestation data.
///
/// # Example
///
/// ```rust,no_run
/// use vexcel_sdk::VexcelClient;
///
/// let client = VexcelClient::new("https://testnet.vexidus.io");
/// let dag = client.get_block_dag(150000).unwrap();
/// println!("Attestations: {}", dag.attestation_count);
/// ```
pub struct VexcelClient {
    rpc_url: String,
    timeout: Duration,
}

impl VexcelClient {
    /// Create a new Vexcel client connected to the given RPC endpoint.
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.trim_end_matches('/').to_string(),
            timeout: HTTP_TIMEOUT,
        }
    }

    /// Create a client with a custom timeout.
    pub fn with_timeout(rpc_url: &str, timeout: Duration) -> Self {
        Self {
            rpc_url: rpc_url.trim_end_matches('/').to_string(),
            timeout,
        }
    }

    /// Get the RPC URL this client is connected to.
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Get all attestation blocks at a given height.
    ///
    /// Returns an empty Vec if no attestations exist (leader was fast,
    /// chain was purely linear at this height).
    pub fn get_attestations(&self, height: u64) -> Result<Vec<AttestationBlock>, VexcelError> {
        let result = self.rpc_call("vex_getAttestations", serde_json::json!([height]))?;
        let attestations: Vec<AttestationBlock> = serde_json::from_value(
            result.get("result").cloned().unwrap_or(serde_json::Value::Array(vec![]))
        ).map_err(|e| VexcelError::ParseError(format!("Failed to parse attestations: {}", e)))?;
        Ok(attestations)
    }

    /// Get the full DAG structure at a given height.
    ///
    /// Returns the canonical block (leader's block) plus all attestation blocks
    /// from non-leader validators.
    pub fn get_block_dag(&self, height: u64) -> Result<BlockDAG, VexcelError> {
        let result = self.rpc_call("vex_getBlockDAG", serde_json::json!([height]))?;
        let dag: BlockDAG = serde_json::from_value(
            result.get("result").cloned()
                .ok_or_else(|| VexcelError::ParseError("No result in response".to_string()))?
        ).map_err(|e| VexcelError::ParseError(format!("Failed to parse BlockDAG: {}", e)))?;
        Ok(dag)
    }

    /// Get the current block height from the connected node.
    pub fn get_current_height(&self) -> Result<u64, VexcelError> {
        let result = self.rpc_call("vex_validatorHealth", serde_json::json!([]))?;
        result.get("result")
            .and_then(|r| r.get("blockHeight"))
            .and_then(|h| h.as_u64())
            .ok_or_else(|| VexcelError::ParseError("Could not read blockHeight".to_string()))
    }

    /// Compute validator liveness scores over a range of blocks.
    ///
    /// Queries attestation data for each height in the range and calculates
    /// how often each validator was active (either as leader or attester).
    pub fn validator_liveness(
        &self,
        from_height: u64,
        to_height: u64,
    ) -> Result<Vec<LivenessScore>, VexcelError> {
        if from_height > to_height {
            return Err(VexcelError::InvalidRange(from_height, to_height));
        }

        let mut validator_attestations: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        let mut validator_leads: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        let mut active_heights: std::collections::HashMap<String, std::collections::HashSet<u64>> = std::collections::HashMap::new();

        let total_blocks = to_height - from_height + 1;

        for height in from_height..=to_height {
            let dag = self.get_block_dag(height)?;

            // Track leader
            let leader = &dag.canonical.proposer;
            *validator_leads.entry(leader.clone()).or_insert(0) += 1;
            active_heights.entry(leader.clone()).or_default().insert(height);

            // Track attesters
            for att in &dag.attestations {
                *validator_attestations.entry(att.proposer.clone()).or_insert(0) += 1;
                active_heights.entry(att.proposer.clone()).or_default().insert(height);
            }
        }

        let mut scores: Vec<LivenessScore> = active_heights.iter().map(|(validator, heights)| {
            LivenessScore {
                validator: validator.clone(),
                attestation_count: *validator_attestations.get(validator).unwrap_or(&0),
                leader_count: *validator_leads.get(validator).unwrap_or(&0),
                total_blocks,
                liveness_pct: (heights.len() as f64 / total_blocks as f64) * 100.0,
            }
        }).collect();

        scores.sort_by(|a, b| b.liveness_pct.partial_cmp(&a.liveness_pct).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scores)
    }

    /// Compute DAG density metrics over a range of blocks.
    ///
    /// Shows how often the network transitions from linear mode to DAG mode,
    /// indicating leader responsiveness across the network.
    pub fn dag_density(
        &self,
        from_height: u64,
        to_height: u64,
    ) -> Result<DAGDensity, VexcelError> {
        if from_height > to_height {
            return Err(VexcelError::InvalidRange(from_height, to_height));
        }

        let total_blocks = to_height - from_height + 1;
        let mut total_attestations = 0u64;
        let mut linear_heights = 0u64;
        let mut dag_heights = 0u64;

        for height in from_height..=to_height {
            let attestations = self.get_attestations(height)?;
            let count = attestations.len() as u64;
            total_attestations += count;
            if count == 0 {
                linear_heights += 1;
            } else {
                dag_heights += 1;
            }
        }

        Ok(DAGDensity {
            from_height,
            to_height,
            total_blocks,
            total_attestations,
            avg_attestations_per_height: total_attestations as f64 / total_blocks as f64,
            linear_heights,
            dag_heights,
            dag_pct: (dag_heights as f64 / total_blocks as f64) * 100.0,
        })
    }

    /// Make a JSON-RPC POST request.
    fn rpc_call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, VexcelError> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let agent = ureq::AgentBuilder::new().timeout(self.timeout).build();
        let resp = agent.post(&self.rpc_url)
            .set("Content-Type", "application/json")
            .send_string(&body.to_string())
            .map_err(|e| VexcelError::RpcError(format!("Request failed: {}", e)))?;
        let json: serde_json::Value = resp.into_json()
            .map_err(|e| VexcelError::ParseError(format!("Invalid JSON response: {}", e)))?;
        if let Some(err) = json.get("error") {
            return Err(VexcelError::RpcError(format!("RPC error: {}", err)));
        }
        Ok(json)
    }
}

/// Errors returned by the Vexcel client.
#[derive(Debug)]
pub enum VexcelError {
    /// RPC request or connection failure
    RpcError(String),
    /// Response parsing failure
    ParseError(String),
    /// Invalid height range (from > to)
    InvalidRange(u64, u64),
}

impl std::fmt::Display for VexcelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VexcelError::RpcError(msg) => write!(f, "Vexcel RPC error: {}", msg),
            VexcelError::ParseError(msg) => write!(f, "Vexcel parse error: {}", msg),
            VexcelError::InvalidRange(from, to) => write!(f, "Invalid range: {} > {}", from, to),
        }
    }
}

impl std::error::Error for VexcelError {}
