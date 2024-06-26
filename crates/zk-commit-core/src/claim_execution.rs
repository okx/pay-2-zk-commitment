use crate::{utils::AmountSecretPairing};
use plonky2::{
    field::{extension::Extendable, types::Field},
    hash::hash_types::{HashOut, RichField},
};

pub struct ClaimProvingInputs<F: RichField + Extendable<D>, const D: usize> {
    pub pair: AmountSecretPairing<F>,
    pub commitment: HashOut<F>,
    pub commitment_merkle_proof: Vec<HashOut<F>>,
    pub nullifier_hash: HashOut<F>,
    pub index: F,
}

pub struct Claim<F: RichField + Extendable<D>, const D: usize> {
    pub pair: AmountSecretPairing<F>,
    pub commitment: HashOut<F>,
    pub commitment_merkle_proof: Vec<HashOut<F>>,
    pub index: usize,
}

impl<F: RichField + Extendable<D>, const D: usize> Claim<F, D> {
    pub fn get_nullifier_hash(&self) -> HashOut<F> {
        self.pair.get_nullifier_hash()
    }

    pub fn get_hash(&self) -> HashOut<F> {
        self.pair.get_own_hash()
    }
}
/// Executes a claim by transforming a claim into its proving inputs.
pub fn get_claim_proving_inputs<F: RichField + Extendable<D>, const D: usize>(
    claim: Claim<F, D>,
) -> ClaimProvingInputs<F, D> {
    let nullifier_hash = claim.get_nullifier_hash();
    ClaimProvingInputs {
        pair: claim.pair,
        commitment: claim.commitment,
        commitment_merkle_proof: claim.commitment_merkle_proof,
        nullifier_hash,
        index: F::from_canonical_u64(claim.index as u64),
    }
}
