use actix_web::{post, web, Responder};

use crate::{
    error::ServiceError, models::bodies::WrapGroth16ReqBody, service::biz, AppState,
};

/// register_user
#[post("/groth16")]
pub async fn wrap_groth16(
    body: web::Json<WrapGroth16ReqBody>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    println!("receive wrap request");
    let ret = biz::wrap_groth16(body.0, app_state).await;
    match ret {
        Ok(r) => Ok(web::Json(r)),
        Err(e) => Err(e),
    }
}
