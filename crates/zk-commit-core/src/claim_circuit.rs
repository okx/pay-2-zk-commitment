use crate::{
    circuit_utils::{
        get_hash_from_input_targets_circuit, verify_hash, verify_merkle_proof_circuit,
    },
    claim_execution::ClaimProvingInputs,
};
use plonky2::{
    field::extension::Extendable,
    hash::{
        hash_types::{HashOutTarget, RichField},
    },
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        config::{GenericHashOut},
    },
};

pub struct ClaimTargets {
    pub amount: Target,
    pub secret: Target,
    pub nullifier_hash: HashOutTarget,
    pub commitment: HashOutTarget,
    pub siblings: Vec<HashOutTarget>,
    pub index_target: Target,
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
pub fn generate_claim_circuit<F: RichField + Extendable<D>, const D: usize>(
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

    // Calculate my own leaf hash
    let inputs = vec![vec![amount], nullifier_hash.elements.to_vec(), vec![secret]].concat();
    let own_leaf_hash = get_hash_from_input_targets_circuit(builder, inputs);

    // Verify the merkle proof of the leaf we calculated
    let mut siblings: Vec<HashOutTarget> = Vec::new();
    for _ in 0..merkle_tree_depth {
        let sibling = builder.add_virtual_hash();
        siblings.push(sibling);
    }

    // Get the index bits up to the merkle tree depth number of bits from Little endian representation
    let index_target = builder.add_virtual_target();
    let index_bits = builder.split_le(index_target, merkle_tree_depth);

    // Verify merkle proof in the circuit
    verify_merkle_proof_circuit(builder, commitment, own_leaf_hash, &index_bits, &siblings);

    ClaimTargets { amount, secret, nullifier_hash, commitment, siblings, index_target }
}

/// Set the partial witness targets for the claim circuit. This includes the public inputs. For a claim, we set the amount, nullifier_hash and the commitment tree root as
/// the public inputs and the secret,
pub fn set_claim_circuit<F: RichField + Extendable<D>, const D: usize>(
    claim_targets: ClaimTargets,
    claim_proving_inputs: ClaimProvingInputs<F, D>,
    pw: &mut PartialWitness<F>,
) {
    pw.set_target(claim_targets.amount, claim_proving_inputs.pair.get_amount());
    pw.set_target(claim_targets.secret, claim_proving_inputs.pair.get_secret());
    pw.set_hash_target(claim_targets.commitment, claim_proving_inputs.commitment);
    pw.set_hash_target(claim_targets.nullifier_hash, claim_proving_inputs.nullifier_hash);

    pw.set_target(claim_targets.index_target, claim_proving_inputs.index);

    for i in 0..claim_targets.siblings.len() {
        pw.set_hash_target(
            *claim_targets.siblings.get(i).unwrap(),
            *claim_proving_inputs.commitment_merkle_proof.get(i).unwrap(),
        );
    }
}

pub struct ClaimProofResponse {}

#[cfg(test)]
mod test {
    use crate::{circuit_config::D, circuit_utils::run_circuit_test, claim_execution::{get_claim_proving_inputs, Claim}, commitment_tree::CommitmentTree, types::F, utils::AmountSecretPairing};
    use plonky2::{field::types::Field};

    use super::{generate_claim_circuit, set_claim_circuit};



    #[test]
    fn test_claim_circuit() {
        run_circuit_test(|builder, pw| {
            let distribution = vec![
                AmountSecretPairing { amount: F::ONE, secret: F::ZERO },
                AmountSecretPairing { amount: F::ONE, secret: F::ONE },
                AmountSecretPairing { amount: F::ONE, secret: F::TWO },
                AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(3) },
                AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(4) },
                AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(5) },
                AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(6) },
                AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(7) },
            ];

            let commitment_tree = CommitmentTree::<F,D>::new_from_distribution(&distribution);
            let claim = Claim::<F,D> {
                pair: *distribution.get(0).unwrap(),
                commitment: commitment_tree.get_root(),
                commitment_merkle_proof: commitment_tree.get_siblings(0),
                index: 0,
            };

            let claim_proving_inputs = get_claim_proving_inputs(claim);

            let claim_targets = generate_claim_circuit(builder, commitment_tree.depth);
            // println!("{:?}", commitment_tree.depth);
            set_claim_circuit(claim_targets, claim_proving_inputs, pw);
        });
    }
}
