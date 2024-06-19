use std::time::{Duration, Instant};

uniffi::setup_scaffolding!();

use zk_commit_core::fibonacci::fibonacci;

#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum ProveError {
    #[error("Failed to execute Fibonacci Proof")]
    FibonacciError,
}


#[derive(uniffi::Record)]
pub struct ProofResult {
    pub first_input: u64,
    pub second_input: u64,
    pub output: u64,
    pub duration: Duration,
}

#[uniffi::export]
pub fn fibonacci_mobile() -> Result<ProofResult, ProveError> {
    let start = Instant::now();
    match fibonacci() {
        Ok(result) => Ok(ProofResult {
            first_input: result.input.0.0,
            second_input: result.input.1.0,
            output: result.output.0,
            duration: start.elapsed(),
        }),
        Err(_) => Err(ProveError::FibonacciError),
    }
}
