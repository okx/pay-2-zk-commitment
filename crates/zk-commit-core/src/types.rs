use plonky2::{
    field::{extension::quadratic::QuadraticExtension, goldilocks_field::GoldilocksField},
    hash::poseidon_bn128::PoseidonBN128Hash,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};

use crate::circuit_config::D;

/// Configuration using Poseidon over the Goldilocks field.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PoseidonBN128GoldilocksConfig;

impl GenericConfig<2> for PoseidonBN128GoldilocksConfig {
    type F = GoldilocksField;
    type FE = QuadraticExtension<F>;
    type Hasher = PoseidonBN128Hash;
    type InnerHasher = PoseidonBN128Hash;
}

pub type Cbn128 = PoseidonBN128GoldilocksConfig;
pub type C = PoseidonGoldilocksConfig;
pub type F = <C as GenericConfig<D>>::F;
