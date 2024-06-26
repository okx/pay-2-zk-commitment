use crate::{claim_execution::Claim, commitment_tree::CommitmentTree, utils::AmountSecretPairing};


#[derive(Debug, Clone)]
pub struct Commitment {
    pub distribution: Vec<AmountSecretPairing>,
    pub commitment_tree: CommitmentTree,
}

impl Commitment {
    /// Get the claim for a given index within the committment tree. A claim includes the amount-secret pair, the commitment root and the merkle proof of inclusion of the
    /// specific index (siblings).
    pub fn get_claim(&self, index: usize) -> Claim {
        assert!(index < self.distribution.len());
        Claim {
            pair: *self.distribution.get(index as usize).unwrap(),
            commitment: self.commitment_tree.get_root(),
            commitment_merkle_proof: self.commitment_tree.get_siblings(index),
            index,
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use crate::{
        commitment_tree::CommitmentTree, types::F, utils::AmountSecretPairing,
    };

    use plonky2::field::types::Field;

    use super::Commitment;

    #[test]
    fn test_get_claim() {
        let distribution: Vec<AmountSecretPairing> = vec![
            AmountSecretPairing { amount: F::ONE, secret: F::ZERO },
            AmountSecretPairing { amount: F::ONE, secret: F::ONE },
            AmountSecretPairing { amount: F::ONE, secret: F::TWO },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(3) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(4) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(5) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(6) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(7) },
        ];

        let commitment_tree = CommitmentTree::new_from_distribution(distribution.borrow());
        let commitment = Commitment { commitment_tree, distribution: distribution.clone() };

        let claim = commitment.get_claim(0);
        assert_eq!(claim.pair.get_amount(), distribution.get(0).unwrap().get_amount());
        assert_eq!(claim.pair.get_secret(), distribution.get(0).unwrap().get_secret());
    }
}
