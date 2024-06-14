use crate::{integration::rapidsnark::RapidSnark, AppState};
use actix_web::web::Data;
use zk_commit_core::groth16::Groth16ProofWithPublicData;

use crate::{
    error::ServiceError,
    models::bodies::{WrapGroth16ReqBody, WrapGroth16ResBody},
};

pub(crate) async fn wrap_groth16(
    body: WrapGroth16ReqBody,
    _app_state: Data<AppState>,
) -> Result<WrapGroth16ResBody, ServiceError> {
    println!("receive  wrap_groth16 request: {:?}", body);
    let rapidsnark = RapidSnark::new("http://localhost:3000");
    let rapidsnark_proof = rapidsnark.request_proof_sync(body.plonky2_proof);
    let pwi: Groth16ProofWithPublicData = rapidsnark_proof.into();

    Ok(WrapGroth16ResBody { groth16_proof: pwi })
}
