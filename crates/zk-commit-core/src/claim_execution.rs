use plonky2::{hash::{hash_types::HashOut, poseidon::PoseidonHash}, plonk::config::Hasher};

use crate::{comitter::AmountSecretPairing, types::F};

pub struct ClaimProvingInputs{
    pub pair: AmountSecretPairing,
    pub commitment: HashOut<F>,
    pub siblings: Vec<HashOut<F>>,
    pub nullifier_hash: HashOut<F>,
    pub own_leaf_hash: HashOut<F>
}

pub struct Claim{
    pub pair: AmountSecretPairing,
    pub commitment: HashOut<F>,
    pub siblings: Vec<HashOut<F>>,
}

pub fn execute_claim(claim: Claim)-> ClaimProvingInputs{
    let nullifier_hash = claim.pair.get_nullifier_hash();
    let inputs = vec![
        vec![claim.pair.get_amount()],
        nullifier_hash.elements.to_vec(),
        vec![claim.pair.get_secret()]
    ].concat();

    let own_leaf_hash = PoseidonHash::hash_no_pad(inputs.as_slice());

    ClaimProvingInputs{
        pair: claim.pair,
        commitment: claim.commitment,
        siblings: claim.siblings,
        nullifier_hash,
        own_leaf_hash
    }
}