use plonky2::{
    hash::poseidon_bn128::PoseidonBN128GoldilocksConfig,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};

pub type C = PoseidonGoldilocksConfig;
pub type F = <C as GenericConfig<D>>::F;
pub type Cbn128 = PoseidonBN128GoldilocksConfig;

// Extension of size 2
pub const D: usize = 2;
