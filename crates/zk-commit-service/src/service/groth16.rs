use std::{fs::File, io::Read};

use crate::{
    integration::{circuits::{circuit_config::HIGH_RATE_CONFIG, recursive::{bn128_proof, get_circuit_data}}, rapidsnark::RapidSnark}, types::{Cbn128, C, D}, AppState
};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::circuit_data::CircuitData};
use zk_commit_core::{groth16::Groth16ProofWithPublicData, prover::MobileProofData};



use crate::error::ServiceError;

pub(crate) fn wrap_groth16(
    proof_file: TempFile,
    _app_state: Data<AppState>,
) -> Result<Groth16ProofWithPublicData, ServiceError> {
    println!("receive  wrap_groth16 request with file");

    let path = format!("./tmp/{}", proof_file.file_name.unwrap());
    log::info!("saving to {path}");
    proof_file.file.persist(path.clone()).unwrap();

    let mut file = File::open(path).expect("Cannot read file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Cannot read file");

    // Deserialize the binary data to a struct
    let mobile_proof_data: MobileProofData = bincode::deserialize(&buffer).unwrap();

    // Wrap the mobile proof in a bn128 plonky proof
    let data = get_circuit_data(mobile_proof_data.merkle_depth);
    let CircuitData { prover_only: _, common, verifier_only } = &data;

    let bn128_proof = bn128_proof::<GoldilocksField, Cbn128, C, D>(
        &mobile_proof_data.proof_with_pis,
        verifier_only,
        common,
        &HIGH_RATE_CONFIG,
        None,
        true,
    ).unwrap();

    let proof_with_pis_json_string = serde_json::to_string(&bn128_proof.0).unwrap();

    let rapidsnark = RapidSnark::new("http://localhost:3000");
    let rapidsnark_proof = rapidsnark.request_proof_sync(proof_with_pis_json_string);
    let pwi: Groth16ProofWithPublicData = rapidsnark_proof.into();

    Ok(pwi)
}
