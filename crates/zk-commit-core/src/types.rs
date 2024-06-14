use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

use crate::circuit_config::D;

pub type C = PoseidonGoldilocksConfig;
pub type F = <C as GenericConfig<D>>::F;
