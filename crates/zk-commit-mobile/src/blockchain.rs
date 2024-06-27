use crate::ZkCommitmentMobileError;
use zk_commit_core::blockchain::Blockchain;
use zk_commit_core::groth16::Groth16ProofWithPublicData;

#[uniffi::export]
pub fn get_claim_token_call_data(pwi: &str) -> Result<Vec<u8>, ZkCommitmentMobileError> {
    let pwi: Groth16ProofWithPublicData = serde_json::from_str(pwi)?;
    Ok(Blockchain::get_claim_token_call_data(&pwi))
}
