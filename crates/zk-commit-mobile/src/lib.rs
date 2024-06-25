use std::time::{Duration, Instant};

use zk_commit_core::fibonacci::fibonacci as core_fibonacci;

pub mod commitment_tree;
pub mod prover;
pub mod utils;

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum ZkCommitmentMobileError {
    #[error("Failed to execute Fibonacci Proof")]
    FibonacciError,
    #[error("Failed to generate proof of claim")]
    GenerateProofOfClaimError,
}

#[derive(uniffi::Record)]
pub struct ProofResult {
    pub first_input: u64,
    pub second_input: u64,
    pub output: u64,
    pub duration: Duration,
}

#[uniffi::export]
pub fn fibonacci() -> Result<ProofResult, ZkCommitmentMobileError> {
    let start = Instant::now();
    match core_fibonacci() {
        Ok(result) => Ok(ProofResult {
            first_input: result.input.0 .0,
            second_input: result.input.1 .0,
            output: result.output.0,
            duration: start.elapsed(),
        }),
        Err(_) => Err(ZkCommitmentMobileError::FibonacciError),
    }
}

#[uniffi::export]
pub fn test_performance() {
    // Add your test code here
    // println!("testing performance")
}
