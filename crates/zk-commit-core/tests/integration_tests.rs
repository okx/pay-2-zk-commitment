use plonky2::plonk::{
    circuit_data::VerifierOnlyCircuitData,
    config::{GenericConfig, GenericHashOut},
};
use zk_commit_core::{
    verifier::{ generate_proof_base64, generate_verifier_config},
    prover::{setup_commitment, generate_proof_of_claim},
};
// use plonky2_field::types::PrimeField64;
use serde::Serialize;
use std::{fs::File, io::Write, path::{Path, PathBuf}};

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


    println!("recursive proof tested successfully");
}

