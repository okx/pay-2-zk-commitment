use plonky2::hash::hash_types::HashOut;

use crate::{types::F, utils::AmountSecretPairing};

pub struct ClaimProvingInputs {
    pub pair: AmountSecretPairing,
    pub commitment: HashOut<F>,
    pub siblings: Vec<HashOut<F>>,
    pub nullifier_hash: HashOut<F>,
    pub own_leaf_hash: HashOut<F>,
}

pub struct Claim {
    pub pair: AmountSecretPairing,
    pub commitment: HashOut<F>,
    pub siblings: Vec<HashOut<F>>,
}

impl Claim {
    pub fn get_nullifier_hash(&self) -> HashOut<F> {
        self.pair.get_nullifier_hash()
    }

    pub fn get_hash(&self) -> HashOut<F> {
        self.pair.get_own_hash()
    }
}

pub fn execute_claim(claim: Claim) -> ClaimProvingInputs {
    let nullifier_hash = claim.get_nullifier_hash();
    let own_leaf_hash = claim.get_hash();
    ClaimProvingInputs {
        pair: claim.pair,
        commitment: claim.commitment,
        siblings: claim.siblings,
        nullifier_hash,
        own_leaf_hash,
    }
}
