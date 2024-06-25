use plonky2::{
    fri::{reduction_strategies::FriReductionStrategy, FriConfig},
    plonk::{
        circuit_data::CircuitConfig,
        config::{GenericConfig, PoseidonGoldilocksConfig},
    },
};

pub type C = PoseidonGoldilocksConfig;
pub type F = <C as GenericConfig<D>>::F;

// Extension of size 2
pub const D: usize = 2;
pub const COMMITMENT_TREE_DEPTH: usize = 4;
pub const NUM_LEAVES_MERKLE_TREE: usize = 1 << (COMMITMENT_TREE_DEPTH - 1);

pub const STANDARD_CONFIG: CircuitConfig = CircuitConfig {
    num_wires: 170,
    num_routed_wires: 80,
    num_constants: 2,
    use_base_arithmetic_gate: true,
    security_bits: 100,
    num_challenges: 2,
    zero_knowledge: false,
    max_quotient_degree_factor: 8,
    fri_config: FriConfig {
        rate_bits: 3,
        cap_height: 4,
        proof_of_work_bits: 16,
        reduction_strategy: FriReductionStrategy::ConstantArityBits(4, 5),
        num_query_rounds: 28,
    },
};
