//! # Vexcel SDK — Adaptive Attestation DAG Client
//!
//! Vexcel is the adaptive attestation DAG consensus layer for the Vexidus blockchain.
//! When the block leader is fast, the chain is purely linear (zero overhead). When the
//! leader is slow, non-leader validators produce lightweight attestation blocks —
//! signed, zero-transaction blocks that prove liveness and state agreement.
//!
//! This SDK provides a typed client for querying attestation data and analyzing
//! validator liveness through the Vexcel DAG.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use vexcel_sdk::VexcelClient;
//!
//! let client = VexcelClient::new("https://testnet.vexidus.io");
//!
//! // Get the full DAG structure at a block height
//! let dag = client.get_block_dag(150000).unwrap();
//! println!("Canonical block: {}", dag.canonical.hash);
//! println!("Attestations: {}", dag.attestation_count);
//!
//! // Get attestation blocks at a height
//! let attestations = client.get_attestations(150000).unwrap();
//! for att in &attestations {
//!     println!("Validator {} attested at {}", att.proposer, att.timestamp);
//! }
//!
//! // Compute validator liveness over a range
//! let liveness = client.validator_liveness(149000, 150000).unwrap();
//! for score in &liveness {
//!     println!("{}: {:.1}% liveness", score.validator, score.liveness_pct);
//! }
//! ```
//!
//! ## Attribution
//!
//! This SDK is open-source under Apache 2.0 with an attribution requirement.
//! If you use Vexcel in your project, you must include:
//!
//! **"Powered by Vexcel — <https://vexcel.xyz>"**
//!
//! See the [NOTICE](https://github.com/vexidus-labs/vexidus-sdk/blob/main/vexcel-sdk/NOTICE)
//! file for full details.

pub mod types;
pub mod client;
pub mod analysis;

pub use types::*;
pub use client::VexcelClient;
pub use analysis::*;

/// Vexcel SDK version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Attribution string. Integrators must display this in a user-visible location.
pub const ATTRIBUTION: &str = "Powered by Vexcel — https://vexcel.xyz";

/// Print the attribution requirement to stdout.
/// Call this at startup to remind integrators of the attribution requirement.
pub fn print_attribution() {
    eprintln!("┌─────────────────────────────────────────────┐");
    eprintln!("│  Vexcel SDK v{}                            │", VERSION);
    eprintln!("│  Powered by Vexcel — https://vexcel.xyz     │");
    eprintln!("│                                             │");
    eprintln!("│  Attribution required: display the line      │");
    eprintln!("│  above in your app footer or docs.          │");
    eprintln!("│  See NOTICE file for details.               │");
    eprintln!("└─────────────────────────────────────────────┘");
}
