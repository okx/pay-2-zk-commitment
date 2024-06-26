use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    plonk::{
        circuit_data::VerifierOnlyCircuitData,
        config::{GenericConfig, GenericHashOut},
        proof::ProofWithPublicInputs,
    },
};
use zk_commit_core::{
    circuit_config::{D, HIGH_RATE_CONFIG, STANDARD_CONFIG},
    prover::{generate_proof_of_claim, recursive_single_proof, setup_commitment},
    types::{Cbn128, C, F},
    utils::AmountSecretPairing,
    verifier::{generate_circom_verifier, generate_proof_base64, generate_verifier_config},
};
// use plonky2_field::types::PrimeField64;
use serde::Serialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn init_logger() {
    // let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    use env_logger::Builder;
    // Initialize the logger builder
    Builder::new()
        // Set the minimum log level to display
        .filter_level(log::LevelFilter::Info)
        // Set the format for the log output
        .format_timestamp_secs() // Disable seconds
        .format_timestamp_millis() // Enable milliseconds
        // Initialize the logger
        .init();
}
use log::info;

#[test]
fn test_full_proof() {
    // init_logger();
    let index = 1;
    let distribution = vec![
        AmountSecretPairing { amount: GoldilocksField::ONE, secret: GoldilocksField::ZERO },
        AmountSecretPairing { amount: GoldilocksField::ONE, secret: GoldilocksField::ONE },
        AmountSecretPairing { amount: GoldilocksField::ONE, secret: GoldilocksField::TWO },
        AmountSecretPairing {
            amount: GoldilocksField::ONE,
            secret: GoldilocksField::from_canonical_u64(3),
        },
        AmountSecretPairing {
            amount: GoldilocksField::ONE,
            secret: GoldilocksField::from_canonical_u64(4),
        },
        AmountSecretPairing {
            amount: GoldilocksField::ONE,
            secret: GoldilocksField::from_canonical_u64(5),
        },
        AmountSecretPairing {
            amount: GoldilocksField::ONE,
            secret: GoldilocksField::from_canonical_u64(6),
        },
        AmountSecretPairing {
            amount: GoldilocksField::ONE,
            secret: GoldilocksField::from_canonical_u64(7),
        },
    ];

    let commitment_tree = setup_commitment(distribution.clone());

    let claim_proof = generate_proof_of_claim::<GoldilocksField, C, C, D>(
        distribution.get(index).unwrap().amount,
        distribution.get(index).unwrap().secret,
        index,
        commitment_tree,
        "test.bin",
    )
    .unwrap();

    let (pi, vd, cd) = &claim_proof;

    let layer_final_recursive_proofs = recursive_single_proof::<GoldilocksField, Cbn128, C, D>(
        pi,
        vd,
        cd,
        &HIGH_RATE_CONFIG,
        None,
        true,
        true,
    );
    let (pi, vd, cd) = &layer_final_recursive_proofs.unwrap();

    let conf = generate_verifier_config(&pi).unwrap();
    let (circom_constants, circom_gates) = generate_circom_verifier(&conf, &cd, &vd).unwrap();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let circom_path = PathBuf::from(manifest_dir).parent().unwrap().join("zk-commit-circom");

    let mut circom_file = File::create(circom_path.join("circuits/src/constants.circom")).unwrap();
    circom_file.write_all(circom_constants.as_bytes()).unwrap();
    circom_file = File::create(circom_path.join("circuits/src/gates.circom")).unwrap();
    circom_file.write_all(circom_gates.as_bytes()).unwrap();

    let proof_json = generate_proof_base64(&pi, &conf).unwrap();

    if !circom_path.join("test/data").is_dir() {
        std::fs::create_dir(circom_path.join("test/data")).unwrap();
    }

    let mut proof_file = File::create(circom_path.join("test/data/proof.json")).unwrap();
    proof_file.write_all(proof_json.as_bytes()).unwrap();

    let mut conf_file = File::create(circom_path.join("test/data/conf.json")).unwrap();
    conf_file.write_all(serde_json::to_string(&conf).unwrap().as_ref()).unwrap();

    println!("recursive proof tested successfully");
}
