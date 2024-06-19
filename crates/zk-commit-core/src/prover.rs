use plonky2::{hash::{hash_types::HashOutTarget, poseidon::PoseidonHash}, iop::target::Target, plonk::{circuit_builder::CircuitBuilder, config::Hasher}};

use crate::{circuit_config::{D, NUM_LEAVES_MERKLE_TREE}, types::F};

pub struct ClaimTargets{
    pub amount: Target,
    pub secret: Target,
    pub nullifier_hash: HashOutTarget,
    pub commitment: HashOutTarget,
    pub merkle_leaves: Vec<HashOutTarget>,
}

/// Generates a circuit to verify the claim by doing the following:
/// -Verifies that the nullifier hash is correctly calculated 
/// -Verifies that the commitment is properly calculated given the leaves
/// -Verifies that the commitment is calculated with the claimaints own leaf hash given their index in the tree
/// The p
pub fn generate_claim_circuit(
    builder: &mut CircuitBuilder<F, D>,
    index: usize
)-> ClaimTargets{
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
    let commitment = builder.add_virtual_hash_public_input();

    // Calculate my own leaf hash
    let inputs = vec![vec![amount], nullifier_hash.elements.to_vec(), vec![secret]].concat();
    let own_leaf_hash = get_hash_from_input_targets(builder, inputs);

    // Build merkle leaves and connect our calculated leaf hash with the input hashes
    let mut merkle_leaves: Vec<HashOutTarget> = Vec::new();

    for i in 0..NUM_LEAVES_MERKLE_TREE{
        let leaf =  builder.add_virtual_hash();
        merkle_leaves.push(leaf);

        if i == index{
            builder.connect_hashes(own_leaf_hash, leaf);
        }
    }

    // Build merkle tree and check to see if calculated root is same as public input commitment
    let merkle_tree = build_merkle_tree_fixed_size(
        builder,
        &mut merkle_leaves
    );

    let calculated_root = merkle_tree.last().unwrap();
    builder.connect_hashes(*calculated_root, commitment);

    ClaimTargets{
        amount,
        secret,
        nullifier_hash,
        commitment,
        merkle_leaves
    }
}



/// Builds a merkle tree of a fixed size with the leaf hash targets as an input and returns the merkle tree as a vector.
pub fn build_merkle_tree_fixed_size(
    builder: &mut CircuitBuilder<F, D>,
    leaves: &mut Vec<HashOutTarget>,
)-> Vec<HashOutTarget>{

  //Create the merkle tree represented as a vector with the leaves at the front and the root at the back
  let mut tree: Vec<HashOutTarget> = Vec::new();
  tree.append(leaves);

  let num_leaves =  leaves.len();
  //Check num leaves is the correct amount
  assert_eq!(num_leaves, NUM_LEAVES_MERKLE_TREE);  

  for i in tree.len()..tree.len() + NUM_LEAVES_MERKLE_TREE - 1{
        let left_child_index =  2*(i - NUM_LEAVES_MERKLE_TREE);
        let right_child_index =  2*(i - NUM_LEAVES_MERKLE_TREE)+1;
        let left_child = tree.get(left_child_index).unwrap();
        let right_child = tree.get(right_child_index).unwrap();
        tree.push(hash_2_subhashes(builder, left_child, right_child));
  }

  return tree;
}


/// Verify a hash is calculated correctly by hashing the inputs and connecting the output hash wire with the input hash. This
/// function only verifies poseidon hashes.
pub fn verify_hash(
    builder: &mut CircuitBuilder<F, D>,
    inputs: Vec<Target>,
    hash_to_verify: HashOutTarget
){
    let hash_calculated = get_hash_from_input_targets(builder, inputs);

    builder.connect_hashes(hash_to_verify, hash_calculated);
}

/// Get Hash target by doing a poseidon hash on my input vector.
pub fn get_hash_from_input_targets(
    builder: &mut CircuitBuilder<F, D>,
    inputs: Vec<Target>,
)->HashOutTarget{
    builder.hash_n_to_hash_no_pad::<PoseidonHash>(
        inputs
    )
}

/// Hash 2 hashout targets by splitting it into its individual component elements
pub fn hash_2_subhashes(
    builder: &mut CircuitBuilder<F, D>,
    hash_1: &HashOutTarget,
    hash_2: &HashOutTarget
)->HashOutTarget{
    let inputs = vec![
        hash_1.elements.to_vec(),
        hash_2.elements.to_vec()
    ].concat();
    get_hash_from_input_targets(builder, inputs)
}