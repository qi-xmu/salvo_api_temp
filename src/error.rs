use salvo::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("username or password is wrong")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("internal server error")]
    InternalServerError,
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        // res.render(salvo::writing::Text::Plain("I'm a error, hahaha!"));
        res.render(Json(serde_json::json!({ "error": self.to_string() })));
        // res.status_code(match self {
        //     AppError::Unauthorized => salvo::http::StatusCode::UNAUTHORIZED,
        //     AppError::NotFound => salvo::http::StatusCode::NOT_FOUND,
        //     AppError::BadRequest => salvo::http::StatusCode::BAD_REQUEST,
        //     AppError::InternalServerError => salvo::http::StatusCode::INTERNAL_SERVER_ERROR,
        // });
    }
}

use salvo::http::{StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};

impl EndpointOutRegister for AppError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Internal server error")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            oapi::Response::new("Not found")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::BAD_REQUEST.as_str(),
            oapi::Response::new("Bad request")
                .add_content("application/json", StatusError::to_schema(components)),
        );
    }
}
