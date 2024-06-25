use core::panic;

use plonky2::{
    field::extension::Extendable,
    hash::{
        hash_types::{HashOutTarget, RichField},
        poseidon::PoseidonHash,
    },
    iop::{
        target::{BoolTarget, Target},
        witness::PartialWitness,
    },
    plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitData, config::GenericConfig},
    util::timing::TimingTree,
};

use crate::{
    circuit_config::{D, STANDARD_CONFIG},
    types::{C, F},
};
use log::Level;
use plonky2::plonk::prover::prove;

/// Builds a merkle tree of a given size (based on the number of leaves) with the leaf hash targets as an input and returns the merkle tree as a vector.
pub fn build_merkle_tree(
    builder: &mut CircuitBuilder<F, D>,
    leaves: &mut Vec<HashOutTarget>,
) -> Vec<HashOutTarget> {
    //Create the merkle tree represented as a vector with the leaves at the front and the root at the back
    let mut tree: Vec<HashOutTarget> = Vec::new();

    let num_leaves = leaves.len();

    for i in 0..(num_leaves * 2 - 1) {
        if i < leaves.len() {
            tree.push(*leaves.get(i).unwrap());
        } else {
            let left_child_index = 2 * (i - num_leaves);
            let right_child_index = 2 * (i - num_leaves) + 1;
            let left_child = tree.get(left_child_index).unwrap();
            let right_child = tree.get(right_child_index).unwrap();
            tree.push(hash_2_subhashes_circuit(builder, left_child, right_child));
        }
    }

    return tree;
}

/// Given the siblings in a merkle tree and my root hash, verify the merkle proof of inclusion of the supplied leaf hash.
/// Since the order of the hash depends on my siblings position, we use the index bits of the leaf to determine the order of the
/// hash inputs.
pub fn verify_merkle_proof_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    merkle_root: HashOutTarget,
    mut leaf_hash: HashOutTarget,
    index_bits: &[BoolTarget],
    siblings: &Vec<HashOutTarget>,
) {
    for i in 0..siblings.len() {
        let sibling = siblings.get(i).unwrap();

        // Order is based on the index bits at the ith index
        let first_hash = select_hash(builder, index_bits[i], *sibling, leaf_hash);
        let second_hash = select_hash(builder, index_bits[i], leaf_hash, *sibling);
        leaf_hash = hash_2_subhashes_circuit(builder, &first_hash, &second_hash);
    }

    builder.connect_hashes(leaf_hash, merkle_root);
}

/// Verify a hash is calculated correctly by hashing the inputs and connecting the output hash wire with the input hash. This
/// function only verifies poseidon hashes.
pub fn verify_hash<
F: RichField + Extendable<D>,
const D: usize,
>(
    builder: &mut CircuitBuilder<F, D>,
    inputs: Vec<Target>,
    hash_to_verify: HashOutTarget,
) {
    let hash_calculated = get_hash_from_input_targets_circuit(builder, inputs);

    builder.connect_hashes(hash_to_verify, hash_calculated);
}

/// Get Hash target by doing a poseidon hash on my input vector.
pub fn get_hash_from_input_targets_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    inputs: Vec<Target>,
) -> HashOutTarget {
    builder.hash_n_to_hash_no_pad::<PoseidonHash>(inputs)
}

/// Hash 2 hashout targets by splitting it into its individual component elements
pub fn hash_2_subhashes_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    hash_1: &HashOutTarget,
    hash_2: &HashOutTarget,
) -> HashOutTarget {
    let inputs = vec![hash_1.elements.to_vec(), hash_2.elements.to_vec()].concat();
    get_hash_from_input_targets_circuit(builder, inputs)
}

/// Test runner for ease of testing
pub fn run_circuit_test<T, F, const D: usize>(test: T) -> ()
where
    T: FnOnce(&mut CircuitBuilder<F, D>, &mut PartialWitness<F>) -> () + panic::UnwindSafe,
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    let mut builder = CircuitBuilder::<F, D>::new(STANDARD_CONFIG);
    let mut pw: PartialWitness<F> = PartialWitness::<F>::new();
    test(&mut builder, &mut pw);
    builder.print_gate_counts(0);
    let mut timing = TimingTree::new("prove", Level::Debug);
    let data = builder.build::<C>();
    let CircuitData { prover_only, common, verifier_only: _ } = &data;
    let proof = prove(&prover_only, &common, pw, &mut timing).expect("Prove fail");
    timing.print();
    data.verify(proof).expect("Verify fail")
}

/// Computes `if b { h0 } else { h1 }`.
pub fn select_hash<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    b: BoolTarget,
    h0: HashOutTarget,
    h1: HashOutTarget,
) -> HashOutTarget {
    HashOutTarget {
        elements: core::array::from_fn(|i| builder.select(b, h0.elements[i], h1.elements[i])),
    }
}

#[cfg(test)]
mod test {
    use plonky2::{
        hash::hash_types::{HashOut, HashOutTarget},
        iop::witness::WitnessWrite,
    };

    use crate::{commitment_tree::CommitmentTree, types::F, utils::AmountSecretPairing};

    use super::{
        get_hash_from_input_targets_circuit, hash_2_subhashes_circuit, run_circuit_test,
        verify_hash, verify_merkle_proof_circuit,
    };
    use plonky2::field::types::Field;

    #[test]
    fn test_get_hash_from_input_targets_circuit() {
        run_circuit_test(|builder, pw| {
            let target_1 = builder.add_virtual_target();
            let target_2 = builder.add_virtual_target();
            let hash_target = builder.add_virtual_hash();
            let calculated_hash_target =
                get_hash_from_input_targets_circuit(builder, vec![target_1, target_2]);
            builder.connect_hashes(hash_target, calculated_hash_target);

            let value_1 = F::ZERO;
            let value_2 = F::ZERO;

            let hash = HashOut::from_vec(vec![
                F::from_canonical_u64(4330397376401421145),
                F::from_canonical_u64(14124799381142128323),
                F::from_canonical_u64(8742572140681234676),
                F::from_canonical_u64(14345658006221440202),
            ]);

            pw.set_target(target_1, value_1);
            pw.set_target(target_2, value_2);
            pw.set_hash_target(hash_target, hash);
        });
    }

    #[test]
    fn test_verify_hash_circuit() {
        run_circuit_test(|builder, pw| {
            let target_1 = builder.add_virtual_target();
            let target_2 = builder.add_virtual_target();
            let hash_target = builder.add_virtual_hash();

            verify_hash(builder, vec![target_1, target_2], hash_target);

            let value_1 = F::ZERO;
            let value_2 = F::ZERO;

            let hash = HashOut::from_vec(vec![
                F::from_canonical_u64(4330397376401421145),
                F::from_canonical_u64(14124799381142128323),
                F::from_canonical_u64(8742572140681234676),
                F::from_canonical_u64(14345658006221440202),
            ]);

            pw.set_target(target_1, value_1);
            pw.set_target(target_2, value_2);
            pw.set_hash_target(hash_target, hash);
        });
    }

    #[test]
    fn test_hash_2_subhashes_circuit() {
        run_circuit_test(|builder, pw| {
            let hash_target_1 = builder.add_virtual_hash();
            let hash_target_2 = builder.add_virtual_hash();
            let hash_target_3 = builder.add_virtual_hash();
            let calculated_hash_target =
                hash_2_subhashes_circuit(builder, &hash_target_1, &hash_target_2);
            builder.connect_hashes(hash_target_3, calculated_hash_target);

            let hash_1 = HashOut::from_vec(vec![
                F::from_canonical_u64(4330397376401421145),
                F::from_canonical_u64(14124799381142128323),
                F::from_canonical_u64(8742572140681234676),
                F::from_canonical_u64(14345658006221440202),
            ]);
            let hash_2 = HashOut::from_vec(vec![
                F::from_canonical_u64(4330397376401421145),
                F::from_canonical_u64(14124799381142128323),
                F::from_canonical_u64(8742572140681234676),
                F::from_canonical_u64(14345658006221440202),
            ]);

            let hash_3 = HashOut::from_vec(vec![
                F::from_canonical_u64(13121882728673923020),
                F::from_canonical_u64(10197653806804742863),
                F::from_canonical_u64(16037207047953124082),
                F::from_canonical_u64(2420399206709257475),
            ]);
            pw.set_hash_target(hash_target_1, hash_1);
            pw.set_hash_target(hash_target_2, hash_2);
            pw.set_hash_target(hash_target_3, hash_3);
        });
    }

    // #[test]
    // fn test_verify_merkle_proof_circuit() {
    //     run_circuit_test(|builder, pw| {
    //         let distribution = vec![
    //             AmountSecretPairing { amount: F::ONE, secret: F::ZERO },
    //             AmountSecretPairing { amount: F::ONE, secret: F::ONE },
    //             AmountSecretPairing { amount: F::ONE, secret: F::TWO },
    //             AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(3) },
    //             AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(4) },
    //             AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(5) },
    //             AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(6) },
    //             AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(7) },
    //         ];

    //         let commitment_tree = CommitmentTree::new_from_distribution(&distribution);
    //         let merkle_proof = commitment_tree.get_siblings(6);
    //         let leaf_hash = distribution.get(6).unwrap().get_own_hash();
    //         let merkle_root = commitment_tree.get_root();

    //         let mut merkle_proof_targets: Vec<HashOutTarget> = Vec::new();
    //         for _ in 0..merkle_proof.len() {
    //             let hash_target = builder.add_virtual_hash();
    //             merkle_proof_targets.push(hash_target);
    //         }
    //         let merkle_root_target = builder.add_virtual_hash();
    //         let leaf_hash_target = builder.add_virtual_hash();

    //         let index_target = builder.add_virtual_target();
    //         let index_bits = builder.split_le(index_target, 3);
    //         verify_merkle_proof_circuit(
    //             builder,
    //             merkle_root_target,
    //             leaf_hash_target,
    //             &index_bits,
    //             &merkle_proof_targets,
    //         );

    //         for i in 0..merkle_proof.len() {
    //             let hash_target = merkle_proof_targets.get(i).unwrap();
    //             pw.set_hash_target(*hash_target, *merkle_proof.get(i).unwrap());
    //         }

    //         pw.set_hash_target(merkle_root_target, merkle_root);
    //         pw.set_hash_target(leaf_hash_target, leaf_hash);
    //         pw.set_target(index_target, F::from_canonical_u64(6));
    //     });
    // }
}
