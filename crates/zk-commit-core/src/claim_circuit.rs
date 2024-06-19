use plonky2::{
    hash::hash_types::HashOutTarget,
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::circuit_builder::CircuitBuilder,
};

use crate::{
    circuit_config::{COMMITMENT_TREE_DEPTH, D},
    circuit_utils::{get_hash_from_input_targets, verify_hash, verify_merkle_proof_fixed_height},
    claim_execution::ClaimProvingInputs,
    types::F,
};

pub struct ClaimTargets {
    pub amount: Target,
    pub secret: Target,
    pub nullifier_hash: HashOutTarget,
    pub commitment: HashOutTarget,
    pub siblings: Vec<HashOutTarget>,
    pub own_leaf_hash: HashOutTarget,
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

    // Calculate my own leaf hash and verify it is correctly calculated
    let inputs = vec![vec![amount], nullifier_hash.elements.to_vec(), vec![secret]].concat();
    let own_leaf_hash_calculated = get_hash_from_input_targets(builder, inputs);
    let own_leaf_hash = builder.add_virtual_hash();
    builder.connect_hashes(own_leaf_hash_calculated, own_leaf_hash);

    // Verify the merkle proof of the leaf we calculated
    let mut siblings: Vec<HashOutTarget> = Vec::new();
    for _ in 0..COMMITMENT_TREE_DEPTH - 1 {
        let sibling = builder.add_virtual_hash();
        siblings.push(sibling);
    }

    verify_merkle_proof_fixed_height(builder, commitment, own_leaf_hash_calculated, &siblings);

    ClaimTargets {
        amount,
        secret,
        nullifier_hash,
        commitment,
        siblings,
        own_leaf_hash,
    }
}

/// Set the partial witness targets for the claim circuit
pub fn set_claim_circuit(
    claim_targets: ClaimTargets,
    claim_proving_inputs: ClaimProvingInputs,
    pw: &mut PartialWitness<F>,
) {
    pw.set_target(claim_targets.amount, claim_proving_inputs.pair.get_amount());
    pw.set_target(claim_targets.secret, claim_proving_inputs.pair.get_secret());
    pw.set_hash_target(claim_targets.commitment, claim_proving_inputs.commitment);
    pw.set_hash_target(
        claim_targets.nullifier_hash,
        claim_proving_inputs.nullifier_hash,
    );
    pw.set_hash_target(
        claim_targets.own_leaf_hash,
        claim_proving_inputs.own_leaf_hash,
    );

    assert_eq!(claim_targets.siblings.len(), COMMITMENT_TREE_DEPTH - 1);
    assert_eq!(
        claim_proving_inputs.siblings.len(),
        COMMITMENT_TREE_DEPTH - 1
    );

    for i in 0..COMMITMENT_TREE_DEPTH - 1 {
        pw.set_hash_target(
            *claim_targets.siblings.get(i).unwrap(),
            *claim_proving_inputs.siblings.get(i).unwrap(),
        );
    }
}
