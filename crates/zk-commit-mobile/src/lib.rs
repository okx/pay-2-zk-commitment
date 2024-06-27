use std::{
    io,
    time::{Duration, Instant},
};

use uniffi::deps::anyhow;

use zk_commit_core::fibonacci::fibonacci as core_fibonacci;

pub mod blockchain;
pub mod commitment_tree;
pub mod prover;
pub mod utils;

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum ZkCommitmentMobileError {
    #[error("Io: {0}")]
    Io(String),
    #[error("Anyhow: {0}")]
    Anyhow(String),
    #[error("Serde_json: {0}")]
    SerdeJson(String),
}

impl From<io::Error> for ZkCommitmentMobileError {
    fn from(value: io::Error) -> Self {
        ZkCommitmentMobileError::Io(value.to_string())
    }
}

impl From<anyhow::Error> for ZkCommitmentMobileError {
    fn from(value: anyhow::Error) -> Self {
        ZkCommitmentMobileError::Anyhow(value.to_string())
    }
}

impl From<serde_json::Error> for ZkCommitmentMobileError {
    fn from(value: serde_json::Error) -> Self {
        ZkCommitmentMobileError::SerdeJson(value.to_string())
    }
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
    let result = core_fibonacci()?;
    Ok(ProofResult {
        first_input: result.input.0 .0,
        second_input: result.input.1 .0,
        output: result.output.0,
        duration: start.elapsed(),
    })
}

#[uniffi::export]
pub fn test_performance() {
    // Add your test code here
    // println!("testing performance")
}
