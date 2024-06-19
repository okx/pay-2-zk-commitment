use plonky2::{hash::{hash_types::HashOut, poseidon::PoseidonHash}, plonk::config::Hasher};

use crate::{circuit_config::NUM_LEAVES_MERKLE_TREE, claim_execution::Claim, types::F};

#[derive(Debug, Clone)]
pub struct Commitment{
    pub distribution: Vec<AmountSecretPairing>,
    pub commitment_root: HashOut<F>,
    pub commitment_tree: Vec<HashOut<F>>
}

impl Commitment{
    /// Get the claim for a given index within the committment tree
    pub fn get_claim(&self, index:usize)->Claim{
        assert!(index < self.distribution.len());
        Claim{
            pair: *self.distribution.get(index).unwrap(),
            commitment: self.commitment_root,
            siblings: self.get_siblings(index),
        }
    }

    /// Get the siblings for the merkle proof of inclusion given a leaf index
    pub fn get_siblings(&self, mut index:usize)->Vec<HashOut<F>>{
        let mut siblings = Vec::new();
        while index < self.commitment_tree.len()-1{
            let parent = index/2 + NUM_LEAVES_MERKLE_TREE;
            index = parent;
            if parent%2 == 1{
                let sibling_index = parent - 1;
                let sibling = self.commitment_tree.get(sibling_index).unwrap();
                siblings.push(*sibling);
            }else{
                let sibling_index = parent + 1;
                let sibling = self.commitment_tree.get(sibling_index).unwrap();
                siblings.push(*sibling);
            }
        }
        return siblings;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AmountSecretPairing{
    pub amount: F,
    pub secret: F
}

impl AmountSecretPairing{
    pub fn get_nullifier_hash(&self)->HashOut<F>{
        PoseidonHash::hash_no_pad(&[self.secret, self.amount])
    }

    pub fn get_amount(&self)->F{
        self.amount
    }

    pub fn get_secret(&self)->F{
        self.secret
    }
}
