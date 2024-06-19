use plonky2::{
    hash::{hash_types::HashOut, poseidon::PoseidonHash},
    plonk::config::Hasher,
};

use crate::types::F;

/// A pair of amount and secret representing the amount of allocation of tokens to a specific amount
#[derive(Debug, Clone, Copy)]
pub struct AmountSecretPairing {
    pub amount: F,
    pub secret: F,
}

impl AmountSecretPairing {
    pub fn get_nullifier_hash(&self) -> HashOut<F> {
        let inputs = vec![self.get_secret(), self.get_amount()];
        hash_inputs(inputs)
    }

    pub fn get_amount(&self) -> F {
        self.amount
    }

    pub fn get_secret(&self) -> F {
        self.secret
    }

    pub fn get_own_hash(&self) -> HashOut<F> {
        let nullifier_hash = self.get_nullifier_hash();
        let inputs = vec![
            vec![self.get_amount()],
            nullifier_hash.elements.to_vec(),
            vec![self.get_secret()],
        ]
        .concat();

        let own_leaf_hash = hash_inputs(inputs);
        return own_leaf_hash;
    }
}

pub fn hash_2_subhashes(hash_1: &HashOut<F>, hash_2: &HashOut<F>) -> HashOut<F> {
    let inputs = vec![hash_1.elements.to_vec(), hash_2.elements.to_vec()].concat();
    hash_inputs(inputs)
}

pub fn hash_inputs(inputs: Vec<F>) -> HashOut<F> {
    let hash = PoseidonHash::hash_no_pad(inputs.as_slice());
    return hash;
}

#[cfg(test)]
mod test {
    use crate::{
        types::F,
        utils::{hash_2_subhashes, AmountSecretPairing},
    };
    use plonky2::{field::types::Field, hash::hash_types::HashOut};

    use super::hash_inputs;

    #[test]
    fn test_hash_inputs() {
        let inputs = vec![F::ZERO, F::ZERO];
        let hash = HashOut::from_vec(vec![
            F::from_canonical_u64(4330397376401421145),
            F::from_canonical_u64(14124799381142128323),
            F::from_canonical_u64(8742572140681234676),
            F::from_canonical_u64(14345658006221440202),
        ]);

        let hash_2 = hash_inputs(inputs);

        assert_eq!(hash, hash_2);
    }

    #[test]
    fn test_hash_2_subhashes() {
        let hash = HashOut::from_vec(vec![
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

        let hash_of_hashes = HashOut::from_vec(vec![
            F::from_canonical_u64(13121882728673923020),
            F::from_canonical_u64(10197653806804742863),
            F::from_canonical_u64(16037207047953124082),
            F::from_canonical_u64(2420399206709257475),
        ]);

        let hash_of_hashes_2 = hash_2_subhashes(&hash, &hash_2);

        assert_eq!(hash_of_hashes, hash_of_hashes_2);
    }

    #[test]
    fn test_nullifier_hash() {
        let pair = AmountSecretPairing {
            amount: F::ZERO,
            secret: F::ZERO,
        };

        let hash = HashOut::from_vec(vec![
            F::from_canonical_u64(4330397376401421145),
            F::from_canonical_u64(14124799381142128323),
            F::from_canonical_u64(8742572140681234676),
            F::from_canonical_u64(14345658006221440202),
        ]);

        let nullifer_hash = pair.get_nullifier_hash();

        assert_eq!(hash, nullifer_hash);
    }

    #[test]
    fn test_get_own_hash() {
        let hash = HashOut::from_vec(vec![
            F::from_canonical_u64(16782238164422600234),
            F::from_canonical_u64(1257922520496225283),
            F::from_canonical_u64(18156851804512452712),
            F::from_canonical_u64(4105490434607387523),
        ]);

        let pair = AmountSecretPairing {
            amount: F::ZERO,
            secret: F::ZERO,
        };

        let calc_hash = pair.get_own_hash();

        assert_eq!(hash, calc_hash);
    }
}
