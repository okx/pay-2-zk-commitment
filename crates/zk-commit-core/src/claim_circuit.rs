use plonky2::{
    hash::hash_types::HashOutTarget, iop::target::Target, plonk::circuit_builder::CircuitBuilder,
};

use crate::{
    circuit_config::{COMMITMENT_TREE_DEPTH, D},
    circuit_utils::{get_hash_from_input_targets, verify_hash, verify_merkle_proof_fixed_height},
    types::F,
};

pub struct ClaimTargets {
    pub amount: Target,
    pub secret: Target,
    pub nullifier_hash: HashOutTarget,
    pub commitment: HashOutTarget,
    pub siblings: Vec<HashOutTarget>,
}

/// Generates a circuit to verify the claim by doing the following:
///
/// - Verifies that the nullifier hash is correctly calculated
///
/// - Verifies that the commitment is properly calculated given the leaves
///
/// - Verifies that the commitment is calculated with the claimaints own leaf hash (leaf hash is calculated correctly) given their index in the tree
///
/// The public inputs are the nullifier hash, the commitment and the claimed amount
pub fn generate_claim_circuit(builder: &mut CircuitBuilder<F, D>) -> ClaimTargets {
    // The amount to claim
    let amount = builder.add_virtual_target();
    builder.register_public_input(amount);

    // Secret value known only to claimant
    let secret = builder.add_virtual_target();

    // Nullfier hash to prevent replay attack
    let nullifier_hash = builder.add_virtual_hash_public_input();

    // Verify the nullifier hash is calculated correctly based on secret and amount
    verify_hash(builder, vec![secret, amount], nullifier_hash);

    // Create the commitment hash target
    let commitment: HashOutTarget = builder.add_virtual_hash_public_input();

    // Calculate my own leaf hash
    let inputs = vec![vec![amount], nullifier_hash.elements.to_vec(), vec![secret]].concat();
    let own_leaf_hash = get_hash_from_input_targets(builder, inputs);

    // Verify the merkle proof of the leaf we calculated
    let mut siblings: Vec<HashOutTarget> = Vec::new();
    for _ in 0..COMMITMENT_TREE_DEPTH {
        let sibling = builder.add_virtual_hash();
        siblings.push(sibling);
    }

    verify_merkle_proof_fixed_height(builder, commitment, own_leaf_hash, &siblings);

    ClaimTargets { amount, secret, nullifier_hash, commitment, siblings }
}
