use plonky2::{
    hash::hash_types::HashOutTarget,
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::circuit_builder::CircuitBuilder,
};

use crate::{
    circuit_config::D,
    circuit_utils::{get_hash_from_input_targets_circuit, verify_hash, verify_merkle_proof_circuit},
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
    pub index_target: Target
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
pub fn generate_claim_circuit(
    builder: &mut CircuitBuilder<F, D>,
    merkle_tree_depth: usize,
) -> ClaimTargets {
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
    let own_leaf_hash_calculated = get_hash_from_input_targets_circuit(builder, inputs);
    let own_leaf_hash = builder.add_virtual_hash();
    builder.connect_hashes(own_leaf_hash_calculated, own_leaf_hash);

    // Verify the merkle proof of the leaf we calculated
    let mut siblings: Vec<HashOutTarget> = Vec::new();
    for _ in 0..merkle_tree_depth - 1 {
        let sibling = builder.add_virtual_hash();
        siblings.push(sibling);
    }

    let index_target = builder.add_virtual_target();
    let index_bits = builder.split_le(index_target, merkle_tree_depth);

    verify_merkle_proof_circuit(builder, commitment, own_leaf_hash_calculated, &index_bits, &siblings);

    ClaimTargets {
        amount,
        secret,
        nullifier_hash,
        commitment,
        siblings,
        own_leaf_hash,
        index_target
    }
}

/// Set the partial witness targets for the claim circuit. This includes the public inputs.
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

    for i in 0..claim_targets.siblings.len() - 1 {
        pw.set_hash_target(
            *claim_targets.siblings.get(i).unwrap(),
            *claim_proving_inputs.commitment_merkle_proof.get(i).unwrap(),
        );
    }
}