use std::{fmt::Write, path::PathBuf};

use anyhow::Result;
use log::Level;
use plonky2::{
    field::{
        extension::{Extendable, FieldExtension},
        types::Field,
    },
    gates::noop::NoopGate,
    hash::hash_types::RichField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{
            CircuitConfig, CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
        },
        config::{AlgebraicHasher, GenericConfig, GenericHashOut, Hasher},
        proof::ProofWithPublicInputs,
        prover::prove,
    },
    util::timing::TimingTree,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct VerifierConfig {
    hash_size: usize,
    field_size: usize,
    ext_field_size: usize,
    merkle_height_size: usize,

    num_wires_cap: usize,
    num_plonk_zs_partial_products_cap: usize,
    num_quotient_polys_cap: usize,

    // openings
    num_openings_constants: usize,
    num_openings_plonk_sigmas: usize,
    num_openings_wires: usize,
    num_openings_plonk_zs: usize,
    num_openings_plonk_zs_next: usize,
    num_openings_partial_products: usize,
    num_openings_quotient_polys: usize,

    // fri proof
    // .commit phase
    num_fri_commit_round: usize,
    fri_commit_merkle_cap_height: usize,
    // .query round
    num_fri_query_round: usize,
    // ..init
    num_fri_query_init_constants_sigmas_v: usize,
    num_fri_query_init_constants_sigmas_p: usize,
    num_fri_query_init_wires_v: usize,
    num_fri_query_init_wires_p: usize,
    num_fri_query_init_zs_partial_v: usize,
    num_fri_query_init_zs_partial_p: usize,
    num_fri_query_init_quotient_v: usize,
    num_fri_query_init_quotient_p: usize,
    // ..steps
    num_fri_query_step0_v: usize,
    num_fri_query_step0_p: usize,
    num_fri_query_step1_v: usize,
    num_fri_query_step1_p: usize,
    // .final poly
    num_fri_final_poly_ext_v: usize,
    // public inputs
    pub num_public_inputs: usize,
}

#[derive(Serialize)]
pub struct ProofForCircom {
    wires_cap: Vec<Vec<String>>,
    plonk_zs_partial_products_cap: Vec<Vec<String>>,
    quotient_polys_cap: Vec<Vec<String>>,

    openings_constants: Vec<Vec<String>>,
    openings_plonk_sigmas: Vec<Vec<String>>,
    openings_wires: Vec<Vec<String>>,
    openings_plonk_zs: Vec<Vec<String>>,
    openings_plonk_zs_next: Vec<Vec<String>>,
    openings_partial_products: Vec<Vec<String>>,
    openings_quotient_polys: Vec<Vec<String>>,

    fri_commit_phase_merkle_caps: Vec<Vec<Vec<String>>>,

    fri_query_init_constants_sigmas_v: Vec<Vec<String>>,
    fri_query_init_constants_sigmas_p: Vec<Vec<Vec<String>>>,
    fri_query_init_wires_v: Vec<Vec<String>>,
    fri_query_init_wires_p: Vec<Vec<Vec<String>>>,
    fri_query_init_zs_partial_v: Vec<Vec<String>>,
    fri_query_init_zs_partial_p: Vec<Vec<Vec<String>>>,
    fri_query_init_quotient_v: Vec<Vec<String>>,
    fri_query_init_quotient_p: Vec<Vec<Vec<String>>>,

    fri_query_step0_v: Vec<Vec<Vec<String>>>,
    fri_query_step0_p: Vec<Vec<Vec<String>>>,
    fri_query_step1_v: Vec<Vec<Vec<String>>>,
    fri_query_step1_p: Vec<Vec<Vec<String>>>,

    fri_final_poly_ext_v: Vec<Vec<String>>,
    fri_pow_witness: String,

    public_inputs: Vec<String>,
}

pub fn generate_verifier_config<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    const D: usize,
>(
    pwpi: &ProofWithPublicInputs<F, C, D>,
) -> anyhow::Result<VerifierConfig> {
    let proof = &pwpi.proof;
    assert_eq!(proof.opening_proof.query_round_proofs[0].steps.len(), 2);

    const HASH_SIZE: usize = 32;
    const FIELD_SIZE: usize = 8; // in bytes
    const EXT_FIELD_SIZE: usize = 16;
    const MERKLE_HEIGHT_SIZE: usize = 1;

    let query_round_init_trees =
        &proof.opening_proof.query_round_proofs[0].initial_trees_proof.evals_proofs;
    let query_round_steps = &proof.opening_proof.query_round_proofs[0].steps;

    let conf = VerifierConfig {
        hash_size: HASH_SIZE,
        field_size: FIELD_SIZE,
        ext_field_size: EXT_FIELD_SIZE,
        merkle_height_size: MERKLE_HEIGHT_SIZE,

        num_wires_cap: proof.wires_cap.0.len(),
        num_plonk_zs_partial_products_cap: proof.plonk_zs_partial_products_cap.0.len(),
        num_quotient_polys_cap: proof.quotient_polys_cap.0.len(),

        num_openings_constants: proof.openings.constants.len(),
        num_openings_plonk_sigmas: proof.openings.plonk_sigmas.len(),
        num_openings_wires: proof.openings.wires.len(),
        num_openings_plonk_zs: proof.openings.plonk_zs.len(),
        num_openings_plonk_zs_next: proof.openings.plonk_zs_next.len(),
        num_openings_partial_products: proof.openings.partial_products.len(),
        num_openings_quotient_polys: proof.openings.quotient_polys.len(),

        num_fri_commit_round: proof.opening_proof.commit_phase_merkle_caps.len(),
        fri_commit_merkle_cap_height: proof.opening_proof.commit_phase_merkle_caps[0].0.len(),
        num_fri_query_round: proof.opening_proof.query_round_proofs.len(),
        num_fri_query_init_constants_sigmas_v: query_round_init_trees[0].0.len(),
        num_fri_query_init_constants_sigmas_p: query_round_init_trees[0].1.siblings.len(),
        num_fri_query_init_wires_v: query_round_init_trees[1].0.len(),
        num_fri_query_init_wires_p: query_round_init_trees[1].1.siblings.len(),
        num_fri_query_init_zs_partial_v: query_round_init_trees[2].0.len(),
        num_fri_query_init_zs_partial_p: query_round_init_trees[2].1.siblings.len(),
        num_fri_query_init_quotient_v: query_round_init_trees[3].0.len(),
        num_fri_query_init_quotient_p: query_round_init_trees[3].1.siblings.len(),
        num_fri_query_step0_v: query_round_steps[0].evals.len(),
        num_fri_query_step0_p: query_round_steps[0].merkle_proof.siblings.len(),
        num_fri_query_step1_v: query_round_steps[1].evals.len(),
        num_fri_query_step1_p: query_round_steps[1].merkle_proof.siblings.len(),
        num_fri_final_poly_ext_v: proof.opening_proof.final_poly.coeffs.len(),

        num_public_inputs: pwpi.public_inputs.len() / 2, // every two gl is packed into one slot
    };
    Ok(conf)
}

pub fn generate_proof_base64<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    const D: usize,
>(
    pwpi: &ProofWithPublicInputs<F, C, D>,
    conf: &VerifierConfig,
) -> anyhow::Result<String> {
    let mut proof_size: usize =
        (conf.num_wires_cap + conf.num_plonk_zs_partial_products_cap + conf.num_quotient_polys_cap)
            * conf.hash_size;

    let mut wires_cap = vec![vec!["0".to_string(); 4]; conf.num_wires_cap];
    for i in 0..conf.num_wires_cap {
        let h = pwpi.proof.wires_cap.0[i].to_vec();
        for j in 0..h.len() {
            wires_cap[i][j] = h[j].to_canonical_u64().to_string();
        }
    }

    let mut plonk_zs_partial_products_cap =
        vec![vec!["0".to_string(); 4]; conf.num_plonk_zs_partial_products_cap];
    for i in 0..conf.num_plonk_zs_partial_products_cap {
        let h = pwpi.proof.plonk_zs_partial_products_cap.0[i].to_vec();
        for j in 0..h.len() {
            plonk_zs_partial_products_cap[i][j] = h[j].to_canonical_u64().to_string();
        }
    }

    let mut quotient_polys_cap = vec![vec!["0".to_string(); 4]; conf.num_quotient_polys_cap];
    for i in 0..conf.num_quotient_polys_cap {
        let h = pwpi.proof.quotient_polys_cap.0[i].to_vec();
        for j in 0..h.len() {
            quotient_polys_cap[i][j] = h[j].to_canonical_u64().to_string();
        }
    }

    proof_size += (conf.num_openings_constants
        + conf.num_openings_plonk_sigmas
        + conf.num_openings_wires
        + conf.num_openings_plonk_zs
        + conf.num_openings_plonk_zs_next
        + conf.num_openings_partial_products
        + conf.num_openings_quotient_polys)
        * conf.ext_field_size;

    let mut openings_constants = vec![vec!["0".to_string(); 2]; conf.num_openings_constants];
    for i in 0..conf.num_openings_constants {
        openings_constants[i][0] =
            pwpi.proof.openings.constants[i].to_basefield_array()[0].to_canonical_u64().to_string();
        openings_constants[i][1] =
            pwpi.proof.openings.constants[i].to_basefield_array()[1].to_canonical_u64().to_string();
    }
    let mut openings_plonk_sigmas = vec![vec!["0".to_string(); 2]; conf.num_openings_plonk_sigmas];
    for i in 0..conf.num_openings_plonk_sigmas {
        openings_plonk_sigmas[i][0] = pwpi.proof.openings.plonk_sigmas[i].to_basefield_array()[0]
            .to_canonical_u64()
            .to_string();
        openings_plonk_sigmas[i][1] = pwpi.proof.openings.plonk_sigmas[i].to_basefield_array()[1]
            .to_canonical_u64()
            .to_string();
    }
    let mut openings_wires = vec![vec!["0".to_string(); 2]; conf.num_openings_wires];
    for i in 0..conf.num_openings_wires {
        openings_wires[i][0] =
            pwpi.proof.openings.wires[i].to_basefield_array()[0].to_canonical_u64().to_string();
        openings_wires[i][1] =
            pwpi.proof.openings.wires[i].to_basefield_array()[1].to_canonical_u64().to_string();
    }
    let mut openings_plonk_zs = vec![vec!["0".to_string(); 2]; conf.num_openings_plonk_zs];
    for i in 0..conf.num_openings_plonk_zs {
        openings_plonk_zs[i][0] =
            pwpi.proof.openings.plonk_zs[i].to_basefield_array()[0].to_canonical_u64().to_string();
        openings_plonk_zs[i][1] =
            pwpi.proof.openings.plonk_zs[i].to_basefield_array()[1].to_canonical_u64().to_string();
    }
    let mut openings_plonk_zs_next =
        vec![vec!["0".to_string(); 2]; conf.num_openings_plonk_zs_next];
    for i in 0..conf.num_openings_plonk_zs_next {
        openings_plonk_zs_next[i][0] = pwpi.proof.openings.plonk_zs_next[i].to_basefield_array()[0]
            .to_canonical_u64()
            .to_string();
        openings_plonk_zs_next[i][1] = pwpi.proof.openings.plonk_zs_next[i].to_basefield_array()[1]
            .to_canonical_u64()
            .to_string();
    }
    let mut openings_partial_products =
        vec![vec!["0".to_string(); 2]; conf.num_openings_partial_products];
    for i in 0..conf.num_openings_partial_products {
        openings_partial_products[i][0] = pwpi.proof.openings.partial_products[i]
            .to_basefield_array()[0]
            .to_canonical_u64()
            .to_string();
        openings_partial_products[i][1] = pwpi.proof.openings.partial_products[i]
            .to_basefield_array()[1]
            .to_canonical_u64()
            .to_string();
    }
    let mut openings_quotient_polys =
        vec![vec!["0".to_string(); 2]; conf.num_openings_quotient_polys];
    for i in 0..conf.num_openings_quotient_polys {
        openings_quotient_polys[i][0] = pwpi.proof.openings.quotient_polys[i].to_basefield_array()
            [0]
        .to_canonical_u64()
        .to_string();
        openings_quotient_polys[i][1] = pwpi.proof.openings.quotient_polys[i].to_basefield_array()
            [1]
        .to_canonical_u64()
        .to_string();
    }

    proof_size += (conf.num_fri_commit_round * conf.fri_commit_merkle_cap_height) * conf.hash_size;

    let mut fri_commit_phase_merkle_caps =
        vec![
            vec![vec!["0".to_string(); 4]; conf.fri_commit_merkle_cap_height];
            conf.num_fri_commit_round
        ];
    for i in 0..conf.num_fri_commit_round {
        let h = pwpi.proof.opening_proof.commit_phase_merkle_caps[i].flatten();
        assert_eq!(h.len(), 4 * conf.fri_commit_merkle_cap_height);
        for j in 0..conf.fri_commit_merkle_cap_height {
            for k in 0..4 {
                fri_commit_phase_merkle_caps[i][j][k] = h[j * 4 + k].to_canonical_u64().to_string();
            }
        }
    }

    proof_size += conf.num_fri_query_round
        * ((conf.num_fri_query_init_constants_sigmas_v
            + conf.num_fri_query_init_wires_v
            + conf.num_fri_query_init_zs_partial_v
            + conf.num_fri_query_init_quotient_v)
            * conf.field_size
            + (conf.num_fri_query_init_constants_sigmas_p
                + conf.num_fri_query_init_wires_p
                + conf.num_fri_query_init_zs_partial_p
                + conf.num_fri_query_init_quotient_p)
                * conf.hash_size
            + conf.merkle_height_size * 4);

    proof_size += conf.num_fri_query_round
        * (conf.num_fri_query_step0_v * conf.ext_field_size
            + conf.num_fri_query_step0_p * conf.hash_size
            + conf.merkle_height_size
            + conf.num_fri_query_step1_v * conf.ext_field_size
            + conf.num_fri_query_step1_p * conf.hash_size
            + conf.merkle_height_size);

    let mut fri_query_init_constants_sigmas_v =
        vec![
            vec!["0".to_string(); conf.num_fri_query_init_constants_sigmas_v];
            conf.num_fri_query_round
        ];
    let mut fri_query_init_wires_v =
        vec![vec!["0".to_string(); conf.num_fri_query_init_wires_v]; conf.num_fri_query_round];
    let mut fri_query_init_zs_partial_v =
        vec![vec!["0".to_string(); conf.num_fri_query_init_zs_partial_v]; conf.num_fri_query_round];
    let mut fri_query_init_quotient_v =
        vec![vec!["0".to_string(); conf.num_fri_query_init_quotient_v]; conf.num_fri_query_round];

    let mut fri_query_init_constants_sigmas_p =
        vec![
            vec![vec!["0".to_string(); 4]; conf.num_fri_query_init_constants_sigmas_p];
            conf.num_fri_query_round
        ];
    let mut fri_query_init_wires_p =
        vec![
            vec![vec!["0".to_string(); 4]; conf.num_fri_query_init_wires_p];
            conf.num_fri_query_round
        ];
    let mut fri_query_init_zs_partial_p =
        vec![
            vec![vec!["0".to_string(); 4]; conf.num_fri_query_init_zs_partial_p];
            conf.num_fri_query_round
        ];
    let mut fri_query_init_quotient_p =
        vec![
            vec![vec!["0".to_string(); 4]; conf.num_fri_query_init_quotient_p];
            conf.num_fri_query_round
        ];

    let mut fri_query_step0_v =
        vec![vec![vec!["0".to_string(); 2]; conf.num_fri_query_step0_v]; conf.num_fri_query_round];
    let mut fri_query_step1_v =
        vec![vec![vec!["0".to_string(); 2]; conf.num_fri_query_step1_v]; conf.num_fri_query_round];
    let mut fri_query_step0_p =
        vec![vec![vec!["0".to_string(); 4]; conf.num_fri_query_step0_p]; conf.num_fri_query_round];
    let mut fri_query_step1_p =
        vec![vec![vec!["0".to_string(); 4]; conf.num_fri_query_step1_p]; conf.num_fri_query_round];

    for i in 0..conf.num_fri_query_round {
        assert_eq!(
            pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs.len(),
            4
        );
        for j in 0..conf.num_fri_query_init_constants_sigmas_v {
            fri_query_init_constants_sigmas_v[i][j] =
                pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs[0]
                    .0[j]
                    .to_canonical_u64()
                    .to_string();
        }
        for j in 0..conf.num_fri_query_init_wires_v {
            fri_query_init_wires_v[i][j] =
                pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs[1]
                    .0[j]
                    .to_canonical_u64()
                    .to_string();
        }
        for j in 0..conf.num_fri_query_init_zs_partial_v {
            fri_query_init_zs_partial_v[i][j] =
                pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs[2]
                    .0[j]
                    .to_canonical_u64()
                    .to_string();
        }
        for j in 0..conf.num_fri_query_init_quotient_v {
            fri_query_init_quotient_v[i][j] =
                pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs[3]
                    .0[j]
                    .to_canonical_u64()
                    .to_string();
        }
        for j in 0..conf.num_fri_query_init_constants_sigmas_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs
                [0]
            .1
            .siblings[j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_init_constants_sigmas_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
        for j in 0..conf.num_fri_query_init_wires_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs
                [1]
            .1
            .siblings[j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_init_wires_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
        for j in 0..conf.num_fri_query_init_zs_partial_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs
                [2]
            .1
            .siblings[j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_init_zs_partial_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
        for j in 0..conf.num_fri_query_init_quotient_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].initial_trees_proof.evals_proofs
                [3]
            .1
            .siblings[j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_init_quotient_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
        for j in 0..conf.num_fri_query_step0_v {
            fri_query_step0_v[i][j][0] = pwpi.proof.opening_proof.query_round_proofs[i].steps[0]
                .evals[j]
                .to_basefield_array()[0]
                .to_canonical_u64()
                .to_string();
            fri_query_step0_v[i][j][1] = pwpi.proof.opening_proof.query_round_proofs[i].steps[0]
                .evals[j]
                .to_basefield_array()[1]
                .to_canonical_u64()
                .to_string();
        }
        for j in 0..conf.num_fri_query_step1_v {
            fri_query_step1_v[i][j][0] = pwpi.proof.opening_proof.query_round_proofs[i].steps[1]
                .evals[j]
                .to_basefield_array()[0]
                .to_canonical_u64()
                .to_string();
            fri_query_step1_v[i][j][1] = pwpi.proof.opening_proof.query_round_proofs[i].steps[1]
                .evals[j]
                .to_basefield_array()[1]
                .to_canonical_u64()
                .to_string();
        }
        assert_eq!(pwpi.proof.opening_proof.query_round_proofs[i].steps.len(), 2);
        for j in 0..conf.num_fri_query_step0_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].steps[0].merkle_proof.siblings
                [j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_step0_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
        for j in 0..conf.num_fri_query_step1_p {
            let h = pwpi.proof.opening_proof.query_round_proofs[i].steps[1].merkle_proof.siblings
                [j]
                .to_vec();
            assert_eq!(h.len(), 4);
            for k in 0..4 {
                fri_query_step1_p[i][j][k] = h[k].to_canonical_u64().to_string();
            }
        }
    }

    proof_size += conf.num_fri_final_poly_ext_v * conf.ext_field_size;

    let mut fri_final_poly_ext_v = vec![vec!["0".to_string(); 2]; conf.num_fri_final_poly_ext_v];
    for i in 0..conf.num_fri_final_poly_ext_v {
        fri_final_poly_ext_v[i][0] = pwpi.proof.opening_proof.final_poly.coeffs[i]
            .to_basefield_array()[0]
            .to_canonical_u64()
            .to_string();
        fri_final_poly_ext_v[i][1] = pwpi.proof.opening_proof.final_poly.coeffs[i]
            .to_basefield_array()[1]
            .to_canonical_u64()
            .to_string();
    }

    proof_size += conf.field_size;

    proof_size += conf.num_public_inputs * conf.field_size;

    let mut public_inputs = vec!["0".to_string(); conf.num_public_inputs];
    for i in 0..conf.num_public_inputs {
        public_inputs[i] = pwpi.public_inputs[i].to_canonical_u64().to_string();
    }


    // proof_size += vd_proof.merkle_proof.siblings.len() * 4 * conf.field_size;

    let circom_proof = ProofForCircom {
        wires_cap,
        plonk_zs_partial_products_cap,
        quotient_polys_cap,
        openings_constants,
        openings_plonk_sigmas,
        openings_wires,
        openings_plonk_zs,
        openings_plonk_zs_next,
        openings_partial_products,
        openings_quotient_polys,
        fri_commit_phase_merkle_caps,
        fri_query_init_constants_sigmas_v,
        fri_query_init_constants_sigmas_p,
        fri_query_init_wires_v,
        fri_query_init_wires_p,
        fri_query_init_zs_partial_v,
        fri_query_init_zs_partial_p,
        fri_query_init_quotient_v,
        fri_query_init_quotient_p,
        fri_query_step0_v,
        fri_query_step0_p,
        fri_query_step1_v,
        fri_query_step1_p,
        fri_final_poly_ext_v,
        fri_pow_witness: pwpi.proof.opening_proof.pow_witness.to_canonical_u64().to_string(),
        public_inputs
    };

    let proof_bytes = pwpi.to_bytes();
    assert_eq!(proof_bytes.len(), proof_size);
    println!("proof size: {}", proof_size);

    Ok(serde_json::to_string(&circom_proof).unwrap())
}
