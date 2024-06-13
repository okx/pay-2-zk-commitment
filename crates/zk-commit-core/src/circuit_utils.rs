use plonky2::{hash::{hash_types::HashOutTarget, poseidon::PoseidonHash}, iop::target::Target, plonk::circuit_builder::CircuitBuilder};

use crate::{circuit_config::{D, NUM_LEAVES_MERKLE_TREE}, types::F};



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

/// Given the siblings in a merkle tree and my root hash, verify the merkle proof of inclusion of the supplied leaf hash.
pub fn verify_merkle_proof_fixed_height(
    builder: &mut CircuitBuilder<F, D>,
    merkle_root: HashOutTarget,
    mut leaf_hash: HashOutTarget,
    siblings: &Vec<HashOutTarget>
){
    for i in 0..siblings.len(){
        let sibling = siblings.get(i).unwrap();
        leaf_hash = hash_2_subhashes(builder, &leaf_hash, sibling);
    }

    builder.connect_hashes(leaf_hash, merkle_root);
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