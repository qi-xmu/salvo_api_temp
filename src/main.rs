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

    // 扩展安全组件
    let components =
        salvo::oapi::Components::default().add_security_scheme("bearer", auth::get_bearer_schema());

    let doc = OpenApi::new("test api", "1.0.0")
        .components(components)
        .merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api/v1/api-doc/openapi.json").into_router("swagger-ui"));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let service =
        Service::new(router).catcher(salvo::catcher::Catcher::default().hoop(handle_error));
    Server::new(acceptor).serve(service).await;
}

#[handler]
async fn handle_error(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let status_code = res.status_code.unwrap();
    let code = status_code.as_u16();
    res.render(Json(
        serde_json::json!({ "code" : code , "error": status_code.to_string() }),
    ));
}
