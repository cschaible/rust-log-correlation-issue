use axum::{
    body::Full,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{debug, instrument};

#[instrument(name = "api.greet", skip_all)]
pub async fn greet() -> impl IntoResponse {
    debug!("Greet user");
    Response::builder()
        .status(StatusCode::OK)
        .body(Full::from("hello world".to_string()))
        .unwrap()
}
