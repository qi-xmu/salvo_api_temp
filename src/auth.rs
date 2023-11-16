use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::oapi::security::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub(crate) username: String,
    pub(crate) role: String,
    pub(crate) exp: i64,
}

pub(crate) fn get_auth_handle() -> JwtAuth<JwtClaims, ConstDecoder> {
    let auth_handler: JwtAuth<JwtClaims, ConstDecoder> =
        JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
            .finders(vec![Box::new(HeaderFinder::new())])
            .force_passed(true);
    auth_handler
}

pub(crate) fn get_bearer_schema() -> SecurityScheme {
    let bearer_auth = Http::new(HttpAuthScheme::Bearer)
        .bearer_format("JWT")
        .description("Header Authorization");
    SecurityScheme::Http(bearer_auth)
}

pub struct CheckAuthed;
#[async_trait]
impl Handler for CheckAuthed {
    async fn handle(
        &self,
        _req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {}
            JwtAuthState::Unauthorized => {
                res.status_code(StatusCode::UNAUTHORIZED);
                ctrl.skip_rest();
            }
            JwtAuthState::Forbidden => {
                res.status_code(StatusCode::FORBIDDEN);
                ctrl.skip_rest();
            }
        }
    }
}

pub struct CheckAuthedIsAdmin;
#[async_trait]
impl Handler for CheckAuthedIsAdmin {
    async fn handle(
        &self,
        _req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if depot.jwt_auth_state() != JwtAuthState::Authorized {
            res.status_code(StatusCode::UNAUTHORIZED);
            ctrl.skip_rest();
        } else {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            if data.claims.role != "admin" {
                res.status_code(StatusCode::FORBIDDEN);
                ctrl.skip_rest();
            }
        }
    }
}
