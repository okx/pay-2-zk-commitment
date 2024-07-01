use std::{
    io,
    time::{Duration, Instant},
};
use uniffi::deps::anyhow::Error;

use zk_commit_core::examples::fibonacci::fibonacci as core_fibonacci;

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
}

impl From<io::Error> for ZkCommitmentMobileError {
    fn from(value: io::Error) -> Self {
        ZkCommitmentMobileError::Io(value.to_string())
    }
}

impl From<Error> for ZkCommitmentMobileError {
    fn from(value: Error) -> Self {
        ZkCommitmentMobileError::Anyhow(value.to_string())
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
