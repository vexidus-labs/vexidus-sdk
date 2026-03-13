use vexcel_sdk::*;

fn mock_dag(att_count: usize, disagreeing: usize) -> BlockDAG {
    let canonical = CanonicalBlock {
        hash: "VxhCanonical123".to_string(),
        timestamp: 1710000000,
        proposer: "Vx0Leader".to_string(),
        proposer_hex: "0xleader".to_string(),
        parent_blocks: vec!["VxhParent".to_string()],
        state_root: "VxhStateRoot".to_string(),
        bundle_count: 5,
    };

    let attestations: Vec<AttestationBlock> = (0..att_count).map(|i| {
        AttestationBlock {
            hash: format!("VxhAtt{}", i),
            height: 150000,
            timestamp: 1710000000 + (i as u64 + 1) * 2,
            proposer: format!("Vx0Validator{}", i),
            proposer_hex: format!("0xval{}", i),
            state_root: if i < disagreeing {
                format!("VxhWrongRoot{}", i)
            } else {
                "VxhStateRoot".to_string()
            },
            parent_hash: "VxhParent".to_string(),
        }
    }).collect();

    BlockDAG {
        height: 150000,
        canonical,
        attestations,
        attestation_count: att_count as u64,
    }
}

#[test]
fn test_state_agreement_unanimous() {
    let dag = mock_dag(4, 0);
    let agreement = check_state_agreement(&dag);
    assert!(agreement.unanimous);
    assert_eq!(agreement.agreeing_validators.len(), 4);
    assert_eq!(agreement.disagreements.len(), 0);
}

#[test]
fn test_state_agreement_with_disagreement() {
    let dag = mock_dag(4, 1);
    let agreement = check_state_agreement(&dag);
    assert!(!agreement.unanimous);
    assert_eq!(agreement.agreeing_validators.len(), 3);
    assert_eq!(agreement.disagreements.len(), 1);
    assert_eq!(agreement.disagreements[0].validator, "Vx0Validator0");
}

#[test]
fn test_is_dag_height() {
    let linear = mock_dag(0, 0);
    assert!(!is_dag_height(&linear));

    let dag = mock_dag(3, 0);
    assert!(is_dag_height(&dag));
}

#[test]
fn test_active_validator_count() {
    let dag = mock_dag(4, 0);
    assert_eq!(active_validator_count(&dag), 5); // 1 leader + 4 attesters
}

#[test]
fn test_active_validator_count_linear() {
    let dag = mock_dag(0, 0);
    assert_eq!(active_validator_count(&dag), 1); // leader only
}

#[test]
fn test_attestation_latencies() {
    let dag = mock_dag(3, 0);
    let latencies = attestation_latencies(&dag);
    assert_eq!(latencies.len(), 3);
    assert_eq!(latencies[0].delta_secs, 2); // first attestation 2s after canonical
    assert_eq!(latencies[1].delta_secs, 4);
    assert_eq!(latencies[2].delta_secs, 6);
}

#[test]
fn test_missing_validators() {
    let dag = mock_dag(2, 0);
    let known = vec![
        "Vx0Leader".to_string(),
        "Vx0Validator0".to_string(),
        "Vx0Validator1".to_string(),
        "Vx0ValidatorOffline".to_string(),
    ];
    let missing = missing_validators(&dag, &known);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0], "Vx0ValidatorOffline");
}

#[test]
fn test_attribution_constant() {
    assert_eq!(ATTRIBUTION, "Powered by Vexcel — https://vexcel.xyz");
}

#[test]
fn test_version() {
    assert_eq!(VERSION, "1.0.0");
}
