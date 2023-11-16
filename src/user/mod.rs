use salvo::oapi::extract::FormBody;
use salvo::prelude::*;
use salvo::rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateLimiter, RemoteIpIssuer};

use crate::error::AppResult;

mod schema;

#[endpoint(
    tags("User"),
    security(("bearer" = []))
)]
async fn create_user(user_info: FormBody<schema::UserInfo>) -> AppResult<Json<schema::UserInfo>> {
    let user_info = user_info.into_inner();
    Ok(Json(user_info))
}

#[endpoint(
    tags("User"),
    security(("bearer" = []))
)]
async fn get_user() -> &'static str {
    "Get User"
}

#[endpoint(
    tags("User"),
    security(("bearer" = []))
)]
async fn update_user() -> &'static str {
    "Update User"
}

#[endpoint(
    tags("User"),
    security(("bearer" = []))
)]
async fn delete_user() -> &'static str {
    "Delete User"
}

pub fn get_user_router() -> Router {
    let limiter = RateLimiter::new(
        FixedGuard::new(),
        MokaStore::new(),
        RemoteIpIssuer,
        BasicQuota::per_second(1),
    );

    let check_authed = crate::auth::CheckAuthed;

    let router = Router::with_path("user")
        .hoop(limiter)
        .hoop(check_authed)
        .push(
            Router::with_path("create_user")
                .hoop(crate::auth::CheckAuthedIsAdmin)
                .post(create_user),
        )
        .push(Router::with_path("get_user").get(get_user))
        .push(
            Router::with_path("update_user")
                .hoop(crate::auth::CheckAuthedIsAdmin)
                .post(update_user),
        )
        .push(
            Router::with_path("delete_user")
                .hoop(crate::auth::CheckAuthedIsAdmin)
                .delete(delete_user),
        );
    router
}
