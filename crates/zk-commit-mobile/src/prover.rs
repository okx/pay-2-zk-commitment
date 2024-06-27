use plonky2::field::types::Field;

use zk_commit_core::{
    circuit_config::D,
    prover::{
        generate_proof_of_claim as core_generate_proof_of_claim,
        setup_commitment as core_setup_commitment,
    },
    types::{C, F},
};

use crate::{commitment_tree::CommitmentTree, utils::AmountSecretPairing, ZkCommitmentMobileError};

#[uniffi::export]
pub fn setup_commitment(distribution: Vec<AmountSecretPairing>) -> CommitmentTree {
    let distribution = distribution.iter().map(|x| x.to_core()).collect::<Vec<_>>();
    let core_commitment_tree = core_setup_commitment(distribution);
    CommitmentTree::from_core(&core_commitment_tree)
}

#[derive(uniffi::Record)]
pub struct GenerateProofOfClaimResult {
    pub proof: Vec<u8>,
    pub amount: u64,
    pub public_inputs: Vec<u64>,
}

#[uniffi::export]
pub fn generate_proof_of_claim(
    amount: u64,
    secret: u64,
    index: i32,
    commitment_tree: CommitmentTree,
    path: &str,
) -> Result<(), ZkCommitmentMobileError> {
    match core_generate_proof_of_claim::<F, C, C, D>(
        F::from_canonical_u64(amount),
        F::from_canonical_u64(secret),
        index as usize,
        commitment_tree.to_core(),
        path,
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(ZkCommitmentMobileError::GenerateProofOfClaimError),
    }
}
