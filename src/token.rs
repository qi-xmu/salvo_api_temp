use jsonwebtoken::{self, EncodingKey};
use salvo::{oapi::extract::*, prelude::*};

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::{auth::JwtClaims, error::AppResult};

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Deserialize, Serialize, ToSchema)]
struct TokenResponse {
    token: String,
    expr: u64,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[salvo(schema(example= json!({"username": "root", "password": "pwd"})))]
struct UserToken {
    username: String,
    password: String,
}
#[endpoint(tags("Token"))]
async fn get_token(user_token: FormBody<UserToken>) -> AppResult<Json<TokenResponse>> {
    let (username, password) = (user_token.username.clone(), user_token.password.clone());

    let exp = OffsetDateTime::now_utc() + Duration::days(2);

    if !validate(&username, &password).await {
        return Err(crate::error::AppError::Unauthorized);
    }

    let claim = JwtClaims {
        username,
        role: "admin".to_string(),
        exp: exp.unix_timestamp(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();

    Ok(Json(TokenResponse {
        token,
        expr: exp.unix_timestamp() as u64,
    }))
}

/// 验证用户并返回用户信息
async fn validate(username: &str, password: &str) -> bool {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    username == "root" && password == "pwd"
}

pub fn get_token_route() -> Router {
    let route = Router::new().push(Router::with_path("get_token").post(get_token));
    route
}
