use actix_web::web::Data;
use crate::integration::rapidsnark::{Groth16ProofWithPublicData, RapidSnark};
use crate::AppState;

use crate::{

    error::ServiceError,
    models::bodies::{
        WrapGroth16ReqBody,
        WrapGroth16ResBody
    },
};

pub(crate) async fn wrap_groth16(
    body: WrapGroth16ReqBody,
    app_state: Data<AppState>,
) -> Result<WrapGroth16ResBody, ServiceError> {
    println!("receive  wrap_groth16 request: {:?}", body);
    let rapidsnark = RapidSnark::new("http://localhost:3000");
    let rapidsnark_proof = rapidsnark.request_proof_sync(body.plonky2_proof);
    let pwi: Groth16ProofWithPublicData = rapidsnark_proof.into();

    Ok(WrapGroth16ResBody { groth16_proof: pwi })
}
