use salvo::prelude::*;

mod auth;
mod error;
mod token;
mod user;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("/api/v1")
        .hoop(auth::get_auth_handle())
        .push(crate::user::get_user_router())
        .push(crate::token::get_token_route());

    let doc = OpenApi::new("test api", "1.0.0").merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api/v1/api-doc/openapi.json").into_router("swagger-ui"));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let service = Service::new(router);
    Server::new(acceptor).serve(service).await;
}
