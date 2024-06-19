use plonky2::{hash::hash_types::HashOut, util::log2_strict};

use crate::{
    types::F,
    utils::{hash_2_subhashes, AmountSecretPairing},
};

/// Commitment
#[derive(Debug, Clone)]
pub struct CommitmentTree {
    pub depth: usize,
    pub commitment_root: HashOut<F>,
    pub commitment_tree: Vec<HashOut<F>>,
}

impl CommitmentTree {
    /// Get the root of the commitment tree.
    pub fn get_root(&self) -> HashOut<F> {
        return self.commitment_root;
    }

    /// Gets the leaf hash value given the index of the leaf
    pub fn get_from_index(&self, index: usize) -> Option<&HashOut<F>> {
        return self.commitment_tree.get(index);
    }

    /// Create a new commitment tree from a distribution of amounts-secret pairs, note that this distribution must be a power of 2.
    /// In the future we can try to pad with empty claims.
    pub fn new_from_distribution(distribution: &Vec<AmountSecretPairing>) -> CommitmentTree {
        let mut commitment_tree: Vec<HashOut<F>> = Vec::new();
        let num_leaves = distribution.len();

        for i in 0..num_leaves * 2 - 1 {
            if i < num_leaves {
                let dist = distribution.get(i).unwrap();
                let hash = dist.get_own_hash();
                commitment_tree.push(hash);
            } else {
                let left_child_index = 2 * (i - num_leaves);
                let right_child_index = 2 * (i - num_leaves) + 1;
                let left_child = commitment_tree.get(left_child_index).unwrap();
                let right_child = commitment_tree.get(right_child_index).unwrap();
                let hash = hash_2_subhashes(left_child, right_child);
                commitment_tree.push(hash);
            }
        }

        CommitmentTree {
            commitment_root: *commitment_tree.last().unwrap(),
            commitment_tree,
            depth: log2_strict(distribution.len()),
        }
    }

    /// Get the siblings for the merkle proof of inclusion given a leaf index.
    /// We get the parent index of a leaf using the formula: parent = index / 2 + num_leaves
    pub fn get_siblings(&self, mut index: usize) -> Vec<HashOut<F>> {
        let mut siblings = Vec::new();
        let num_leaves = 1 << self.depth;
        while index < self.commitment_tree.len() - 1 {
            if index % 2 == 1 {
                let sibling_index = index - 1;
                let sibling = self.commitment_tree.get(sibling_index).unwrap();
                siblings.push(*sibling);
            } else {
                let sibling_index = index + 1;
                let sibling = self.commitment_tree.get(sibling_index).unwrap();
                siblings.push(*sibling);
            }

            let parent = index / 2 + num_leaves;
            index = parent;
        }
        return siblings;
    }
}

#[cfg(test)]
mod test {
    use plonky2::{field::types::Field, hash::hash_types::HashOut};

    use crate::{
        types::F,
        utils::{hash_2_subhashes, AmountSecretPairing},
    };

    use super::CommitmentTree;

    #[test]
    fn test_new_from_distribution() {
        let distribution = vec![
            AmountSecretPairing { amount: F::ONE, secret: F::ZERO },
            AmountSecretPairing { amount: F::ONE, secret: F::ONE },
            AmountSecretPairing { amount: F::ONE, secret: F::TWO },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(3) },
        ];

        let mut first_layer_hashes: Vec<HashOut<F>> =
            distribution.iter().map(|x| x.get_own_hash()).collect();
        let mut second_layer_hashes: Vec<HashOut<F>> = first_layer_hashes
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < first_layer_hashes.len() / 2)
            .map(|(i, _)| {
                let hash_1 = first_layer_hashes.get(i * 2).unwrap();
                let hash_2 = first_layer_hashes.get(i * 2 + 1).unwrap();
                hash_2_subhashes(hash_1, hash_2)
            })
            .collect();
        let mut final_hash: Vec<HashOut<F>> = vec![hash_2_subhashes(
            second_layer_hashes.get(0).unwrap(),
            second_layer_hashes.get(1).unwrap(),
        )];
        let mut calculated_tree: Vec<HashOut<F>> = Vec::new();
        calculated_tree.append(&mut first_layer_hashes);
        calculated_tree.append(&mut second_layer_hashes);
        calculated_tree.append(&mut final_hash);

        let commitment_tree = CommitmentTree::new_from_distribution(&distribution);

        println!("CALCULATED TREE ROOT: {:?}", calculated_tree.last().unwrap());
        println!("COMMITMENT TREE ROOT: {:?}", commitment_tree.commitment_root);

        for i in 0..7 {
            assert_eq!(calculated_tree.get(i).unwrap(), commitment_tree.get_from_index(i).unwrap());
        }
    }

    #[test]
    fn test_get_siblings() {
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

        let commitment_tree = CommitmentTree::new_from_distribution(&distribution);

        let mut siblings_calculated: Vec<HashOut<F>> = Vec::new();
        siblings_calculated.push(*commitment_tree.get_from_index(0).unwrap());
        siblings_calculated.push(*commitment_tree.get_from_index(9).unwrap());
        siblings_calculated.push(*commitment_tree.get_from_index(13).unwrap());

        let siblings = commitment_tree.get_siblings(1);
        assert_eq!(siblings, siblings_calculated);
    }
}
